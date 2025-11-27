use crate::{MpNum, StatusNum};

#[derive(Debug, Clone)]
pub struct PassiveStatus {
    /// 魔法攻撃力に乗算させる値 default = 1.0
    ///
    /// 乗算するな
    pub magic_attuck_mag: StatusNum,

    /// 物理攻撃力に乗算させる値 default = 1.0
    ///
    /// 乗算するな
    pub physics_attuck_mag: StatusNum,

    /// 最大HPに乗算させる値
    ///
    /// default = 1.0
    pub max_hp_mag: StatusNum,

    /// 被魔法ダメージ倍率
    ///
    /// defalut = 1.0
    /// ### Rule
    /// 加算、減算はするな
    ///
    /// todo: ドキュメントで禁止するだけじゃなくて何らかのRustの機能を使って禁止にしたい
    pub recv_magic_dmg_mag: StatusNum,

    /// 被物理ダメージ倍率
    ///
    /// つまり大きいほど防御力が低いことになる.
    ///
    /// defalut = 1.0
    /// ### Rule
    /// 加算、減算はするな
    ///
    /// todo: ドキュメントで禁止するだけじゃなくて何らかのRustの機能を使って禁止にしたい
    pub recv_physics_dmg_mag: StatusNum,

    /// 追加MP回復
    ///
    /// ターン開始時に追加で回復されるMP量
    ///
    /// チームの合計が負の値になったとしてもMPが減ることはない
    pub add_heal_mp: MpNum,

    pub add_agi: StatusNum,
}

impl Default for PassiveStatus {
    fn default() -> Self {
        Self {
            magic_attuck_mag: 1.0,
            physics_attuck_mag: 1.0,
            max_hp_mag: 1.0,
            recv_magic_dmg_mag: 1.0,
            recv_physics_dmg_mag: 1.0,
            add_heal_mp: 0,
            add_agi: 0.0,
        }
    }
}
impl PassiveStatus {
    pub(crate) fn reset(&mut self) {
        *self = Self::default();
    }
}
