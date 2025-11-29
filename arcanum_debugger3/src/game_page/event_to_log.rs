use game_core3::{
    event::{self, Event},
    skill::SkillTrait,
    state::{GameState, Side},
};

pub fn event_to_log(event: &event::Event, state: &GameState) -> Option<String> {
    match event {
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
        Event::DeadEnemy { enemy_id } => {
            let enemy_name = state.enemys().get(*enemy_id).lt().name();
            Some(format!("{enemy_name}は戦闘不能になった"))
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
        _ => None,
    }
}
