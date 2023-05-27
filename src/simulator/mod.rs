mod action;
mod state;
mod tables;

pub use action::*;
pub use state::*;

use std::ops::Index;

use num_derive::FromPrimitive;

#[derive(Debug, Copy, Clone, FromPrimitive)]
pub enum ActionID {
    BasicSynthesis = 0,
    BasicTouch,
    MastersMend,
    HastyTouch,
    RapidSynthesis,
    Observe,
    TricksOfTheTrade,
    WasteNot,
    Veneration,
    StandardTouch,
    GreatStrides,
    Innovation,
    FinalAppraisal,
    WasteNotII,
    ByregotsBlessing,
    PreciseTouch,
    MuscleMemory,
    CarefulSynthesis,
    Manipulation,
    PrudentTouch,
    FocusedSynthesis,
    FocusedTouch,
    Reflect,
    PreparatoryTouch,
    Groundwork,
    DelicateSynthesis,
    IntensiveSynthesis,
    AdvancedTouch,
    PrudentSynthesis,
    TrainedFinesse,
}

impl Index<ActionID> for ActionList {
    type Output = Action;

    fn index(&self, action_id: ActionID) -> &Self::Output {
        &self[action_id as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]

    const GENERIC_PARAMS: CraftParams = CraftParams {
        job_level: 90,
        craftsmanship: 2763,
        control: 2780,
        cp: 545,

        recipe_level: 517,

        progress: 2000,
        quality: 5200,
        durability: 80,

        gear_effects: GearEffects { splendorous: false },
    };

    #[test]
    fn action_list_is_populated() {
        assert_eq!(ACTIONS.len(), ActionID::TrainedFinesse as usize + 1);
    }

    #[test]
    fn reflect_test() {
        let params: CraftParams = CraftParams {
            job_level: 90,
            craftsmanship: 2278,
            control: 2348,
            cp: 532,

            recipe_level: 16,

            progress: 31,
            quality: 866,
            durability: 80,

            gear_effects: Default::default(),
        };

        let mut craft_state = params.new_craft();
        let actions_to_execute = vec![
            ActionID::Reflect,
            ActionID::BasicTouch,
            ActionID::CarefulSynthesis,
        ];

        for a in actions_to_execute {
            assert!(craft_state.play_action(a));
        }

        assert!(craft_state.is_finished());
        assert_eq!(craft_state.progress, 822);
        assert_eq!(craft_state.quality, 1797);
        assert_eq!(craft_state.durability, 50);
        assert_eq!(craft_state.buffs.inner_quiet, 3);
        assert_eq!(craft_state.cp, 501);
    }

    #[test]
    fn low_level_rotation() {
        let params: CraftParams = CraftParams {
            job_level: 90,
            craftsmanship: 2278,
            control: 2348,
            cp: 532,

            recipe_level: 16,

            progress: 31,
            quality: 866,
            durability: 80,

            gear_effects: Default::default(),
        };

        let mut craft_state = params.new_craft();

        assert!(craft_state.play_action(ActionID::Reflect));
        assert_eq!(craft_state.quality, 817);

        assert!(craft_state.play_action(ActionID::BasicTouch));
        assert_eq!(craft_state.quality, 1797);

        assert!(craft_state.play_action(ActionID::ByregotsBlessing));
        assert_eq!(craft_state.quality, 3496);

        assert!(craft_state.play_action(ActionID::CarefulSynthesis));

        assert!(craft_state.is_finished());
        assert_eq!(craft_state.progress, 822);
        assert_eq!(craft_state.quality, 3496);
        assert_eq!(craft_state.durability, 40);
        assert_eq!(craft_state.buffs.inner_quiet, 0);
        assert_eq!(craft_state.cp, 477);
    }

    #[test]
    fn great_strides_test() {
        let params: CraftParams = CraftParams {
            job_level: 90,
            craftsmanship: 2278,
            control: 2348,
            cp: 532,

            recipe_level: 16,

            progress: 31,
            quality: 866,
            durability: 80,

            gear_effects: Default::default(),
        };

        let mut craft_state = params.new_craft();

        assert!(craft_state.play_action(ActionID::Reflect));
        assert_eq!(craft_state.quality, 817);

        assert!(craft_state.play_action(ActionID::Innovation));
        assert!(craft_state.play_action(ActionID::GreatStrides));

        assert!(craft_state.play_action(ActionID::BasicTouch));
        assert_eq!(craft_state.quality, 3268);
    }

    #[test]
    fn high_stack_byregots_test() {
        let params: CraftParams = CraftParams {
            job_level: 90,
            craftsmanship: 2278,
            control: 2348,
            cp: 532,

            recipe_level: 16,

            progress: 31,
            quality: 866,
            durability: 80,

            gear_effects: Default::default(),
        };

        let mut craft_state = params.new_craft();

        let actions_to_execute = vec![
            ActionID::Reflect,
            ActionID::BasicTouch,
            ActionID::BasicTouch,
            ActionID::MastersMend,
            ActionID::BasicTouch,
            ActionID::BasicTouch,
            ActionID::BasicTouch,
            ActionID::MastersMend,
            ActionID::BasicTouch,
            ActionID::BasicTouch,
            ActionID::BasicTouch,
            ActionID::ByregotsBlessing,
            ActionID::CarefulSynthesis,
        ];

        for a in actions_to_execute {
            assert!(craft_state.play_action(a));
        }

        assert!(craft_state.is_finished());
        assert_eq!(craft_state.progress, 822);
        assert_eq!(craft_state.quality, 15846);
        assert_eq!(craft_state.durability, 30);
        assert_eq!(craft_state.buffs.inner_quiet, 0);
        assert_eq!(craft_state.cp, 175);
    }

    #[test]
    fn innovation_test() {
        let mut craft_state = GENERIC_PARAMS.new_craft();

        assert!(craft_state.play_action(ActionID::Reflect));
        assert_eq!(craft_state.quality, 299);

        assert!(craft_state.play_action(ActionID::DelicateSynthesis));
        assert_eq!(craft_state.quality, 657);

        assert!(craft_state.play_action(ActionID::DelicateSynthesis));
        assert_eq!(craft_state.quality, 1045);

        assert!(craft_state.play_action(ActionID::WasteNot));
        assert!(craft_state.play_action(ActionID::Groundwork));
        assert!(craft_state.play_action(ActionID::Innovation));

        assert!(craft_state.play_action(ActionID::PreparatoryTouch));
        assert_eq!(craft_state.quality, 2300);

        assert!(craft_state.play_action(ActionID::PreparatoryTouch));
        assert_eq!(craft_state.quality, 3735);

        assert!(craft_state.play_action(ActionID::MastersMend));

        assert!(craft_state.play_action(ActionID::PreparatoryTouch));
        assert_eq!(craft_state.quality, 5349);

        assert_eq!(craft_state.is_finished(), false);

        assert_eq!(craft_state.progress, 1288);
        assert_eq!(craft_state.quality, 5349);
        assert_eq!(craft_state.durability, 30);
        assert_eq!(craft_state.buffs.inner_quiet, 10);
        assert_eq!(craft_state.cp, 175);
    }

    #[test]
    fn control_flooring_test() {
        let mut craft_state = GENERIC_PARAMS.new_craft();

        let actions_to_execute = vec![
            ActionID::Reflect,
            ActionID::WasteNotII,
            ActionID::PreparatoryTouch,
            ActionID::PreparatoryTouch,
            ActionID::PreparatoryTouch,
            ActionID::PreparatoryTouch,
            ActionID::PreparatoryTouch,
            ActionID::Innovation,
        ];

        for a in actions_to_execute {
            assert!(craft_state.play_action(a));
        }

        assert_eq!(craft_state.quality, 5081);

        craft_state.set_next_step_outcome(0.0, StepState::Good);
        craft_state.play_action(ActionID::PreparatoryTouch);
        assert_eq!(craft_state.quality, 7772);
    }

    #[test]
    fn basic_touch_combo_test() {
        let mut craft_state = GENERIC_PARAMS.new_craft();

        assert!(craft_state.play_action(ActionID::StandardTouch));
        assert!(craft_state.play_action(ActionID::AdvancedTouch));

        // Since Standard Touch and Advanced Touch were not comboed, there
        // should be no CP discount.
        assert_eq!(craft_state.max_cp - craft_state.cp, 32 + 46);

        let mut craft_state = GENERIC_PARAMS.new_craft();

        assert!(craft_state.play_action(ActionID::BasicTouch));
        assert!(craft_state.play_action(ActionID::StandardTouch));
        assert!(craft_state.play_action(ActionID::AdvancedTouch));

        // Since Standard Touch and Advanced Touch were part of the combo, there
        // should be CP discounts.
        assert_eq!(craft_state.max_cp - craft_state.cp, 18 + 18 + 18);
    }

    #[test]
    fn standard_touch_should_combo_from_basic_touch() {
        let mut craft_state = GENERIC_PARAMS.new_craft();

        let actions_to_execute = vec![
            ActionID::BasicTouch,
            ActionID::StandardTouch,
            ActionID::StandardTouch,
        ];

        for a in actions_to_execute {
            assert!(craft_state.play_action(a));
        }

        assert_eq!(craft_state.max_cp - craft_state.cp, 18 + 18 + 32);

        // The combo was broken, so there should be no CP discount for
        // Advanced Touch
        assert!(craft_state.play_action(ActionID::AdvancedTouch));
        assert_eq!(craft_state.max_cp - craft_state.cp, 18 + 18 + 32 + 46);
    }

    #[test]
    fn standard_touch_should_combo_only_after_basic_touch() {
        let mut craft_state = GENERIC_PARAMS.new_craft();

        let actions_to_execute = vec![
            ActionID::BasicTouch,
            ActionID::GreatStrides,
            ActionID::StandardTouch,
        ];

        for a in actions_to_execute {
            assert!(craft_state.play_action(a));
        }

        assert_eq!(craft_state.max_cp - craft_state.cp, 18 + 32 + 32);

        // The combo was broken, so there should be no CP discount for
        // Advanced Touch
        assert!(craft_state.play_action(ActionID::AdvancedTouch));
        assert_eq!(craft_state.max_cp - craft_state.cp, 18 + 32 + 32 + 46);
    }

    #[test]
    fn lvl90_recipe_test() {
        let params: CraftParams = CraftParams {
            job_level: 90,
            craftsmanship: 2659,
            control: 2803,
            cp: 548,

            recipe_level: 560,

            progress: 1000,
            quality: 5200,
            durability: 80,

            gear_effects: Default::default(),
        };

        let mut craft_state = params.new_craft();
        let actions_to_execute = vec![
            ActionID::Reflect,
            ActionID::BasicSynthesis,
            ActionID::BasicTouch,
        ];

        for a in actions_to_execute {
            assert!(craft_state.play_action(a));
        }

        assert_eq!(craft_state.progress, 222);
        assert_eq!(craft_state.quality, 488);
        assert_eq!(craft_state.durability, 50);
    }

    #[test]
    fn r570_recipe_test() {
        let params: CraftParams = CraftParams {
            job_level: 90,
            craftsmanship: 3216,
            control: 3298,
            cp: 548,

            recipe_level: 570,

            progress: 3700,
            quality: 7400,
            durability: 80,

            gear_effects: Default::default(),
        };

        let mut craft_state = params.new_craft();
        let actions_to_execute = vec![
            ActionID::Reflect,
            ActionID::BasicSynthesis,
            ActionID::BasicTouch,
        ];

        for a in actions_to_execute {
            assert!(craft_state.play_action(a));
        }

        assert_eq!(craft_state.progress, 268);
        assert_eq!(craft_state.quality, 565);
        assert_eq!(craft_state.durability, 50);
    }

    #[test]
    fn r450_recipe_test() {
        let params: CraftParams = CraftParams {
            job_level: 80,
            craftsmanship: 2626,
            control: 2477,
            cp: 522,

            recipe_level: 450,

            progress: 2050,
            quality: 9000,
            durability: 70,

            gear_effects: Default::default(),
        };

        let mut craft_state = params.new_craft();

        assert!(craft_state.play_action(ActionID::BasicSynthesis));
        assert_eq!(craft_state.progress, 230);

        assert!(craft_state.play_action(ActionID::BasicTouch));
        assert_eq!(craft_state.quality, 217);
    }

    #[test]
    fn progress_flooring_test() {
        let params: CraftParams = CraftParams {
            job_level: 90,
            craftsmanship: 2606,
            control: 2457,
            cp: 507,

            recipe_level: 535,

            progress: 3000,
            quality: 6700,
            durability: 70,

            gear_effects: Default::default(),
        };

        let mut craft_state = params.new_craft();

        let actions_to_execute = vec![ActionID::CarefulSynthesis];

        for a in actions_to_execute {
            assert!(craft_state.play_action(a));
        }

        assert_eq!(craft_state.progress, 378);
    }

    #[test]
    fn basic_flooring_test() {
        let params: CraftParams = CraftParams {
            job_level: 80,
            craftsmanship: 1645,
            control: 1532,
            cp: 400,

            recipe_level: 517,

            progress: 2000,
            quality: 5200,
            durability: 70,

            gear_effects: Default::default(),
        };

        let mut craft_state = params.new_craft();

        let actions_to_execute = vec![
            ActionID::BasicTouch,
            ActionID::BasicTouch,
            ActionID::BasicTouch,
            ActionID::BasicTouch,
        ];

        for a in actions_to_execute {
            assert!(craft_state.play_action(a));
        }

        assert_eq!(craft_state.quality, 828);
    }

    #[test]
    fn basic_flooring_test2() {
        let params: CraftParams = CraftParams {
            job_level: 90,
            craftsmanship: 3289,
            control: 3420,
            cp: 400,

            recipe_level: 580,

            progress: 3900,
            quality: 10920,
            durability: 70,

            gear_effects: Default::default(),
        };

        let mut craft_state = params.new_craft();

        let actions_to_execute = vec![
            ActionID::MuscleMemory,
            ActionID::Veneration,
            ActionID::Groundwork,
            ActionID::Groundwork,
            ActionID::Observe,
            ActionID::Observe,
            ActionID::CarefulSynthesis,
        ];

        assert!(craft_state.play_action(actions_to_execute[0]));
        assert_eq!(craft_state.progress, 609);

        for a in &actions_to_execute[1..] {
            assert!(craft_state.play_action(*a));
        }

        assert_eq!(craft_state.progress, 3897);
    }

    #[test]
    fn quality_flooring_test() {
        let params: CraftParams = CraftParams {
            job_level: 58,
            craftsmanship: 2606,
            control: 434,
            cp: 507,

            recipe_level: 145,

            progress: 3000,
            quality: 6700,
            durability: 70,

            gear_effects: Default::default(),
        };

        let mut craft_state = params.new_craft();

        let actions_to_execute = vec![
            ActionID::Innovation,
            ActionID::BasicTouch,
            ActionID::StandardTouch,
            ActionID::BasicTouch,
        ];

        for a in &actions_to_execute[0..3] {
            assert!(craft_state.play_action(*a));
        }
        let old_quality = craft_state.quality;
        assert!(craft_state.play_action(actions_to_execute[3]));

        assert_eq!(craft_state.quality - old_quality, 225);
    }

    #[test]
    fn quality_flooring_test2() {
        let params: CraftParams = CraftParams {
            job_level: 90,
            craftsmanship: 3702,
            control: 3792,
            cp: 588,

            recipe_level: 610,

            progress: 5060,
            quality: 12628,
            durability: 70,

            gear_effects: Default::default(),
        };

        let mut craft_state = params.new_craft();

        let actions_to_execute = vec![
            ActionID::MuscleMemory,
            ActionID::Manipulation,
            ActionID::Veneration,
            ActionID::WasteNotII,
            ActionID::Groundwork,
            ActionID::Groundwork,
            ActionID::DelicateSynthesis,
            ActionID::PreparatoryTouch,
            ActionID::PreparatoryTouch,
        ];

        for a in &actions_to_execute[0..8] {
            assert!(craft_state.play_action(*a));
        }
        let old_quality = craft_state.quality;
        assert!(craft_state.play_action(actions_to_execute[8]));

        assert_eq!(craft_state.quality - old_quality, 663);
    }

    #[test]
    fn quality_buff_flooring_test() {
        let params: CraftParams = CraftParams {
            job_level: 66,
            craftsmanship: 813,
            control: 683,
            cp: 283,

            recipe_level: 285,

            progress: 980,
            quality: 3420,
            durability: 80,

            gear_effects: Default::default(),
        };

        let mut craft_state = params.new_craft();

        let actions_to_execute = vec![
            ActionID::Innovation,
            ActionID::PrudentTouch,
            ActionID::PrudentTouch,
            ActionID::PrudentTouch,
        ];

        for a in actions_to_execute {
            assert!(craft_state.play_action(a));
        }

        assert_eq!(craft_state.quality, 667);
    }

    #[test]
    fn invalid_step_should_not_execute() {
        let mut craft_state = GENERIC_PARAMS.new_craft();

        assert!(craft_state.play_action(ActionID::GreatStrides));
        assert_eq!(craft_state.play_action(ActionID::TricksOfTheTrade), false);

        assert_eq!(craft_state.buffs.great_strides, 3);
        assert_eq!(craft_state.max_cp - craft_state.cp, 32);
        assert_eq!(craft_state.step_num, 1);
    }

    #[test]
    fn final_appraisal_should_not_tick_buffs() {
        let mut craft_state = GENERIC_PARAMS.new_craft();

        let actions_to_execute = vec![ActionID::GreatStrides, ActionID::FinalAppraisal];

        for a in actions_to_execute {
            assert!(craft_state.play_action(a));
        }

        assert_eq!(craft_state.step_num, 1);
        assert_eq!(craft_state.buffs.great_strides, 3);
        assert_eq!(craft_state.buffs.final_appraisal, 5);
    }

    #[test]
    fn buff_durations_should_tick_properly() {
        let mut craft_state = GENERIC_PARAMS.new_craft();

        let actions_to_execute = vec![
            ActionID::MuscleMemory,
            ActionID::Manipulation,
            ActionID::BasicTouch,
            ActionID::Veneration,
            ActionID::Groundwork,
        ];

        for a in actions_to_execute {
            assert!(craft_state.play_action(a));
        }

        assert_eq!(craft_state.buffs.manipulation, 5);
        assert_eq!(craft_state.buffs.inner_quiet, 1);
        assert_eq!(craft_state.buffs.veneration, 3);
    }

    #[test]
    fn focused_synthesis_should_chain_from_observe() {
        let mut craft_state = GENERIC_PARAMS.new_craft();

        assert!(craft_state.play_action(ActionID::MuscleMemory));
        assert_eq!(craft_state.progress, 690);

        craft_state.set_next_step_outcome(1.0, StepState::Normal);
        assert!(craft_state.play_action(ActionID::FocusedSynthesis));
        assert_eq!(craft_state.progress, 690);

        assert!(craft_state.play_action(ActionID::Observe));

        craft_state.set_next_step_outcome(1.0, StepState::Normal);
        assert!(craft_state.play_action(ActionID::FocusedSynthesis));

        assert_eq!(craft_state.max_cp - craft_state.cp, 6 + 5 + 7 + 5);
        assert_eq!(craft_state.progress, 1610);
        assert_eq!(craft_state.quality, 0);
    }

    #[test]
    fn focused_touch_should_chain_from_observe() {
        let mut craft_state = GENERIC_PARAMS.new_craft();

        assert!(craft_state.play_action(ActionID::MuscleMemory));

        craft_state.set_next_step_outcome(1.0, StepState::Normal);
        assert!(craft_state.play_action(ActionID::FocusedTouch));

        assert_eq!(craft_state.quality, 0);

        assert!(craft_state.play_action(ActionID::Observe));

        craft_state.set_next_step_outcome(1.0, StepState::Normal);
        assert!(craft_state.play_action(ActionID::FocusedTouch));

        assert_eq!(craft_state.max_cp - craft_state.cp, 6 + 18 + 7 + 18);
        assert_eq!(craft_state.progress, 690);
        assert_eq!(craft_state.quality, 448);
    }

    #[test]
    fn excellent_test() {
        let mut craft_state = GENERIC_PARAMS.new_craft();

        assert!(craft_state.play_action(ActionID::MuscleMemory));

        craft_state.set_next_step_outcome(0.0, StepState::Excellent);

        assert!(craft_state.play_action(ActionID::BasicTouch));

        assert_eq!(craft_state.max_cp - craft_state.cp, 6 + 18);
        assert_eq!(craft_state.progress, 690);
        assert_eq!(craft_state.quality, 1196);
        assert_eq!(craft_state.step_state, StepState::Poor);

        // Setting the next state outcome should not change the step state from
        // poor
        for _ in 0..50 {
            craft_state.set_next_state_rng();
            assert_eq!(craft_state.step_state, StepState::Poor);
        }

        assert!(craft_state.play_action(ActionID::BasicTouch));

        assert_eq!(craft_state.max_cp - craft_state.cp, 6 + 18 + 18);
        assert_eq!(craft_state.progress, 690);
        assert_eq!(craft_state.quality, 1360);
        assert_eq!(craft_state.step_state, StepState::Normal);
    }

    #[test]
    fn good_omen_test() {
        let mut craft_state = GENERIC_PARAMS.new_craft();

        assert!(craft_state.play_action(ActionID::MuscleMemory));

        craft_state.set_next_step_outcome(0.0, StepState::GoodOmen);

        assert!(craft_state.play_action(ActionID::BasicTouch));

        assert_eq!(craft_state.max_cp - craft_state.cp, 6 + 18);
        assert_eq!(craft_state.progress, 690);
        assert_eq!(craft_state.quality, 299);
        assert_eq!(craft_state.step_state, StepState::Good);

        // Setting the next state outcome should not change the step state from
        // poor
        for _ in 0..50 {
            craft_state.set_next_state_rng();
            assert_eq!(craft_state.step_state, StepState::Good);
        }

        assert!(craft_state.play_action(ActionID::BasicTouch));

        assert_eq!(craft_state.max_cp - craft_state.cp, 6 + 18 + 18);
        assert_eq!(craft_state.progress, 690);
        assert_eq!(craft_state.quality, 792);
        assert_eq!(craft_state.step_state, StepState::Normal);
    }

    #[test]
    fn pliant_step_state_should_reduce_cp() {
        let mut craft_state = GENERIC_PARAMS.new_craft();

        assert!(craft_state.play_action(ActionID::MuscleMemory));

        craft_state.set_next_step_outcome(0.0, StepState::Pliant);

        assert!(craft_state.play_action(ActionID::WasteNot));

        assert_eq!(craft_state.max_cp - craft_state.cp, 6 + (56 / 2));
    }

    #[test]
    fn pliant_step_state_should_reduce_cp2() {
        let mut craft_state = GENERIC_PARAMS.new_craft();

        craft_state.set_next_step_outcome(0.0, StepState::Pliant);

        assert!(craft_state.play_action(ActionID::PrudentTouch));

        assert_eq!(craft_state.max_cp - craft_state.cp, 13);
    }

    #[test]
    fn sturdy_step_state_should_reduce_durability_cost() {
        let mut craft_state = GENERIC_PARAMS.new_craft();

        craft_state.set_next_step_outcome(0.0, StepState::Sturdy);

        assert!(craft_state.play_action(ActionID::PrudentTouch));

        assert_eq!(
            craft_state.max_durability as i32 - craft_state.durability,
            3
        );
    }

    #[test]
    fn sturdy_step_state_should_reduce_durability_cost_with_waste_not() {
        let mut craft_state = GENERIC_PARAMS.new_craft();

        assert!(craft_state.play_action(ActionID::WasteNot));

        craft_state.set_next_step_outcome(0.0, StepState::Sturdy);

        assert!(craft_state.play_action(ActionID::CarefulSynthesis));

        assert_eq!(
            craft_state.max_durability as i32 - craft_state.durability,
            3
        );
    }

    #[test]
    fn malleable_step_state_should_floor_progress_bonuses() {
        let params = CraftParams {
            job_level: 90,
            craftsmanship: 2763,
            control: 2780,
            cp: 545,

            recipe_level: 513,

            progress: 5059,
            quality: 15474,
            durability: 55,

            gear_effects: Default::default(),
        };

        let mut craft_state = params.new_craft();

        assert!(craft_state.play_action(ActionID::Veneration));

        craft_state.set_next_step_outcome(0.0, StepState::Malleable);

        assert!(craft_state.play_action(ActionID::RapidSynthesis));

        assert_eq!(craft_state.progress, 2238);
    }

    #[test]
    fn tricks_should_only_be_valid_on_good_or_excellent() {
        let mut craft_state = GENERIC_PARAMS.new_craft();

        let validations = vec![
            (StepState::Normal, false),
            (StepState::Good, true),
            (StepState::Excellent, true),
            (StepState::Poor, false),
            (StepState::Centered, false),
            (StepState::Sturdy, false),
            (StepState::Pliant, false),
            (StepState::Malleable, false),
            (StepState::Primed, false),
        ];

        assert!(craft_state.play_action(ActionID::MuscleMemory));
        for (step_state, should_be_valid) in validations {
            craft_state.set_next_step_outcome(0.0, step_state);

            let action_mask = craft_state.get_valid_action_mask();
            assert_eq!(
                action_mask[ActionID::TricksOfTheTrade as usize],
                should_be_valid
            );
        }
    }

    #[test]
    fn precise_touch_should_only_be_valid_on_good_or_excellent() {
        let mut craft_state = GENERIC_PARAMS.new_craft();

        let validations = vec![
            (StepState::Normal, false),
            (StepState::Good, true),
            (StepState::Excellent, true),
            (StepState::Poor, false),
            (StepState::Centered, false),
            (StepState::Sturdy, false),
            (StepState::Pliant, false),
            (StepState::Malleable, false),
            (StepState::Primed, false),
        ];

        assert!(craft_state.play_action(ActionID::MuscleMemory));
        for (step_state, should_be_valid) in validations {
            craft_state.set_next_step_outcome(0.0, step_state);

            let action_mask = craft_state.get_valid_action_mask();
            assert_eq!(
                action_mask[ActionID::PreciseTouch as usize],
                should_be_valid
            );
        }
    }

    #[test]
    fn intensive_synth_should_only_be_valid_on_good_or_excellent() {
        let mut craft_state = GENERIC_PARAMS.new_craft();

        let validations = vec![
            (StepState::Normal, false),
            (StepState::Good, true),
            (StepState::Excellent, true),
            (StepState::Poor, false),
            (StepState::Centered, false),
            (StepState::Sturdy, false),
            (StepState::Pliant, false),
            (StepState::Malleable, false),
            (StepState::Primed, false),
        ];

        assert!(craft_state.play_action(ActionID::MuscleMemory));

        for (step_state, should_be_valid) in validations {
            craft_state.set_next_step_outcome(0.0, step_state);

            let action_mask = craft_state.get_valid_action_mask();
            assert_eq!(
                action_mask[ActionID::IntensiveSynthesis as usize],
                should_be_valid
            );
        }
    }

    #[test]
    fn non_centered_should_have_normal_success_rates() {
        let validations = vec![
            (0.0, true),
            (0.2, true),
            (0.4, true),
            (0.599, true),
            (0.6, false),
            (0.8, false),
            (1.0, false),
        ];

        for (success_rng, should_succeed) in validations {
            let mut craft_state = GENERIC_PARAMS.new_craft();

            assert!(craft_state.play_action(ActionID::MuscleMemory));

            craft_state.set_next_step_outcome(success_rng, StepState::Normal);

            assert!(craft_state.play_action(ActionID::HastyTouch));

            assert_eq!(craft_state.quality > 0, should_succeed);
        }
    }

    #[test]
    fn centered_should_improve_success_rates() {
        let validations = vec![
            (0.0, true),
            (0.2, true),
            (0.4, true),
            (0.599, true),
            (0.6, true),
            (0.8, true),
            (0.8499, true),
            (0.85, false),
            (1.0, false),
        ];

        for (success_rng, should_succeed) in validations {
            let mut craft_state = GENERIC_PARAMS.new_craft();

            assert!(craft_state.play_action(ActionID::MuscleMemory));

            craft_state.set_next_step_outcome(success_rng, StepState::Centered);

            assert!(craft_state.play_action(ActionID::HastyTouch));

            assert_eq!(craft_state.quality > 0, should_succeed);
        }
    }

    #[test]
    fn splendorous_tools_should_have_boosted_good_modifier() {
        let params = CraftParams {
            job_level: 90,
            craftsmanship: 4041,
            control: 3987,
            cp: 616,

            recipe_level: 1,

            progress: 9,
            quality: 80,
            durability: 80,

            gear_effects: GearEffects { splendorous: true },
        };
        let mut craft_state = params.new_craft();

        assert!(craft_state.play_action(ActionID::Observe));
        craft_state.set_next_step_outcome(0.0, StepState::Good);
        assert!(craft_state.play_action(ActionID::BasicTouch));
        assert_eq!(craft_state.quality, 2387);
    }

    #[test]
    fn benchmark_rotation() {
        let params = CraftParams {
            job_level: 90,
            craftsmanship: 2763,
            control: 2780,
            cp: 545,

            recipe_level: 560,

            progress: 3500,
            quality: 7200,
            durability: 80,

            gear_effects: Default::default(),
        };
        let mut craft_state = params.new_craft();

        let actions_to_execute = vec![
            ActionID::MuscleMemory,
            ActionID::Veneration,
            ActionID::Manipulation,
            ActionID::WasteNotII,
            ActionID::Groundwork,
            ActionID::Groundwork,
            ActionID::BasicTouch,
            ActionID::StandardTouch,
            ActionID::AdvancedTouch,
            ActionID::PreparatoryTouch,
            ActionID::PreparatoryTouch,
            ActionID::Innovation,
            ActionID::PreparatoryTouch,
            ActionID::PreparatoryTouch,
            ActionID::GreatStrides,
            ActionID::ByregotsBlessing,
            ActionID::BasicSynthesis,
        ];

        for a in actions_to_execute {
            assert!(craft_state.play_action(a));
        }

        assert!(craft_state.is_finished());
        assert_eq!(craft_state.progress, 3589);
        assert_eq!(craft_state.quality, 7859);
        assert_eq!(craft_state.cp, 3);
    }

    #[test]
    fn full_expert_rotation_a() {
        let params = CraftParams {
            job_level: 90,
            craftsmanship: 3216,
            control: 3368,
            cp: 621,

            recipe_level: 514,

            progress: 5077,
            quality: 14321,
            durability: 55,

            gear_effects: Default::default(),
        };
        let mut craft_state = params.new_craft();
        // Action, Current State, Fail step?
        let actions_to_execute = vec![
            (ActionID::MuscleMemory, StepState::Normal, false),
            (ActionID::RapidSynthesis, StepState::Sturdy, true),
            (ActionID::Manipulation, StepState::Pliant, false),
            (ActionID::RapidSynthesis, StepState::Pliant, false),
            (ActionID::HastyTouch, StepState::Sturdy, false),
            (ActionID::Veneration, StepState::Normal, false),
            (ActionID::RapidSynthesis, StepState::Normal, false),
            (ActionID::Innovation, StepState::Pliant, false),
            (ActionID::PreparatoryTouch, StepState::Sturdy, false),
            (ActionID::PrudentTouch, StepState::Normal, false),
            (ActionID::Manipulation, StepState::Pliant, false),
            (ActionID::PrudentTouch, StepState::Normal, false),
            (ActionID::Innovation, StepState::Pliant, false),
            (ActionID::PreparatoryTouch, StepState::Good, false),
            (ActionID::HastyTouch, StepState::Centered, false),
            (ActionID::HastyTouch, StepState::Centered, false),
            (ActionID::HastyTouch, StepState::Centered, false),
            (ActionID::MastersMend, StepState::Centered, false),
            (ActionID::GreatStrides, StepState::Normal, false),
            (ActionID::PreparatoryTouch, StepState::Sturdy, false),
            (ActionID::GreatStrides, StepState::Sturdy, false),
            (ActionID::Innovation, StepState::Normal, false),
            (ActionID::PreparatoryTouch, StepState::Centered, false),
            (ActionID::GreatStrides, StepState::Normal, false),
            (ActionID::ByregotsBlessing, StepState::Good, false),
            (ActionID::PrudentTouch, StepState::Good, false),
            (ActionID::CarefulSynthesis, StepState::Normal, false),
        ];

        for (a, next_state, next_rng) in actions_to_execute {
            craft_state.set_next_step_outcome(next_rng as u8 as f32, next_state);

            let action_mask = craft_state.get_valid_action_mask();
            assert!(action_mask[a as usize]);
            craft_state.play_action(a);
        }

        assert!(craft_state.is_finished());
        assert_eq!(craft_state.progress, 5150);
        assert_eq!(craft_state.quality, 19385);
        assert_eq!(craft_state.durability, -5);
        assert_eq!(craft_state.cp, 15);
    }

    #[test]
    fn full_expert_rotation_b() {
        let params = CraftParams {
            job_level: 90,
            craftsmanship: 3216,
            control: 3368,
            cp: 621,

            recipe_level: 515,

            progress: 5095,
            quality: 14854,
            durability: 55,

            gear_effects: Default::default(),
        };
        let mut craft_state = params.new_craft();
        // Action, Current State, Fail step?
        let actions_to_execute = vec![
            (ActionID::MuscleMemory, StepState::Normal, false),
            (ActionID::RapidSynthesis, StepState::Sturdy, false),
            (ActionID::PreciseTouch, StepState::Good, false),
            (ActionID::Manipulation, StepState::Pliant, false),
            (ActionID::RapidSynthesis, StepState::Malleable, false),
            (ActionID::MastersMend, StepState::Pliant, false),
            (ActionID::PreparatoryTouch, StepState::Pliant, false),
            (ActionID::PreciseTouch, StepState::Good, false),
            (ActionID::Innovation, StepState::Sturdy, false),
            (ActionID::PreparatoryTouch, StepState::Primed, false),
            (ActionID::PrudentTouch, StepState::Malleable, false),
            (ActionID::PrudentTouch, StepState::Normal, false),
            (ActionID::PreparatoryTouch, StepState::Sturdy, false),
            (ActionID::MastersMend, StepState::Pliant, false),
            (ActionID::Innovation, StepState::Malleable, false),
            (ActionID::PreparatoryTouch, StepState::Sturdy, false),
            (ActionID::AdvancedTouch, StepState::Malleable, false),
            (ActionID::TrainedFinesse, StepState::Normal, false),
            (ActionID::PreparatoryTouch, StepState::Sturdy, false),
            (ActionID::GreatStrides, StepState::Normal, false),
            (ActionID::Innovation, StepState::Normal, false),
            (ActionID::ByregotsBlessing, StepState::Sturdy, false),
            (ActionID::CarefulSynthesis, StepState::Sturdy, false),
        ];

        for (a, next_state, next_rng) in actions_to_execute {
            craft_state.set_next_step_outcome(next_rng as u8 as f32, next_state);

            let action_mask = craft_state.get_valid_action_mask();
            assert!(action_mask[a as usize]);
            craft_state.play_action(a);
        }

        assert!(craft_state.is_finished());
        assert_eq!(craft_state.progress, 5150);
        assert_eq!(craft_state.quality, 17240);
        assert_eq!(craft_state.durability, 5);
        assert_eq!(craft_state.cp, 18);
    }

    #[test]
    fn full_expert_rotation_c() {
        let params = CraftParams {
            job_level: 90,
            craftsmanship: 3216,
            control: 3368,
            cp: 621,

            recipe_level: 516,

            progress: 5470,
            quality: 16156,
            durability: 60,

            gear_effects: Default::default(),
        };
        let mut craft_state = params.new_craft();
        // Action, Current State, Fail step?
        let actions_to_execute = vec![
            (ActionID::MuscleMemory, StepState::Normal, false),
            (ActionID::RapidSynthesis, StepState::Sturdy, false),
            (ActionID::PrudentTouch, StepState::Malleable, false),
            (ActionID::RapidSynthesis, StepState::Normal, true),
            (ActionID::Manipulation, StepState::Pliant, false),
            (ActionID::RapidSynthesis, StepState::Sturdy, true),
            (ActionID::RapidSynthesis, StepState::Sturdy, false),
            (ActionID::PreciseTouch, StepState::Good, false),
            (ActionID::PreciseTouch, StepState::Good, false),
            (ActionID::RapidSynthesis, StepState::Normal, false),
            (ActionID::PreciseTouch, StepState::Good, false),
            (ActionID::Innovation, StepState::Normal, false),
            (ActionID::PreciseTouch, StepState::Good, false),
            (ActionID::MastersMend, StepState::Centered, false),
            (ActionID::HastyTouch, StepState::Centered, false),
            (ActionID::PreparatoryTouch, StepState::Sturdy, false),
            (ActionID::Innovation, StepState::Normal, false),
            (ActionID::TrainedFinesse, StepState::Normal, false),
            (ActionID::TrainedFinesse, StepState::Primed, false),
            (ActionID::TrainedFinesse, StepState::Normal, false),
            (ActionID::TrainedFinesse, StepState::Normal, false),
            (ActionID::Innovation, StepState::Centered, false),
            (ActionID::TrainedFinesse, StepState::Pliant, false),
            (ActionID::TrainedFinesse, StepState::Normal, false),
            (ActionID::BasicTouch, StepState::Sturdy, false),
            (ActionID::GreatStrides, StepState::Primed, false),
            (ActionID::ByregotsBlessing, StepState::Good, false),
            (ActionID::CarefulSynthesis, StepState::Centered, false),
        ];

        for (a, next_state, next_rng) in actions_to_execute {
            craft_state.set_next_step_outcome(next_rng as u8 as f32, next_state);

            let action_mask = craft_state.get_valid_action_mask();
            assert!(action_mask[a as usize]);
            craft_state.play_action(a);
        }

        assert!(craft_state.is_finished());
        assert_eq!(craft_state.progress, 5728);
        assert_eq!(craft_state.quality, 18625);
        assert_eq!(craft_state.durability, -5);
        assert_eq!(craft_state.cp, 31);
    }
}
