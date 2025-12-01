#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq)]
pub struct RuntimeCharId {
    idx: usize,
}

use std::collections::HashSet;

use crate::{
    HateNum, NUM_MAX_CHAR_IN_TEAM, args, buttle_char::ButtleChar, static_char::StaticCharId,
};

#[derive(Debug, Clone)]
pub struct ButtleChars {
    chars: Vec<ButtleChar>,
}

impl ButtleChars {
    pub(crate) fn new(char_datas: &[args::CharData]) -> Result<Self, crate::Error> {
        if char_datas.is_empty() || char_datas.len() > NUM_MAX_CHAR_IN_TEAM {
            return Err(crate::Error::InvalidNumTeamMembers {
                got_num_members: char_datas.len(),
            });
        }

        if let Err(char_id) = check_confrict_char(char_datas) {
            return Err(crate::Error::ConfrictChar(char_id));
        }

        let mut chars = Vec::with_capacity(char_datas.len());

        for (i, char_data) in char_datas.iter().enumerate() {
            let idx = RuntimeCharId { idx: i };
            chars.push(ButtleChar::new(char_data, idx));
        }

        assert_eq!(chars.len(), char_datas.len());

        debug_assert!({
            chars
                .iter()
                .zip(char_datas)
                .enumerate()
                .all(|(i, (buttle_char, char_data))| {
                    buttle_char.runtime_id().idx == i
                        && buttle_char.static_data().id == char_data.static_char_id
                        && buttle_char.lt().level() == char_data.level
                })
        });

        debug_assert!({
            let set = chars
                .iter()
                .map(|c| c.static_data().id)
                .collect::<HashSet<StaticCharId>>();
            set.len() == chars.len()
        });

        Ok(Self { chars })
    }
}

fn check_confrict_char(chars: &[args::CharData]) -> Result<(), StaticCharId> {
    let mut set = HashSet::new();
    for char in chars.iter() {
        if set.contains(&char.static_char_id) {
            return Err(char.static_char_id);
        }
        set.insert(char.static_char_id);
    }
    Ok(())
}

//--------------------------------------------------//
//                       READ                       //
//--------------------------------------------------//
impl ButtleChars {
    pub fn get_char_by_static_id(&self, id: StaticCharId) -> Result<&ButtleChar, crate::Error> {
        self.chars
            .iter()
            .find(|char| char.static_data().id == id)
            .ok_or(crate::Error::NotFoundChar(id))
    }

    pub fn get_char(&self, id: RuntimeCharId) -> &ButtleChar {
        let char = self
            .chars
            .get(id.idx)
            .unwrap_or_else(|| panic!("不正なButtleTimeCharId: {:?}", id));

        assert_eq!(char.runtime_id(), id);

        char
    }

    pub fn chars(&self) -> &Vec<ButtleChar> {
        &self.chars
    }

    pub(crate) fn get_highest_hate_char(&self) -> &ButtleChar {
        assert!(!self.chars.is_empty());
        // MEMO: iter().max()は最後の値になってしまう。
        // 最初の値を取得する方法が見つかったら書き換えたい
        let mut max_hate_char = &self.chars[0];
        for char in self.chars.iter().skip(1) {
            if max_hate_char.hate() < char.hate() {
                max_hate_char = char
            }
        }

        debug_assert!({
            let v = self.chars_sorted_by_hate();
            v.first().unwrap().runtime_id() == max_hate_char.runtime_id()
        });

        max_hate_char
    }

    pub fn chars_sorted_by_hate(&self) -> Vec<&ButtleChar> {
        let mut chars = self.chars.iter().collect::<Vec<_>>();
        chars.sort_by_key(|char| HateNum::MAX - char.hate());
        assert_eq!(self.chars.len(), chars.len());
        chars
    }
}

//--------------------------------------------------//
//                      WRITE                       //
//--------------------------------------------------//
impl ButtleChars {
    pub(crate) fn get_mut_char(&mut self, id: RuntimeCharId) -> &mut ButtleChar {
        let char = self
            .chars
            .get_mut(id.idx)
            .unwrap_or_else(|| panic!("不正なButtleTimeCharId: {:?}", id));

        assert_eq!(char.runtime_id(), id);

        char
    }
}

#[cfg(test)]
mod tests {
    use crate::{args, state::chars::ButtleChars, static_char::StaticCharId};

    #[test]
    fn test_new() {
        let args = [
            args::CharData {
                level: 1,
                static_char_id: crate::static_char::StaticCharId::Asya,
                own_skill_ids: vec![],
            },
            args::CharData {
                level: 1,
                static_char_id: crate::static_char::StaticCharId::Elena,
                own_skill_ids: vec![],
            },
            args::CharData {
                level: 1,
                static_char_id: crate::static_char::StaticCharId::Yura,
                own_skill_ids: vec![],
            },
            args::CharData {
                level: 1,
                static_char_id: crate::static_char::StaticCharId::Yuuko,
                own_skill_ids: vec![],
            },
        ];

        let chars = ButtleChars::new(&args).unwrap();
        let elena = chars
            .get_char_by_static_id(crate::static_char::StaticCharId::Elena)
            .unwrap();
        assert_eq!(elena.static_data().name, "エレナ");
        assert_eq!(
            chars.get_char(elena.runtime_id()).static_data().id,
            StaticCharId::Elena
        );
        assert_eq!(elena.lt().level(), 1);
    }

    #[test]
    fn test_new_err_invalid_num_members() {
        let args = [];
        let err = ButtleChars::new(&args).unwrap_err();
        assert!(matches!(
            err,
            crate::Error::InvalidNumTeamMembers { got_num_members: 0 }
        ));
    }

    #[test]
    fn test_new_err_confrict_char() {
        let args = [
            args::CharData {
                level: 1,
                static_char_id: StaticCharId::Asya,
                own_skill_ids: vec![],
            },
            args::CharData {
                level: 1,
                static_char_id: StaticCharId::Asya,
                own_skill_ids: vec![],
            },
        ];
        let err = ButtleChars::new(&args).unwrap_err();
        assert!(matches!(
            err,
            crate::Error::ConfrictChar(StaticCharId::Asya)
        ));
    }
}
