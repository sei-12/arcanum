use std::marker::PhantomData;

use crate::{Frame, UpdateStateMessage};

pub(super) struct Effects<'a> {
    inner: EffectsInner<'a, UpdateStateMessage, Frame>,
}

impl<'a> Iterator for Effects<'a> {
    type Item = &'a UpdateStateMessage;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a> Effects<'a> {
    pub fn new(frames: &'a [Frame]) -> Self {
        Self {
            inner: EffectsInner::new(frames),
        }
    }
    pub fn has_remaining(&self) -> bool {
        self.inner.has_remaining()
    }
}

trait FrameInterface<T: 'static> {
    fn main_effect(&self) -> &T;
    fn sub_effects(&self) -> &[T];
}

impl FrameInterface<UpdateStateMessage> for Frame {
    fn main_effect(&self) -> &UpdateStateMessage {
        &self.main_effect
    }
    fn sub_effects(&self) -> &[UpdateStateMessage] {
        &self.sub_effects
    }
}

//--------------------------------------------------//
//                                                  //
//                      INNER                       //
//                                                  //
//--------------------------------------------------//

struct EffectsInner<'a, T: 'static, F: FrameInterface<T>> {
    __t_marker: PhantomData<T>,

    frames: &'a [F],
    // 次にnextで取得する値
    frame_cursor: usize,
    // 次にnextで取得する値
    sub_effect_cursor: usize,

    next_is_main_effect: bool,
}

impl<'a, T, F: FrameInterface<T>> EffectsInner<'a, T, F> {
    pub(super) fn new(frames: &'a [F]) -> Self {
        EffectsInner {
            frames,
            frame_cursor: 0,
            sub_effect_cursor: 0,
            next_is_main_effect: true,
            __t_marker: PhantomData,
        }
    }

    pub(super) fn has_remaining(&self) -> bool {
        self.frame_cursor < self.frames.len()
    }
}

impl<'a, T, F: FrameInterface<T>> Iterator for EffectsInner<'a, T, F> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.has_remaining() {
            return None;
        }

        let item = if self.next_is_main_effect {
            let item = self.frames[self.frame_cursor].main_effect();
            self.next_is_main_effect = false;
            item
        } else {
            let item = &self.frames[self.frame_cursor].sub_effects()[self.sub_effect_cursor];
            self.sub_effect_cursor += 1;
            item
        };

        let sub_effects = self.frames[self.frame_cursor].sub_effects();

        if sub_effects.is_empty() || self.sub_effect_cursor == sub_effects.len() {
            self.frame_cursor += 1;
            self.next_is_main_effect = true;
            self.sub_effect_cursor = 0;
        };

        Some(item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// テスト用の簡易フレーム
    struct TestFrame {
        main: u8,
        subs: Vec<u8>,
    }

    impl FrameInterface<u8> for TestFrame {
        fn main_effect(&self) -> &u8 {
            &self.main
        }
        fn sub_effects(&self) -> &[u8] {
            &self.subs
        }
    }

    #[test]
    fn test_effects_inner_iteration() {
        // frame1: main=1, subs=[11, 12]
        // frame2: main=2, subs=[21]
        let frames = vec![
            TestFrame {
                main: 1,
                subs: vec![11, 12],
            },
            TestFrame {
                main: 2,
                subs: vec![21],
            },
        ];

        let iter = EffectsInner::<u8, TestFrame>::new(&frames);

        let collected: Vec<u8> = iter.cloned().collect();

        assert_eq!(
            collected,
            vec![
                1, 11, 12, // frame1
                2, 21, // frame2
            ]
        );
    }

    #[test]
    fn test_has_remaining() {
        let frames = vec![TestFrame {
            main: 1,
            subs: vec![11],
        }];

        let mut iter = EffectsInner::<u8, TestFrame>::new(&frames);

        assert!(iter.has_remaining());
        iter.next(); // main
        assert!(iter.has_remaining());
        iter.next(); // sub
        assert!(!iter.has_remaining());
    }

    #[test]
    fn test_has_remaining2() {
        let frames = vec![];
        let iter = EffectsInner::<u8, TestFrame>::new(&frames);
        assert!(!iter.has_remaining());
    }

    #[test]
    fn test_effects_inner_iteration2() {
        let frames = vec![
            TestFrame {
                main: 1,
                subs: vec![],
            },
            TestFrame {
                main: 2,
                subs: vec![],
            },
            TestFrame {
                main: 3,
                subs: vec![4, 5, 6],
            },
            TestFrame {
                main: 7,
                subs: vec![8, 9, 10],
            },
        ];

        let iter = EffectsInner::<u8, TestFrame>::new(&frames);

        let collected: Vec<u8> = iter.cloned().collect();

        assert_eq!(collected, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
