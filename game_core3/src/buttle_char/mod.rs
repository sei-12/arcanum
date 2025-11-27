use std::ops::{Deref, DerefMut};

use crate::{
    HateNum,
    args::CharData,
    lt_common::{self, LtCommon},
    state::chars::RuntimeCharId,
    static_char::StaticCharData,
};
#[derive(Debug, Clone)]
pub struct ButtleChar {
    static_data: &'static StaticCharData,
    // char_idx: CharIdx,
    lt_common: LtCommon,
    // skills: ButtleSkills,
    runtime_id: RuntimeCharId,
    hate: HateNum,
}

impl ButtleChar {
    pub(crate) fn new(data: &CharData, runtime_id: RuntimeCharId) -> Self {
        let static_data = StaticCharData::get(data.static_char_id);
        // let skills = ButtleSkills::new(&data.own_skill_ids)?;

        let lt_common = LtCommon::new(&static_data.potential, data.level, false);

        Self {
            static_data,
            lt_common,
            runtime_id,
            hate: 0,
        }
        // Ok(Self {
        //
        //     char_idx: idx,
        //     static_data,
        //     lt_common: LtCommon::new(&static_data.potential, data.level, false),
        //     skills,
        //     hate: 0,
        // })
    }

    // /// 全ての`ButtleChar`は`ButtleChars`の子要素として作成されます
    // /// この関数は`ButtleChars`から一意に`ButtleChar`を取得するためのIdxを返します
    // pub(crate) fn char_idx(&self) -> CharIdx {
    //     self.char_idx
    // }

    pub(crate) fn runtime_id(&self) -> RuntimeCharId {
        self.runtime_id
    }
    pub fn static_data(&self) -> &'static StaticCharData {
        self.static_data
    }

    pub fn add_hate(&mut self, num: HateNum) {
        self.hate += num;
    }

    pub fn hate(&self) -> HateNum {
        self.hate
    }

    // pub fn skills(&self) -> &ButtleSkills {
    //     &self.skills
    // }
    // pub fn skills_mut(&mut self) -> &mut ButtleSkills {
    //     &mut self.skills
    // }
}

impl Deref for ButtleChar {
    type Target = LtCommon;
    fn deref(&self) -> &Self::Target {
        &self.lt_common
    }
}

impl DerefMut for ButtleChar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.lt_common
    }
}
