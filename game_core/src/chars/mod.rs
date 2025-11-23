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
        int: 10.0,
        vit: 10.0,
        str: 10.0,
        dex: 10.0,
        agi: 10.0,
    },
};

const YURA: StaticCharData = StaticCharData {
    id: 1,
    name: "ゆら",
    potential: Potential {
        int: 10.0,
        vit: 10.0,
        str: 10.0,
        dex: 10.0,
        agi: 10.0,
    },
};

const YUUKO: StaticCharData = StaticCharData {
    id: 2,
    name: "幽狐",
    potential: Potential {
        int: 10.0,
        vit: 10.0,
        str: 10.0,
        dex: 10.0,
        agi: 10.0,
    },
};

const ASYA: StaticCharData = StaticCharData {
    id: 3,
    name: "アーシャ",
    potential: Potential {
        int: 10.0,
        vit: 10.0,
        str: 10.0,
        dex: 10.0,
        agi: 10.0,
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
        assert!(CHARS.iter().all(|c| {
            let sum = c.potential.agi
                + c.potential.dex
                + c.potential.int
                + c.potential.str
                + c.potential.vit;

            // 小数点以下の数字を含む場合は50.0ちょうどにならない可能性があるが
            // 小数点以下の数字を使うべきではないという制約も含めてこの確認方法
            // をとっている
            sum == 50.0
        }));
    }
}
