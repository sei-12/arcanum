use game_core6::{
    StaticSkillId,
    damage::Damage,
    effect::Effect,
    passive::PassiveInstance,
    skill::{SkillCost, SkillInstance, SkillTrait},
};

use crate::game_assets::passive::{self, KousitukaPassive};

pub enum StaticSkillIdEnum {
    Fireball = 1,
    Kousituka = 2,
    HonoonoOotatumaki = 3,
}
impl From<StaticSkillIdEnum> for game_core6::StaticSkillId {
    fn from(val: StaticSkillIdEnum) -> Self {
        val as game_core6::StaticSkillId
    }
}

#[derive(Debug)]
pub struct Fireball;
impl SkillTrait for Fireball {
    fn call(
        &self,
        user_id: game_core6::runtime_id::RuntimeCharId,
        _skill_id: game_core6::runtime_id::RuntimeSkillId,
        target_id: Option<game_core6::runtime_id::RuntimeEnemyId>,
        effector: &mut dyn game_core6::effector::EffectorTrait,
    ) -> Result<game_core6::skill::SkillCost, game_core6::WinOrLoseOrNextwave> {
        let target = effector
            .state()
            .get_enemys_highest_target_priority(target_id)
            .next()
            .unwrap();

        effector.accept_effect(Effect::Damage(Damage::new_magic_damage(
            effector.state(),
            user_id.into(),
            target.lt_id(),
            1.0,
        )))?;

        Ok(SkillCost::from_defalut(self.info()))
    }

    fn clone_instance(&self) -> SkillInstance {
        SkillInstance::new(Self)
    }

    fn info(&self) -> &game_core6::skill::SkillInfomation {
        &game_core6::skill::SkillInfomation {
            name: "ファイヤーボール",
            description: "todo",
            id: StaticSkillIdEnum::Fireball as StaticSkillId,
            default_need_mp: 50,
            defalut_hate: 50,
            defalut_cooldown: 50,
        }
    }
}

#[derive(Debug)]
pub struct Kousituka;
impl SkillTrait for Kousituka {
    fn call(
        &self,
        user_id: game_core6::runtime_id::RuntimeCharId,
        _skill_id: game_core6::runtime_id::RuntimeSkillId,
        _target_id: Option<game_core6::runtime_id::RuntimeEnemyId>,
        effector: &mut dyn game_core6::effector::EffectorTrait,
    ) -> Result<SkillCost, game_core6::WinOrLoseOrNextwave> {
        let turn_count = if effector.state().get_char(user_id).lt().vit() >= 13.0 {
            4
        } else {
            3
        };

        effector.accept_effect(Effect::AddPassive {
            target_id: user_id.into(),
            passive: PassiveInstance::new(KousitukaPassive::new(turn_count, 0)),
        })?;

        Ok(SkillCost::from_defalut(self.info()))
    }

    fn clone_instance(&self) -> SkillInstance {
        SkillInstance::new(Self)
    }

    fn info(&self) -> &game_core6::skill::SkillInfomation {
        &game_core6::skill::SkillInfomation {
            name: "硬質化",
            description: "自身に3ターンの硬質化を付与する。\n\
            VITが13以上なら代わりに4ターン付与する。\n\
            ",
            id: StaticSkillIdEnum::Kousituka as StaticSkillId,
            default_need_mp: 60,
            defalut_hate: 90,
            defalut_cooldown: 450,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HonoonoOotatumaki;
impl SkillTrait for HonoonoOotatumaki {
    fn call(
        &self,
        user_id: game_core6::runtime_id::RuntimeCharId,
        _skill_id: game_core6::runtime_id::RuntimeSkillId,
        _target_id: Option<game_core6::runtime_id::RuntimeEnemyId>,
        effector: &mut dyn game_core6::effector::EffectorTrait,
    ) -> Result<SkillCost, game_core6::WinOrLoseOrNextwave> {
        let mut enemys = effector.state().enemys_with_living_check();
        let mut dmg_mag = 1.5;
        let user = effector.state().get_char(user_id);

        if user.lt().int() >= 14.0 {
            dmg_mag += 0.3;
        }

        let flag = user.lt().dex() + user.lt().int() <= 12.0;
        let mut hate_add = 0;

        while let Some(target_enemy) = enemys.next_living_enemy(effector.state()) {
            effector.accept_effect(Effect::Damage(Damage::new_magic_damage(
                effector.state(),
                user_id.into(),
                target_enemy.lt_id(),
                dmg_mag,
            )))?;

            hate_add += 60;
        }

        if flag {
            let mut chars = effector.state().chars_with_living_check();
            while let Some(target_char) = chars.next_living_char(effector.state()) {
                let target_id = target_char.lt_id();

                effector.accept_effect(Effect::Damage(Damage::new_magic_damage(
                    effector.state(),
                    user_id.into(),
                    target_id,
                    0.3,
                )))?;

                effector.accept_effect(Effect::AddPassive {
                    target_id,
                    passive: PassiveInstance::new(passive::Burn::new(2)),
                })?;
            }
        }

        Ok(SkillCost {
            mp: self.info().default_need_mp,
            hate: self.info().defalut_hate + hate_add,
            cooldown: self.info().defalut_cooldown,
        })
    }

    fn clone_instance(&self) -> SkillInstance {
        SkillInstance::new(self.clone())
    }

    fn info(&self) -> &game_core6::skill::SkillInfomation {
        &game_core6::skill::SkillInfomation {
            name: "炎の大竜巻",
            description: "敵全員に倍率1.5の魔法ダメージを与える。\n\
            INTが14以上なら倍率を+0.3する。\n\
            ダメージを与えた敵の数だけヘイト値が+60される。\n\
            INTとDEXの合計値が12以下なら味方全員に倍率0.3の魔法ダメージと2ターンの火傷を与える。
            ",
            id: StaticSkillIdEnum::HonoonoOotatumaki as StaticSkillId,
            default_need_mp: 120,
            defalut_hate: 50,
            defalut_cooldown: 600,
        }
    }
}
