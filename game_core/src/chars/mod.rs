use crate::lt::Potential;

pub type StaticCharId = usize;

#[derive(Debug)]
pub struct StaticCharData {
    pub id: StaticCharId,
    pub name: &'static str,
    pub potential: Potential,
}

const CHARS: [&StaticCharData; 4] = [&ELENA, &YURA, &YUUKO, &ASYA];

impl StaticCharData {
    pub fn get(id: StaticCharId) -> Option<&'static StaticCharData> {
        CHARS.get(id).copied()
    }
}

const ELENA: StaticCharData = StaticCharData {
    id: 0,
    name: "エレナ",
    potential: Potential {
        int: 16.0,
        vit: 9.0,
        str: 5.0,
        dex: 13.0,
        agi: 7.0,
    },
};

const YURA: StaticCharData = StaticCharData {
    id: 1,
    name: "ゆら",
    potential: Potential {
        int: 5.0,
        vit: 9.0,
        str: 17.0,
        dex: 11.0,
        agi: 8.0,
    },
};

const YUUKO: StaticCharData = StaticCharData {
    id: 2,
    name: "幽狐",
    potential: Potential {
        int: 14.0,
        vit: 7.0,
        str: 4.0,
        dex: 10.0,
        agi: 15.0,
    },
};

const ASYA: StaticCharData = StaticCharData {
    id: 3,
    name: "アーシャ",
    potential: Potential {
        int: 6.0,
        vit: 16.0,
        str: 10.0,
        dex: 7.0,
        agi: 11.0,
    },
};

#[cfg(test)]
mod tests {
    use crate::chars::CHARS;

    #[test]
    fn check_id() {
        assert!(
            CHARS
                .iter()
                .enumerate()
                .all(|(index, char)| { index == char.id })
        );
    }

    #[test]
    fn check_potential() {
        // ポテンシャルの合計値はちょうど50である必要がある
        CHARS.iter().for_each(| c| {
            let sum = c.potential.agi
                + c.potential.dex
                + c.potential.int
                + c.potential.str
                + c.potential.vit;

            // 小数点以下の数字を含む場合は50.0ちょうどにならない可能性があるが
            // 小数点以下の数字を使うべきではないという制約も含めてこの確認方法
            // をとっている
            assert_eq!(sum, 50.0, "char_name={}", c.name);
        });
    }
}
