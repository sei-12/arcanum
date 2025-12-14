#[allow(unused)]
pub(crate) mod arc_with_accessor {
    use std::{ops::Deref, sync::Arc};

    /// `Arc<T>` を内部に保持しつつ、
    /// 任意の `Accessor` を用いて `T` の一部への参照を提供する構造体。
    ///
    /// この型は `Deref` を実装しており、
    /// `Accessor::Target` への参照として透過的に利用できる。
    ///
    /// # 特徴
    ///
    /// - `Arc<T>` のクローンは安価
    /// - アクセサを差し替えた「ビュー」を簡単に生成可能
    /// - 参照は常に `&T` から派生するため安全
    #[derive(Debug, Clone)]
    pub struct ArcWithAccessor<T, A: Accessor<T>> {
        accessor: A,
        data: Arc<T>,
    }

    impl<T, A: Accessor<T>> Deref for ArcWithAccessor<T, A> {
        type Target = A::Target;

        /// 内部の `Accessor` を用いて `Target` への参照を取得する。
        fn deref(&self) -> &Self::Target {
            self.get()
        }
    }

    impl<T, A: Accessor<T>> ArcWithAccessor<T, A> {
        /// 同じ `Arc<T>` を共有しつつ、
        /// 別の `Accessor` を持つ `ArcWithAccessor` を生成する。
        pub fn clone_with_accessor<B: Accessor<T>>(&self, accessor: B) -> ArcWithAccessor<T, B> {
            ArcWithAccessor {
                accessor,
                data: self.data.clone(),
            }
        }

        /// `Accessor` を通して `T` の一部への参照を明示的に取得する。
        pub fn get(&self) -> &A::Target {
            self.accessor.access(&self.data)
        }

        /// 内部で保持している `Arc<T>` を取り出す。
        ///
        /// この操作により `ArcWithAccessor` は消費される。
        pub fn take_arc(self) -> Arc<T> {
            self.data
        }
    }

    /// `T` への参照から、その一部 (`Target`) への参照を取得するためのトレイト。
    ///
    /// フィールドアクセス、計算済みビュー、
    /// あるいは構造体の一部を指す参照などに利用できる。
    pub trait Accessor<T> {
        /// アクセス対象の型
        type Target;

        /// `T` から `Target` への参照を取得する
        fn access<'a>(&self, t: &'a T) -> &'a Self::Target;
    }

    /// `Arc<T>` から `ArcWithAccessor` を生成するための補助トレイト。
    pub trait WithAccessor<T> {
        /// 指定した `Accessor` を用いたビューを作成する。
        fn with_accessor<A: Accessor<T>>(&self, accessor: A) -> ArcWithAccessor<T, A>;
    }

    impl<T> WithAccessor<T> for Arc<T> {
        fn with_accessor<A: Accessor<T>>(&self, accessor: A) -> ArcWithAccessor<T, A> {
            ArcWithAccessor {
                accessor,
                data: self.clone(),
            }
        }
    }

    impl<T, F, U> Accessor<T> for F
    where
        F: Fn(&T) -> &U,
    {
        type Target = U;
        fn access<'a>(&self, t: &'a T) -> &'a Self::Target {
            self(t)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::sync::Arc;

        #[derive(Debug)]
        struct Data {
            a: i32,
            b: String,
        }

        struct AccessA;
        impl Accessor<Data> for AccessA {
            type Target = i32;
            fn access<'a>(&self, t: &'a Data) -> &'a Self::Target {
                &t.a
            }
        }

        struct AccessB;
        impl Accessor<Data> for AccessB {
            type Target = String;
            fn access<'a>(&self, t: &'a Data) -> &'a Self::Target {
                &t.b
            }
        }

        #[test]
        fn basic_access() {
            let data = Arc::new(Data {
                a: 10,
                b: "hello".to_string(),
            });

            let view = data.with_accessor(AccessA);
            assert_eq!(*view, 10);
            assert_eq!(*view.get(), 10);
        }

        #[test]
        fn clone_with_different_accessor() {
            let data = Arc::new(Data {
                a: 42,
                b: "world".to_string(),
            });

            let a_view = data.with_accessor(AccessA);
            let b_view = a_view.clone_with_accessor(AccessB);

            assert_eq!(*a_view, 42);
            assert_eq!(b_view.as_str(), "world");
        }

        #[test]
        fn take_arc_returns_original_arc() {
            let data = Arc::new(Data {
                a: 1,
                b: "x".to_string(),
            });

            let view = data.with_accessor(AccessA);
            let arc = view.take_arc();

            assert_eq!(arc.a, 1);
        }
    }
}
