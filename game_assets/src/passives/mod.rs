use game_core7::{
    TimeNum,
    passive::{PassiveTrait, PassiveUpdateInfo, passive_box::PassiveBox},
};

use crate::FPS;

#[derive(Debug, Clone)]
pub struct Warcry {
    pub time: u64,
}

impl PassiveTrait for Warcry {
    fn display(&self) -> String {
        format!("ウォークライ: {}s", self.time / FPS)
    }

    fn frame(
        &self,
        owner: game_core7::runtime_id::LtId,
        state: &game_core7::state::GameState,
        ctx: &mut game_core7::core_actor::CtxContainer,
    ) {
    }

    fn clone_box(&self) -> PassiveBox {
        PassiveBox::new(self.clone())
    }

    fn info(&self) -> &game_core7::passive::PassiveInfo {
        todo!()
    }

    fn merge(&mut self, passive: &dyn std::any::Any) -> game_core7::passive::PassiveUpdateInfo {
        self.time += passive.downcast_ref::<Self>().unwrap().time;
        PassiveUpdateInfo {
            need_update_cached_status: false,
        }
    }

    fn should_trash(&self) -> bool {
        self.time == 0
    }

    fn static_id(&self) -> game_core7::StaticPassiveId {
        0
    }

    fn status(&self, s: &mut game_core7::passive::status::PassiveStatus) {
        s.physics_attuck_mag_buff.add(0.1);
    }

    fn tick(&mut self) -> PassiveUpdateInfo {
        self.time -= 1;

        PassiveUpdateInfo {
            need_update_cached_status: false,
        }
    }
}
