use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use downcast_rs::{Downcast, impl_downcast};
use smallbox::{SmallBox, smallbox, space::S4};

#[derive(Debug)]
pub struct MessageBox {
    inner: SmallBox<dyn AnyMessage, S4>,
}

impl MessageBox {
    pub fn new(msg: impl AnyMessage + 'static) -> Self {
        Self {
            inner: smallbox!(msg),
        }
    }
}
impl Deref for MessageBox {
    type Target = dyn AnyMessage;
    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl DerefMut for MessageBox {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.deref_mut()
    }
}

impl Clone for MessageBox {
    fn clone(&self) -> Self {
        self.inner.clone_box()
    }
}

pub trait AnyMessage: Downcast + Debug {
    fn clone_box(&self) -> MessageBox;
}

impl<T: Clone + Debug + 'static> AnyMessage for T {
    fn clone_box(&self) -> MessageBox {
        MessageBox::new(self.clone())
    }
}

impl_downcast!(AnyMessage);
