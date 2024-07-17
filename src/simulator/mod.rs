mod action;
mod state;
mod tables;

pub use action::*;
pub use state::*;

#[cfg(test)]
mod tests {
    use std::str::FromStr;

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
    fn action_id_serde() {
        assert_eq!(Actions::ByregotsBlessing.as_ref(), "Byregot's Blessing");
        assert!(matches!(
            Actions::from_str("Byregot's Blessing").unwrap(),
            Actions::ByregotsBlessing
        ));
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
            Actions::Reflect,
            Actions::BasicTouch,
            Actions::CarefulSynthesis,
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

        assert!(craft_state.play_action(Actions::Reflect));
        assert_eq!(craft_state.quality, 817);

        assert!(craft_state.play_action(Actions::BasicTouch));
        assert_eq!(craft_state.quality, 1797);

        assert!(craft_state.play_action(Actions::ByregotsBlessing));
        assert_eq!(craft_state.quality, 3496);

        assert!(craft_state.play_action(Actions::CarefulSynthesis));

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

        assert!(craft_state.play_action(Actions::Reflect));
        assert_eq!(craft_state.quality, 817);

        assert!(craft_state.play_action(Actions::Innovation));
        assert!(craft_state.play_action(Actions::GreatStrides));

        assert!(craft_state.play_action(Actions::BasicTouch));
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
            Actions::Reflect,
            Actions::BasicTouch,
            Actions::BasicTouch,
            Actions::MastersMend,
            Actions::BasicTouch,
            Actions::BasicTouch,
            Actions::BasicTouch,
            Actions::MastersMend,
            Actions::BasicTouch,
            Actions::BasicTouch,
            Actions::BasicTouch,
            Actions::ByregotsBlessing,
            Actions::CarefulSynthesis,
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

        assert!(craft_state.play_action(Actions::Reflect));
        assert_eq!(craft_state.quality, 299);

        assert!(craft_state.play_action(Actions::DelicateSynthesis));
        assert_eq!(craft_state.quality, 657);

        assert!(craft_state.play_action(Actions::DelicateSynthesis));
        assert_eq!(craft_state.quality, 1045);

        assert!(craft_state.play_action(Actions::WasteNot));
        assert!(craft_state.play_action(Actions::Groundwork));
        assert!(craft_state.play_action(Actions::Innovation));

        assert!(craft_state.play_action(Actions::PreparatoryTouch));
        assert_eq!(craft_state.quality, 2300);

        assert!(craft_state.play_action(Actions::PreparatoryTouch));
        assert_eq!(craft_state.quality, 3735);

        assert!(craft_state.play_action(Actions::MastersMend));

        assert!(craft_state.play_action(Actions::PreparatoryTouch));
        assert_eq!(craft_state.quality, 5349);

        assert!(!craft_state.is_finished());

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
            Actions::Reflect,
            Actions::WasteNotII,
            Actions::PreparatoryTouch,
            Actions::PreparatoryTouch,
            Actions::PreparatoryTouch,
            Actions::PreparatoryTouch,
            Actions::PreparatoryTouch,
            Actions::Innovation,
        ];

        for a in actions_to_execute {
            assert!(craft_state.play_action(a));
        }

        assert_eq!(craft_state.quality, 5081);

