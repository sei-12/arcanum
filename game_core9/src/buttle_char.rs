use crate::lt_common::LtCommon;

pub struct ButtleChar {
    lt_common: LtCommon,
}

impl ButtleChar {
    pub fn new() -> Result<Self, crate::Error> {
        todo!()
    }
    
    pub fn lt(&self) -> &LtCommon {
        &self.lt_common
    } 
    
    pub(crate) fn lt_mut(&mut self) -> &mut LtCommon {
        &mut self.lt_common
    }
}

