use crate::MpNum;

#[derive(Debug, Clone, Default)]
pub(super) struct Mp {
    value: MpNum,
}

impl Mp {
    pub fn consume(&mut self, num: MpNum) {
        if num > self.value {
            self.value = 0
        } else {
            self.value -= num
        }
    }

    pub fn heal(&mut self, num: MpNum) {
        self.value += num;
    }

    pub fn get(&self) -> MpNum {
        self.value
    }
}
