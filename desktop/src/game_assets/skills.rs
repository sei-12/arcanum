use game_core6::{
    StaticSkillId,
    damage::Damage,
    effect::Effect,
    passive::PassiveInstance,
    skill::{SkillCost, SkillInstance, SkillTrait},
};

use crate::game_assets::passive::KousitukaPassive;

pub enum StaticSkillIdEnum {
    Fireball = 1,
    Kousituka = 2,
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

    fn clone(&self) -> SkillInstance {
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

    fn clone(&self) -> SkillInstance {
        SkillInstance::new(Self)
    }

    fn info(&self) -> &game_core6::skill::SkillInfomation {
        &game_core6::skill::SkillInfomation {
            name: "硬質化",
            description: "自身に3ターンの硬質化を付与する。
            VITが13以上なら代わりに4ターン付与する。
            ",
            id: StaticSkillIdEnum::Kousituka as StaticSkillId,
            default_need_mp: 60,
            defalut_hate: 90,
            defalut_cooldown: 450,
        }
    }
}
