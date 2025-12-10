use crate::StatusNum;

// メリット効果は加算 デメリット効果は乗算
//
// メリット効果を乗算にすると倍々ですぐにとんでもない値になりそう。
// デメリット効果を減算にすると負の値になったりして扱いにくい。
//
// 例外としてPotentialに関するデメリット効果は減算で良い
// その代わりLtCommon側で0以下の処理などを加える必要はある

#[derive(Debug, Clone)]
pub struct PassiveStatus {
    /// 魔法攻撃力に乗算させる値 default = 1.0
    pub magic_attuck_mag_buff: BuffMagnificationNum,
    /// 魔法攻撃力に乗算させる値 default = 1.0
    pub magic_attuck_mag_debuff: DebuffMagnificationNum,

    /// 物理攻撃力に乗算させる値
    pub physics_attuck_mag_buff: BuffMagnificationNum,
    /// 物理攻撃力に乗算させる値
    pub physics_attuck_mag_debuff: DebuffMagnificationNum,

    /// 最大HPに乗算させる値
    pub max_hp_mag_buff: BuffMagnificationNum,
    /// 最大HPに乗算させる値
    pub max_hp_mag_debuff: DebuffMagnificationNum,

    /// 被魔法ダメージ倍率
    pub recv_magic_dmg_mag: RecvDamageMagnificationNum,

    /// 被物理ダメージ倍率
    pub recv_physics_dmg_mag: RecvDamageMagnificationNum,

    pub add_agi: StatusNum,
    pub add_str: StatusNum,
    pub add_vit: StatusNum,
    pub add_dex: StatusNum,
    pub add_int: StatusNum,
}

impl Default for PassiveStatus {
    fn default() -> Self {
        Self {
            max_hp_mag_buff: BuffMagnificationNum::default(),
            max_hp_mag_debuff: DebuffMagnificationNum::default(),
            recv_magic_dmg_mag: RecvDamageMagnificationNum::default(),
            recv_physics_dmg_mag: RecvDamageMagnificationNum::default(),
            add_agi: 0.0,
            add_str: 0.0,
            add_dex: 0.0,
            add_int: 0.0,
            add_vit: 0.0,
            magic_attuck_mag_buff: BuffMagnificationNum::default(),
            magic_attuck_mag_debuff: DebuffMagnificationNum::default(),
            physics_attuck_mag_buff: BuffMagnificationNum::default(),
            physics_attuck_mag_debuff: DebuffMagnificationNum::default(),
        }
    }
}

impl PassiveStatus {
    pub(crate) fn reset(&mut self) {
        *self = Self::default();
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RecvDamageMagnificationNum {
    value: StatusNum,
}
impl Default for RecvDamageMagnificationNum {
    fn default() -> Self {
        Self { value: 1.0 }
    }
}
impl RecvDamageMagnificationNum {
    pub fn get(&self) -> StatusNum {
        self.value
    }
    pub fn mul(&mut self, val: StatusNum) {
        assert!(val >= 0.0);
        self.value *= val;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BuffMagnificationNum {
    value: StatusNum,
}
impl Default for BuffMagnificationNum {
    fn default() -> Self {
        Self { value: 1.0 }
    }
}
impl BuffMagnificationNum {
    /// 0以上が保証されている
    pub fn get(&self) -> StatusNum {
        assert!(self.value >= 0.0);
        self.value
    }
    pub fn add(&mut self, buff: StatusNum) {
        assert!(buff >= 0.0);
        self.value += buff;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DebuffMagnificationNum {
    value: StatusNum,
}
impl Default for DebuffMagnificationNum {
    fn default() -> Self {
        Self { value: 1.0 }
    }
}
impl DebuffMagnificationNum {
    /// 0以上が保証されている
    pub fn get(&self) -> StatusNum {
        assert!(self.value >= 0.0);
        self.value
    }
    pub fn mul(&mut self, buff: StatusNum) {
        assert!(buff <= 1.0);
        self.value *= buff;
    }
}
