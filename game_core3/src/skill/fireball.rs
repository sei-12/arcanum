use crate::{
    damage::Damage,
    event::Event,
    event_accepter::{EventAccepter, WinOrLoseOrNextwave},
    passive::public_passive::burn::Burn,
    skill::{SkillResult, SkillTrait, SkillTraitPrivate},
    state::chars::RuntimeCharId,
};

#[derive(Debug, Clone)]
pub struct Fireball;

impl Fireball {
    pub(super) fn new() -> Self {
        Self
    }
}

impl SkillTrait for Fireball {
    fn id(&self) -> super::StaticSkillId {
        super::StaticSkillId::Fireball
    }

    fn document(&self) -> &'static super::SkillDocument {
        &super::SkillDocument {
            need_mp: 80,
            hate: 80,
            cooldown: 80,
            text: "敵1体に倍率1.1の魔法ダメージを与える。
        INTが16以上なら倍率が+0.2される。
        DEXが5以下なら自身に3ターンの火傷を与える。
        AGIが17以上なら消費MPが20減少、CDが20減少、倍率が0.2減少。",
            name: "ファイヤーボール",
        }
    }

    fn need_mp(
        &self,
        user: &crate::buttle_char::ButtleChar,
        _state: &crate::state::GameState,
    ) -> crate::MpNum {
        if user.lt().agi() >= 17.0 {
            self.document().need_mp - 20
        } else {
            self.document().need_mp
        }
    }

    // fn call(
    //     &self,
    //     user: &crate::buttle_char::ButtleChar,
    //     state: &crate::state::GameState,
    //     events: &mut impl crate::event::EventsQuePusher,
    // ) -> super::SkillResult {
    //     let mut dmg_mag = 1.1;
    //     let burn_flag = user.lt().dex() <= 5.0;
    //     let mut cooldown = self.document().cooldown;

    //     if user.lt().agi() >= 17.0 {
    //         cooldown -= 20;
    //         dmg_mag -= 0.2;
    //     }

    //     if user.lt().int() >= 16.0 {
    //         dmg_mag += 0.2;
    //     }

    //     let target = state.get_enemy_with_highest_target_priority();

    //     let dmg = Event::Damage(Damage::new_magic_damage(
    //         state,
    //         user.lt_id(),
    //         crate::state::LtId::Enemy(target.runtime_id()),
    //         dmg_mag,
    //     ));

    //     events.push(dmg);

    //     if burn_flag {
    //         let add_passive_event = Event::AddPassive {
    //             target_id: user.lt_id(),
    //             passive: Box::new(Burn::new(3)),
    //         };

    //         events.push(add_passive_event);
    //     }

    //     SkillResult {
    //         consume_mp: self.need_mp(user, state),
    //         hate: self.document().hate,
    //         cooldown,
    //     }
    // }

    // fn call_new(&self) -> super::SkillFlow { call }
}

impl SkillTraitPrivate for Fireball {
    fn get_skill_fn(&self) -> super::SkillFlow {
        call
    }
}

fn call(
    accepter: &mut EventAccepter,
    user: RuntimeCharId,
) -> Result<SkillResult, WinOrLoseOrNextwave> {
    todo!()
}
