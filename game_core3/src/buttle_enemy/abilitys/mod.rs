use std::any::Any;

use crate::passive::{RuntimePassiveId, gen_passive_runtime_id, traits::Passive};

pub enum EnemyAbility {
    // 変質
    Transition,
    // 逆襲
    Revenge,
}

pub(crate) fn make_enemy_ability(ability: EnemyAbility) -> Box<dyn Passive> {
    todo!()
}

//--------------------------------------------------//
//                                                  //
//                     REVENGE                      //
//                                                  //
//--------------------------------------------------//

// #[derive(Debug, Clone)]
// struct Revenge {
//     id: RuntimePassiveId,
// }
// impl Revenge {
//     fn new() -> Self {
//         Self {
//             id: gen_passive_runtime_id(),
//         }
//     }
// }

// impl Passive for Revenge {
//     fn display(&'_ self) -> Option<crate::passive::DisplayPassiveInfo<'_>> {
//         Some(crate::passive::DisplayPassiveInfo {
//             header: "".into(),
//             text: "".into(),
//         })
//     }

//     fn merge(&mut self, passive: Box<dyn Passive>) {
//         assert_eq!(self.static_id(), passive.static_id());
//     }

//     fn runtime_id(&self) -> crate::passive::RuntimePassiveId {
//         self.id
//     }
//     fn should_merge_type(&self) -> bool {
//         false
//     }
//     fn merge_state(&self) -> Option<Box<dyn Any>> {
//         None
//     }
//     fn should_trash(&self) -> bool {
//         false
//     }
//     fn static_id(&self) -> std::any::TypeId {
//         self.type_id()
//     }
//     fn status(&self, status: &mut crate::passive::status::PassiveStatus) {
//         todo!()
//     }
//     fn update_state(
//         &mut self,
//         msg: &crate::passive::PassiveUpdateStateMessage,
//     ) -> Result<(), crate::passive::PassiveUpdateStateError> {
//         Ok(())
//     }
// }

//--------------------------------------------------//
//                                                  //
//                    TRANSITION                    //
//                                                  //
//--------------------------------------------------//
