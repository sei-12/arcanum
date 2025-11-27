use crate::StatusNum;

#[derive(Debug, Clone)]
pub struct Potential {
    int: StatusNum,
    vit: StatusNum,
    str: StatusNum,
    dex: StatusNum,
    agi: StatusNum,
}

impl Potential {
    pub(crate) const fn new(
        int: StatusNum,
        vit: StatusNum,
        str: StatusNum,
        dex: StatusNum,
        agi: StatusNum,
    ) -> Self {
        let sum = agi + dex + int + str + vit;

        if sum != 50.0 {
            panic!("ポテンシャルの合計値はちょうど50である必要がある")
        };

        Self {
            int,
            vit,
            str,
            dex,
            agi,
        }
    }

    pub fn int(&self) -> StatusNum {
        self.int
    }

    pub fn vit(&self) -> StatusNum {
        self.vit
    }

    pub fn str(&self) -> StatusNum {
        self.str
    }

    pub fn dex(&self) -> StatusNum {
        self.dex
    }

    pub fn agi(&self) -> StatusNum {
        self.agi
    }
}
