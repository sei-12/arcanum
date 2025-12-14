use std::collections::VecDeque;

use rand::{Rng, rng};

use crate::{core_actor::CtxContainer, enemy_skill::EnemySkill, state::GameState};

#[derive(Debug)]
pub struct EnemySkillRunnner {
    schedule: VecDeque<usize>,
    current_frame: u64,
    skills: Vec<EnemySkill>,
    action_patterns: Vec<Vec<usize>>,
}
impl EnemySkillRunnner {
    pub const NUM_VIEW_SKILLS: usize = 5;
    pub fn new(skills: Vec<EnemySkill>, action_patterns: Vec<Vec<usize>>) -> Self {
        assert!({
            let range = 0..skills.len();
            action_patterns
                .iter()
                .all(|p| p.iter().all(|idx| range.contains(idx)))
        });

        let mut s = Self {
            schedule: VecDeque::new(),
            current_frame: 0,
            action_patterns,
            skills,
        };
        s.push_schedule();
        s
    }

    fn push_schedule(&mut self) {
        while self.schedule.len() < Self::NUM_VIEW_SKILLS {
            let pattern = Self::random_select_action_pattern(&self.action_patterns);
            pattern.iter().for_each(|skill_idx| {
                self.schedule.push_back(*skill_idx);
            });
        }
    }

    pub fn view_skills(&self) -> impl Iterator<Item = &EnemySkill> {
        assert!(self.schedule.len() >= Self::NUM_VIEW_SKILLS);

        let iter = self.schedule.as_slices().0[0..Self::NUM_VIEW_SKILLS]
            .iter()
            .map(|idx| self.skills.get(*idx).unwrap());

        debug_assert!(iter.clone().count() == Self::NUM_VIEW_SKILLS);

        iter
    }

    pub fn get_current_skill(&self) -> &EnemySkill {
        let idx = self.schedule.front().copied().unwrap();
        self.skills.get(idx).unwrap()
    }

    pub fn tick(&mut self) {
        self.current_frame += 1;

        let current_skill = self.get_current_skill();
        if self.current_frame >= current_skill.total_frames() {
            self.current_frame = 0;
            self.schedule.pop_front();
            self.push_schedule();
        }
    }

    pub fn frame(&self, state: &GameState, ctx: &mut CtxContainer) {
        let f = self.get_current_skill();
        if self.current_frame == f.start_up_frames {
            f.run_actions(state, ctx);
        }
    }

    fn random_select_action_pattern(action_patterns: &[Vec<usize>]) -> &Vec<usize> {
        action_patterns
            .get(rng().random_range(0..action_patterns.len()))
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::enemy_skill::EnemySkill;
    fn enemy_skill1() -> EnemySkill {
        EnemySkill {
            id: 1,
            name: "()",
            need_mp: 10.0,
            start_up_frames: 10,
            recovery_frame: 10,
            actions: vec![],
        }
    }

    fn enemy_skill2() -> EnemySkill {
        EnemySkill {
            id: 2,
            name: "()",
            need_mp: 10.0,
            start_up_frames: 10,
            recovery_frame: 10,
            actions: vec![],
        }
    }

    fn enemy_skill3() -> EnemySkill {
        EnemySkill {
            id: 3,
            name: "()",
            need_mp: 10.0,
            start_up_frames: 10,
            recovery_frame: 10,
            actions: vec![],
        }
    }

    #[test]
    fn test_enemy_skill_runner_schedule() {
        let skills = vec![enemy_skill1(), enemy_skill2()];
        let action_patterns = vec![vec![0, 1]];
        let runner = super::EnemySkillRunnner::new(skills, action_patterns);

        let mut expect_skill_id = 1;

        runner.view_skills().for_each(|skill| {
            assert_eq!(
                skill.id,
                expect_skill_id,
                "{:?}",
                runner.view_skills().collect::<Vec<_>>()
            );

            if expect_skill_id == 1 {
                expect_skill_id = 2;
            } else {
                expect_skill_id = 1;
            }
        });
    }

    #[test]
    fn test_tick() {
        let skills = vec![enemy_skill1(), enemy_skill2()];
        let action_patterns = vec![vec![0, 1]];
        let mut runner = super::EnemySkillRunnner::new(skills, action_patterns);

        for _ in 0..100 {
            for _ in 0..20 {
                let current_skill = runner.get_current_skill();
                assert_eq!(current_skill.id, 1);
                runner.tick();
            }

            for _ in 0..20 {
                let current_skill = runner.get_current_skill();
                assert_eq!(current_skill.id, 2);
                runner.tick();
            }
        }
    }

    #[test]
    fn test_random() {
        let mut skill1 = enemy_skill1();
        skill1.start_up_frames = 1;
        skill1.recovery_frame = 1;
        let mut skill2 = enemy_skill2();
        skill2.start_up_frames = 1;
        skill2.recovery_frame = 1;
        let mut skill3 = enemy_skill3();
        skill3.start_up_frames = 1;
        skill3.recovery_frame = 1;

        let skills = vec![skill1, skill2, skill3];

        let action_patterns = vec![vec![0], vec![1], vec![2]];
        let mut runner = super::EnemySkillRunnner::new(skills, action_patterns);

        let counts = &mut [0; 3];
        const N: usize = 800;
        for _ in 0..N * 3 {
            for _ in 0..2 {
                runner.tick();
            }
            let current_skill = runner.get_current_skill();
            counts[(current_skill.id - 1) as usize] += 1;
        }

        for (i, count) in counts.iter().enumerate() {
            assert!(*count > N - 50, "counts: {:?}, i={}", counts, i);
        }
    }
}