        craft_state.set_next_step_outcome(0.0, StepState::Good);
        craft_state.play_action(Actions::PreparatoryTouch);
        assert_eq!(craft_state.quality, 7772);
    }

    #[test]
    fn basic_touch_combo_test() {
        let mut craft_state = GENERIC_PARAMS.new_craft();

        assert!(craft_state.play_action(Actions::StandardTouch));
        assert!(craft_state.play_action(Actions::AdvancedTouch));

        // Since Standard Touch and Advanced Touch were not comboed, there
        // should be no CP discount.
        assert_eq!(craft_state.max_cp - craft_state.cp, 32 + 46);

        let mut craft_state = GENERIC_PARAMS.new_craft();

        assert!(craft_state.play_action(Actions::BasicTouch));
        assert!(craft_state.play_action(Actions::StandardTouch));
        assert!(craft_state.play_action(Actions::AdvancedTouch));

        // Since Standard Touch and Advanced Touch were part of the combo, there
        // should be CP discounts.
        assert_eq!(craft_state.max_cp - craft_state.cp, 18 + 18 + 18);
    }

    #[test]
    fn standard_touch_should_combo_from_basic_touch() {
        let mut craft_state = GENERIC_PARAMS.new_craft();

        let actions_to_execute = vec![
            Actions::BasicTouch,
            Actions::StandardTouch,
            Actions::StandardTouch,
        ];

        for a in actions_to_execute {
            assert!(craft_state.play_action(a));
        }

        assert_eq!(craft_state.max_cp - craft_state.cp, 18 + 18 + 32);

        // The combo was broken, so there should be no CP discount for
        // Advanced Touch
        assert!(craft_state.play_action(Actions::AdvancedTouch));
        assert_eq!(craft_state.max_cp - craft_state.cp, 18 + 18 + 32 + 46);
    }

    #[test]
    fn standard_touch_should_combo_only_after_basic_touch() {
        let mut craft_state = GENERIC_PARAMS.new_craft();

        let actions_to_execute = vec![
            Actions::BasicTouch,
            Actions::GreatStrides,
            Actions::StandardTouch,
        ];

        for a in actions_to_execute {
            assert!(craft_state.play_action(a));
        }

        assert_eq!(craft_state.max_cp - craft_state.cp, 18 + 32 + 32);

        // The combo was broken, so there should be no CP discount for
        // Advanced Touch
        assert!(craft_state.play_action(Actions::AdvancedTouch));
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
            Actions::Reflect,
            Actions::BasicSynthesis,
            Actions::BasicTouch,
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
            Actions::Reflect,
            Actions::BasicSynthesis,
            Actions::BasicTouch,
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

        assert!(craft_state.play_action(Actions::BasicSynthesis));
        assert_eq!(craft_state.progress, 230);

        assert!(craft_state.play_action(Actions::BasicTouch));
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

        let actions_to_execute = vec![Actions::CarefulSynthesis];

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
            Actions::BasicTouch,
            Actions::BasicTouch,
            Actions::BasicTouch,
            Actions::BasicTouch,
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

        let actions_to_execute = [
            Actions::MuscleMemory,
            Actions::Veneration,
            Actions::Groundwork,
            Actions::Groundwork,
            Actions::Observe,
            Actions::Observe,
            Actions::CarefulSynthesis,
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

        let actions_to_execute = [
            Actions::Innovation,
            Actions::BasicTouch,
            Actions::StandardTouch,
            Actions::BasicTouch,
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

        let actions_to_execute = [
            Actions::MuscleMemory,
            Actions::Manipulation,
            Actions::Veneration,
            Actions::WasteNotII,
            Actions::Groundwork,
            Actions::Groundwork,
            Actions::DelicateSynthesis,
            Actions::PreparatoryTouch,
            Actions::PreparatoryTouch,
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
            Actions::Innovation,
            Actions::PrudentTouch,
            Actions::PrudentTouch,
            Actions::PrudentTouch,
        ];

        for a in actions_to_execute {
            assert!(craft_state.play_action(a));
        }

        assert_eq!(craft_state.quality, 667);
    }

    #[test]
    fn invalid_step_should_not_execute() {
        let mut craft_state = GENERIC_PARAMS.new_craft();

        assert!(craft_state.play_action(Actions::GreatStrides));
        assert!(!craft_state.play_action(Actions::TricksOfTheTrade));

        assert_eq!(craft_state.buffs.great_strides, 3);
        assert_eq!(craft_state.max_cp - craft_state.cp, 32);
        assert_eq!(craft_state.step_num, 1);
    }

    #[test]
    fn final_appraisal_should_not_tick_buffs() {
        let mut craft_state = GENERIC_PARAMS.new_craft();

        let actions_to_execute = vec![Actions::GreatStrides, Actions::FinalAppraisal];

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
            Actions::MuscleMemory,
            Actions::Manipulation,
            Actions::BasicTouch,
            Actions::Veneration,
            Actions::Groundwork,
        ];

        for a in actions_to_execute {
            assert!(craft_state.play_action(a));
        }

        assert_eq!(craft_state.buffs.manipulation, 5);
        assert_eq!(craft_state.buffs.inner_quiet, 1);
        assert_eq!(craft_state.buffs.veneration, 3);
    }

    #[test]
    fn excellent_test() {
        let mut craft_state = GENERIC_PARAMS.new_craft();

        assert!(craft_state.play_action(Actions::MuscleMemory));

        craft_state.set_next_step_outcome(0.0, StepState::Excellent);

        assert!(craft_state.play_action(Actions::BasicTouch));

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

        assert!(craft_state.play_action(Actions::BasicTouch));

        assert_eq!(craft_state.max_cp - craft_state.cp, 6 + 18 + 18);
        assert_eq!(craft_state.progress, 690);
        assert_eq!(craft_state.quality, 1360);
        assert_eq!(craft_state.step_state, StepState::Normal);
    }

    #[test]
    fn good_omen_test() {
        let mut craft_state = GENERIC_PARAMS.new_craft();

        assert!(craft_state.play_action(Actions::MuscleMemory));

        craft_state.set_next_step_outcome(0.0, StepState::GoodOmen);

        assert!(craft_state.play_action(Actions::BasicTouch));

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

        assert!(craft_state.play_action(Actions::BasicTouch));

        assert_eq!(craft_state.max_cp - craft_state.cp, 6 + 18 + 18);
        assert_eq!(craft_state.progress, 690);
        assert_eq!(craft_state.quality, 792);
        assert_eq!(craft_state.step_state, StepState::Normal);
    }

    #[test]
    fn pliant_step_state_should_reduce_cp() {
        let mut craft_state = GENERIC_PARAMS.new_craft();

        assert!(craft_state.play_action(Actions::MuscleMemory));

        craft_state.set_next_step_outcome(0.0, StepState::Pliant);

        assert!(craft_state.play_action(Actions::WasteNot));

        assert_eq!(craft_state.max_cp - craft_state.cp, 6 + (56 / 2));
    }

    #[test]
    fn pliant_step_state_should_reduce_cp2() {
        let mut craft_state = GENERIC_PARAMS.new_craft();

        craft_state.set_next_step_outcome(0.0, StepState::Pliant);

        assert!(craft_state.play_action(Actions::PrudentTouch));

        assert_eq!(craft_state.max_cp - craft_state.cp, 13);
    }

    #[test]
    fn sturdy_step_state_should_reduce_durability_cost() {
        let mut craft_state = GENERIC_PARAMS.new_craft();

        craft_state.set_next_step_outcome(0.0, StepState::Sturdy);

        assert!(craft_state.play_action(Actions::PrudentTouch));

        assert_eq!(
            craft_state.max_durability as i32 - craft_state.durability,
            3
        );
    }

    #[test]
    fn sturdy_step_state_should_reduce_durability_cost_with_waste_not() {
        let mut craft_state = GENERIC_PARAMS.new_craft();

        assert!(craft_state.play_action(Actions::WasteNot));

        craft_state.set_next_step_outcome(0.0, StepState::Sturdy);

        assert!(craft_state.play_action(Actions::CarefulSynthesis));

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

        assert!(craft_state.play_action(Actions::Veneration));

        craft_state.set_next_step_outcome(0.0, StepState::Malleable);

        assert!(craft_state.play_action(Actions::RapidSynthesis));

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

        assert!(craft_state.play_action(Actions::MuscleMemory));
        for (step_state, should_be_valid) in validations {
            craft_state.set_next_step_outcome(0.0, step_state);

            let action_mask = craft_state.get_valid_action_mask();
            assert_eq!(
                action_mask[Actions::TricksOfTheTrade as usize],
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

        assert!(craft_state.play_action(Actions::MuscleMemory));
        for (step_state, should_be_valid) in validations {
            craft_state.set_next_step_outcome(0.0, step_state);

            let action_mask = craft_state.get_valid_action_mask();
            assert_eq!(action_mask[Actions::PreciseTouch as usize], should_be_valid);
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

        assert!(craft_state.play_action(Actions::MuscleMemory));

        for (step_state, should_be_valid) in validations {
            craft_state.set_next_step_outcome(0.0, step_state);

            let action_mask = craft_state.get_valid_action_mask();
            assert_eq!(
                action_mask[Actions::IntensiveSynthesis as usize],
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

            assert!(craft_state.play_action(Actions::MuscleMemory));

            craft_state.set_next_step_outcome(success_rng, StepState::Normal);

            assert!(craft_state.play_action(Actions::HastyTouch));

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

            assert!(craft_state.play_action(Actions::MuscleMemory));

            craft_state.set_next_step_outcome(success_rng, StepState::Centered);

            assert!(craft_state.play_action(Actions::HastyTouch));

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

        assert!(craft_state.play_action(Actions::Observe));
        craft_state.set_next_step_outcome(0.0, StepState::Good);
        assert!(craft_state.play_action(Actions::BasicTouch));
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
            Actions::MuscleMemory,
            Actions::Veneration,
            Actions::Manipulation,
            Actions::WasteNotII,
            Actions::Groundwork,
            Actions::Groundwork,
            Actions::BasicTouch,
            Actions::StandardTouch,
            Actions::AdvancedTouch,
            Actions::PreparatoryTouch,
            Actions::PreparatoryTouch,
            Actions::Innovation,
            Actions::PreparatoryTouch,
            Actions::PreparatoryTouch,
            Actions::GreatStrides,
            Actions::ByregotsBlessing,
            Actions::BasicSynthesis,
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
            (Actions::MuscleMemory, StepState::Normal, false),
            (Actions::RapidSynthesis, StepState::Sturdy, true),
            (Actions::Manipulation, StepState::Pliant, false),
            (Actions::RapidSynthesis, StepState::Pliant, false),
            (Actions::HastyTouch, StepState::Sturdy, false),
            (Actions::Veneration, StepState::Normal, false),
            (Actions::RapidSynthesis, StepState::Normal, false),
            (Actions::Innovation, StepState::Pliant, false),
            (Actions::PreparatoryTouch, StepState::Sturdy, false),
            (Actions::PrudentTouch, StepState::Normal, false),
            (Actions::Manipulation, StepState::Pliant, false),
            (Actions::PrudentTouch, StepState::Normal, false),
            (Actions::Innovation, StepState::Pliant, false),
            (Actions::PreparatoryTouch, StepState::Good, false),
            (Actions::HastyTouch, StepState::Centered, false),
            (Actions::HastyTouch, StepState::Centered, false),
            (Actions::HastyTouch, StepState::Centered, false),
            (Actions::MastersMend, StepState::Centered, false),
            (Actions::GreatStrides, StepState::Normal, false),
            (Actions::PreparatoryTouch, StepState::Sturdy, false),
            (Actions::GreatStrides, StepState::Sturdy, false),
            (Actions::Innovation, StepState::Normal, false),
            (Actions::PreparatoryTouch, StepState::Centered, false),
            (Actions::GreatStrides, StepState::Normal, false),
            (Actions::ByregotsBlessing, StepState::Good, false),
            (Actions::PrudentTouch, StepState::Good, false),
            (Actions::CarefulSynthesis, StepState::Normal, false),
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
            (Actions::MuscleMemory, StepState::Normal, false),
            (Actions::RapidSynthesis, StepState::Sturdy, false),
            (Actions::PreciseTouch, StepState::Good, false),
            (Actions::Manipulation, StepState::Pliant, false),
            (Actions::RapidSynthesis, StepState::Malleable, false),
            (Actions::MastersMend, StepState::Pliant, false),
            (Actions::PreparatoryTouch, StepState::Pliant, false),
            (Actions::PreciseTouch, StepState::Good, false),
            (Actions::Innovation, StepState::Sturdy, false),
            (Actions::PreparatoryTouch, StepState::Primed, false),
            (Actions::PrudentTouch, StepState::Malleable, false),
            (Actions::PrudentTouch, StepState::Normal, false),
            (Actions::PreparatoryTouch, StepState::Sturdy, false),
            (Actions::MastersMend, StepState::Pliant, false),
            (Actions::Innovation, StepState::Malleable, false),
            (Actions::PreparatoryTouch, StepState::Sturdy, false),
            (Actions::AdvancedTouch, StepState::Malleable, false),
            (Actions::TrainedFinesse, StepState::Normal, false),
            (Actions::PreparatoryTouch, StepState::Sturdy, false),
            (Actions::GreatStrides, StepState::Normal, false),
            (Actions::Innovation, StepState::Normal, false),
            (Actions::ByregotsBlessing, StepState::Sturdy, false),
            (Actions::CarefulSynthesis, StepState::Sturdy, false),
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
            (Actions::MuscleMemory, StepState::Normal, false),
            (Actions::RapidSynthesis, StepState::Sturdy, false),
            (Actions::PrudentTouch, StepState::Malleable, false),
            (Actions::RapidSynthesis, StepState::Normal, true),
            (Actions::Manipulation, StepState::Pliant, false),
            (Actions::RapidSynthesis, StepState::Sturdy, true),
            (Actions::RapidSynthesis, StepState::Sturdy, false),
            (Actions::PreciseTouch, StepState::Good, false),
            (Actions::PreciseTouch, StepState::Good, false),
            (Actions::RapidSynthesis, StepState::Normal, false),
            (Actions::PreciseTouch, StepState::Good, false),
            (Actions::Innovation, StepState::Normal, false),
            (Actions::PreciseTouch, StepState::Good, false),
            (Actions::MastersMend, StepState::Centered, false),
            (Actions::HastyTouch, StepState::Centered, false),
            (Actions::PreparatoryTouch, StepState::Sturdy, false),
            (Actions::Innovation, StepState::Normal, false),
            (Actions::TrainedFinesse, StepState::Normal, false),
            (Actions::TrainedFinesse, StepState::Primed, false),
            (Actions::TrainedFinesse, StepState::Normal, false),
            (Actions::TrainedFinesse, StepState::Normal, false),
            (Actions::Innovation, StepState::Centered, false),
            (Actions::TrainedFinesse, StepState::Pliant, false),
            (Actions::TrainedFinesse, StepState::Normal, false),
            (Actions::BasicTouch, StepState::Sturdy, false),
            (Actions::GreatStrides, StepState::Primed, false),
            (Actions::ByregotsBlessing, StepState::Good, false),
            (Actions::CarefulSynthesis, StepState::Centered, false),
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
