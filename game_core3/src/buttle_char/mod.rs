use crate::{
    HateNum,
    args::CharData,
    lt_common::LtCommon,
    skill::skills::ButtleSkills,
    state::{LtId, chars::RuntimeCharId},
    static_char::StaticCharData,
};

#[derive(Debug, Clone)]
pub struct ButtleChar {
    static_data: &'static StaticCharData,
    lt_common: LtCommon,
    pub skills: ButtleSkills,
    runtime_id: RuntimeCharId,
    hate: HateNum,
}

impl ButtleChar {
    pub(crate) fn new(data: &CharData, runtime_id: RuntimeCharId) -> Self {
        let static_data = StaticCharData::get(data.static_char_id);

        let lt_common = LtCommon::new(&static_data.potential, data.level, false);
        let skills = ButtleSkills::new(&data.own_skill_ids);

        Self {
            static_data,
            lt_common,
            runtime_id,
            skills,
            hate: 0,
        }
    }

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

    pub fn lt(&self) -> &LtCommon {
        &self.lt_common
    }

    pub fn lt_mut(&mut self) -> &mut LtCommon {
        &mut self.lt_common
    }
    
    pub fn lt_id(&self) -> LtId {
        LtId::Char(self.runtime_id)
    }
}
