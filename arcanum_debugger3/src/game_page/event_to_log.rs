use game_core3::{
    GameResult,
    event::{self, Event},
    skill::SkillTrait,
    state::{GameState, Side},
};

pub fn event_to_log(event: &event::Event, state: &GameState) -> Option<String> {
    match event {
        Event::ConsumeSp {
            enemy_id: _,
            num: _,
        } => None,
        Event::HealSp {
            enemy_id: _,
            num: _,
        } => None,
        Event::SetSkillCooldown {
            char_id: _,
            skill_id: _,
            cooldown: _,
        } => None,
        Event::HeallSkillCooldownAll {
            char_id: _,
            heal_num: _,
        } => None,
        Event::UnFocusEnemy => None,
        Event::ChangeFocusEnemy { enemy_id: _ } => None,
        Event::AddHate { char_id, hate } => {
            let char_name = state.chars().get_char(*char_id).static_data().name;
            Some(format!("{char_name}のヘイト値が{hate}上昇した"))
        }
        Event::AddPassive { target_id, passive } => {
            let lt = state.get_lt(*target_id);
            passive
                .display()
                .map(|d| d.header)
                .map(|passive_name| format!("{}に{}が付与された", lt.name(), passive_name))
        }
        Event::ConsumeMp { mp } => Some(format!("MPを{mp}消費した")),
        Event::Damage(dmg) => {
            let msg = match dmg.causer().to_lt_id() {
                Some(causer_id) => {
                    let causer = state.get_lt(causer_id).name();
                    let target_name = state.get_lt(dmg.target()).name();
                    let ty = dmg.ty().type_str();
                    let dmg = dmg.dmg().round();
                    format!("{causer}が{target_name}に{dmg}の{ty}ダメージを与えた")
                }
                None => {
                    let target_name = state.get_lt(dmg.target()).name();
                    let ty = dmg.ty().type_str();
                    let dmg = dmg.dmg().round();
                    format!("{target_name}は{dmg}の{ty}ダメージを受けた")
                }
            };
            Some(msg)
        }
        Event::UseSkill {
            user_name,
            skill_name,
        } => Some(format!("{user_name}は{skill_name}を使った")),
        Event::HealSkillCooldown {
            char_id,
            skill_id,
            heal_num,
        } => {
            let user = state.chars().get_char(*char_id);
            let skill_name = user
                .skills
                .get(*skill_id)
                .unwrap()
                .static_data()
                .document()
                .name;

            Some(format!(
                "{}の{}のクールダウンが{}回復した",
                user.lt().name(),
                skill_name,
                heal_num
            ))
        }

        Event::GoNextWave => Some("次のウェーブへ".to_string()),

        Event::TurnStart(side) => Some(match side {
            Side::Enemy => "敵のターン".to_string(),
            Side::Player => "あなたのターン".to_string(),
        }),

        Event::Log(log) => Some(log.clone()),
        Event::HealMp { mp } => Some(format!("MPが{}回復した", mp)),

        Event::UpdatePassiveState {
            target_id: _,
            passive_id: _,
            msg: _,
        } => None,

        Event::GameEnd(result) => match result {
            GameResult::Lose => Some("LOSE".to_string()),
            GameResult::Win => Some("WIN".to_string()),
        },
    }
}
