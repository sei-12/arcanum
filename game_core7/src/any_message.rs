use std::{any::Any, sync::Arc};

#[derive(Debug, Clone)]
pub enum AnyMessage {
    MagicNum(u64),
    Str(&'static str),
    Buffer([u8; 16]),
    ArcAny(Arc<dyn Any>),
}

impl AnyMessage {
    pub fn is_num_and(&self, n: u64) -> bool {
        match self {
            AnyMessage::MagicNum(magic_number) => *magic_number == n,
            _ => false,
        }
    }
    /// 'static str が与えられた条件と一致するかどうか
    pub fn is_str_and(&self, condition: &str) -> bool {
        match self {
            AnyMessage::Str(s) => *s == condition,
            _ => false,
        }
    }

    /// Arc<dyn Any> から T への参照のダウンキャスト
    pub fn try_downcast_ref<T: Any + Send + Sync + 'static>(&self) -> Option<&T> {
        match self {
            AnyMessage::ArcAny(arc_any) => arc_any.downcast_ref::<T>(),
            _ => None,
        }
    }

    /// Buffer から T への変換を試みる
    pub fn from_buffer<'a, T: TryFrom<&'a [u8; 16]>>(&'a self) -> Option<T> {
        match self {
            AnyMessage::Buffer(buf) => T::try_from(buf).ok(),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_is_num_and() {
        let msg = AnyMessage::MagicNum(42);
        assert!(msg.is_num_and(42));
        assert!(!msg.is_num_and(100));
    }

    #[test]
    fn test_is_str_and() {
        let msg = AnyMessage::Str("hello");
        assert!(msg.is_str_and("hello"));
        assert!(!msg.is_str_and("world"));
    }

    #[test]
    fn test_try_downcast_ref_success() {
        let arc = Arc::new(100u32) as Arc<dyn Any>;
        let msg = AnyMessage::ArcAny(arc);

        let value: Option<&u32> = msg.try_downcast_ref::<u32>();
        assert_eq!(value.copied(), Some(100));
    }

    #[test]
    fn test_try_downcast_ref_fail() {
        let arc = Arc::new("text") as Arc<dyn Any>;
        let msg = AnyMessage::ArcAny(arc);

        let value: Option<&u32> = msg.try_downcast_ref::<u32>();
        assert!(value.is_none());
    }

    #[test]
    fn test_from_buffer_success() {
        #[derive(Debug, PartialEq)]
        struct MyType(u128);

        impl TryFrom<&[u8; 16]> for MyType {
            type Error = ();

            fn try_from(value: &[u8; 16]) -> Result<Self, Self::Error> {
                Ok(MyType(u128::from_be_bytes(*value)))
            }
        }

        let buffer = [0u8; 16];
        let msg = AnyMessage::Buffer(buffer);

        let result = msg.from_buffer::<MyType>();
        assert_eq!(result, Some(MyType(0)));
    }

    #[test]
    fn test_from_buffer_fail_wrong_variant() {
        let msg = AnyMessage::MagicNum(5);

        #[derive(Debug)]
        struct Dummy;

        impl TryFrom<&[u8; 16]> for Dummy {
            type Error = ();

            fn try_from(_: &[u8; 16]) -> Result<Self, Self::Error> {
                Ok(Dummy)
            }
        }

        let result = msg.from_buffer::<Dummy>();
        assert!(result.is_none());
    }
}
