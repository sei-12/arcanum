mod fire_tornado;
mod fireball;
mod hametu_no_yokubou;
mod heal;
mod provoke;
mod wind_slash;

use crate::{
    Num, SkillCooltimeNum, TurnNum, container::Container, damage::DamageType, error::GameError,
    game_state::GameState,
};

const SKILLS: [&StaticActiveSkill; 6] = [
    &fireball::SKILL,
    &heal::SKILL,
    &fire_tornado::SKILL,
    &provoke::SKILL,
    &wind_slash::SKILL,
    &hametu_no_yokubou::SKILL,
];

fn get_active_skill(id: StaticSkillId) -> Option<&'static StaticActiveSkill> {
    debug_assert!(SKILLS.iter().enumerate().all(|(i, s)| s.id == i));
    SKILLS.get(id).copied()
}

pub type StaticSkillId = usize;

#[derive(Debug, Clone)]
pub struct ActiveSkillState {
    pub static_data: &'static StaticActiveSkill,
    current_cooltime: SkillCooltimeNum,
}

impl ActiveSkillState {
    pub fn new(id: StaticSkillId) -> Option<ActiveSkillState> {
        let static_data = get_active_skill(id)?;
        Some(Self {
            static_data,
            current_cooltime: 0.0,
        })
    }

    pub fn current_cooltime(&self) -> SkillCooltimeNum {
        self.current_cooltime
    }
    
    pub fn set_skill_cooltime(&mut self, time: SkillCooltimeNum) {
        self.current_cooltime = time;
    }
    
    pub fn heal_skill_cooltime(&mut self, time: SkillCooltimeNum) {
        self.current_cooltime -= time;
        if self.current_cooltime < 0.0 {
            self.current_cooltime = 0.0;
        }
    }
    
    pub fn useable(&self, state: &GameState) -> bool {
        self.current_cooltime <= 0.0 && state.player_side_mp >= self.static_data.need_mp
    }
}

#[derive(Debug)]
pub struct StaticActiveSkill {
    pub name: &'static str,
    pub id: StaticSkillId,
    pub text: &'static str,
    pub(crate) call: fn(user_static_id: usize, con: &mut Container) -> Result<(), GameError>,
    pub need_mp: Num,
}

fn dmg_msg_template(
    user_name: &str,
    skill_name: &str,
    target_name: &str,
    dmg: Num,
    dmg_type: DamageType,
) -> String {
    let type_str = match dmg_type {
        DamageType::Magic => "魔法",
        DamageType::Physics => "物理",
    };

    format!(
        "{user_name}が{skill_name}を発動!!{target_name}に{}点の{type_str}ダメージを与えた。",
        dmg.round() as u32
    )
}
