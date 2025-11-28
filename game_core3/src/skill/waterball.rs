// use crate::skill::SkillTrait;

// #[derive(Debug, Clone)]
// pub struct Waterball;
// impl Waterball {
//     pub(super) fn new() -> Self {
//         Self
//     }
// }

// impl SkillTrait for Waterball {
//     fn id(&self) -> super::StaticSkillId {
//         super::StaticSkillId::Waterball
//     }
//     fn name(&self) -> &'static str {
//         "ファイヤーボール"
//     }

//     fn text(&self) -> &'static str {
//         "敵に倍率1.1の魔法ダメージを与える。
//         INTが16以上なら倍率が+0.2される。
//         DEXが5以下なら自身に3ターンの火傷を与える。
//         AGIが17以上なら消費MPが20減少、CDが20減少、倍率が0.2減少。"
//     }

//     fn call(
//         &self,
//         user: &crate::buttle_char::ButtleChar,
//         state: &crate::state::GameState,
//         events: &mut impl crate::event::EventsQuePusher,
//     ) -> super::SkillResult {
//         todo!()
//     }

//     fn defalut_need_mp(&self) -> crate::MpNum {
//         80
//     }
// }
