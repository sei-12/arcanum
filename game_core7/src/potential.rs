use crate::StatusNum;

//--------------------------------------------------//
//                                                  //
//                    POTENTIAL                     //
//                                                  //
//--------------------------------------------------//
#[derive(Debug, Clone)]
pub struct Potential {
    int: StatusNum,
    vit: StatusNum,
    str: StatusNum,
    dex: StatusNum,
    agi: StatusNum,
}

impl Potential {
    pub const fn new(
        agi: StatusNum,
        dex: StatusNum,
        int: StatusNum,
        str: StatusNum,
        vit: StatusNum,
    ) -> Self {
        let sum = agi + dex + int + str + vit;

        if int <= 0.0 {
            panic!("ポテンシャルの値は0より大きい値である必要がある")
        }

        if vit <= 0.0 {
            panic!("ポテンシャルの値は0より大きい値である必要がある")
        }

        if str <= 0.0 {
            panic!("ポテンシャルの値は0より大きい値である必要がある")
        }

        if dex <= 0.0 {
            panic!("ポテンシャルの値は0より大きい値である必要がある")
        }

        if agi <= 0.0 {
            panic!("ポテンシャルの値は0より大きい値である必要がある")
        }

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

    /// 0より大きいことが保証されている
    pub fn int(&self) -> StatusNum {
        self.int
    }

    /// 0より大きいことが保証されている
    pub fn vit(&self) -> StatusNum {
        self.vit
    }

    /// 0より大きいことが保証されている
    pub fn str(&self) -> StatusNum {
        self.str
    }

    /// 0より大きいことが保証されている
    pub fn dex(&self) -> StatusNum {
        self.dex
    }

    /// 0より大きいことが保証されている
    pub fn agi(&self) -> StatusNum {
        self.agi
    }
}
