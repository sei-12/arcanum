use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::passive::PassiveTrait;

#[derive(Debug)]
pub struct PassiveBox(Box<dyn PassiveTrait>);
impl PassiveBox {
    pub fn new(inner: impl PassiveTrait + 'static) -> Self {
        Self(Box::new(inner))
    }
}
impl Deref for PassiveBox {
    type Target = dyn PassiveTrait;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl DerefMut for PassiveBox {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.0
    }
}

impl Clone for PassiveBox {
    fn clone(&self) -> Self {
        self.0.clone_box()
    }
}
