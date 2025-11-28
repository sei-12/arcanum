// pub mod isinage;

use crate::SpNum;

pub struct EnemySkillDocument {
    pub name: &'static str,
    pub text: &'static str,
    pub need_sp: SpNum,
}

pub mod hikkaku {
    use crate::{
        buttle_enemy::skill::EnemySkillDocument, damage::Damage,
        passive::public_passive::bleeding::Bleeding,
    };
    pub const DOCUMENT: EnemySkillDocument = EnemySkillDocument {
        need_sp: 80,
        name: "ひっかく",
        text: "最もヘイト値の高いキャラクター1体に対して倍率0.3の物理ダメージを3回与える。
        STRが16以上なら倍率が+0.1される。
        AGIが15以上ならダメージを4回与える。
        DEXが17以上なら対象に対して3ターンの「出血」を与える。",
    };

    pub fn call(
        enemy_id: crate::enemys::RuntimeEnemyId,
        state: &crate::state::GameState,
        events: &mut impl crate::event::EventsQuePusher,
    ) {
        let target = state.chars().get_highest_hate_char();
        let user = state.enemys().get(enemy_id);
        let mut dmg_mag = 0.5;
        if user.lt().str() >= 16.0 {
            dmg_mag += 0.2;
        }
        let mut damage_count = 3;
        if user.lt().agi() >= 15.0 {
            damage_count += 1;
        }

        for _ in 0..damage_count {
            events.push(crate::event::Event::Damage(Damage::new_physics_damage(
                state,
                target.lt_id(),
                target.lt_id(),
                dmg_mag,
            )));
        }

        if user.lt().dex() >= 17.0 {
            events.push(crate::event::Event::AddPassive {
                target_id: target.lt_id(),
                passive: Box::new(Bleeding::new(3)),
            });
        }
    }
}

pub mod isinage {
    use crate::buttle_enemy::skill::EnemySkillDocument;
    pub const DOCUMENT: EnemySkillDocument = EnemySkillDocument {
        need_sp: 0,
        name: "石投げ",
        text: "",
    };

    pub fn call(
        enemy_id: crate::enemys::RuntimeEnemyId,
        state: &crate::state::GameState,
        events: &mut impl crate::event::EventsQuePusher,
    ) {
    }
}
