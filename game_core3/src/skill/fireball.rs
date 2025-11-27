use crate::skill::{SkillTrait, fireball};

#[derive(Debug, Clone)]
pub struct Fireball;

impl Fireball {
    pub(super) fn new() -> Self {
        Self
    }
}

impl SkillTrait for Fireball {
    fn name(&self) -> &'static str {
        "ファイヤーボール"
    }

    fn text(&self) -> &'static str {
        "敵に倍率1.1の魔法ダメージを与える。
        INTが16以上なら倍率が+0.2される。
        DEXが5以下なら自身に3ターンの火傷を与える。
        AGIが17以上なら消費MPが20減少、CDが20減少、倍率が0.2減少。"
    }

    fn call(&self, state: &crate::state::GameState, events: &mut impl super::EventsQue) {}
}
