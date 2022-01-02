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
        job_level: 80,
        craftsmanship: 2087,
        control: 1873,
        cp: 463,

        recipe_level: 430,
        suggested_craftsmanship: 1866,
        suggested_control: 1733,

        progress: 3943,
        quality: 20287,
        durability: 80,
    };

    const R510_PARAMS: CraftParams = CraftParams {
        job_level: 80,
        craftsmanship: 2659,
        control: 2775 + 70,
        cp: 557 + 72 + 16,

        recipe_level: 510,
        suggested_craftsmanship: 2620,
        suggested_control: 2540,

        progress: 8591,
        quality: 56662,
        durability: 70,
    };

    const R513_PARAMS: CraftParams = CraftParams {
        job_level: 80,
        craftsmanship: 2763,
        control: 2795 + 70,
        cp: 572 + 72 + 16,

        recipe_level: 513,
        suggested_craftsmanship: 2620,
        suggested_control: 2540,

        progress: 12046,
        quality: 81447,
        durability: 55,
    };

    #[test]
    fn action_list_is_populated() {
        assert_eq!(ACTIONS.len(), ActionID::TrainedFinesse as usize + 1);
    }

    #[test]
    fn novice_rotation() {
        let mut craft_state = GENERIC_PARAMS.new_craft();
        let actions_to_execute = vec![
            ActionID::MuscleMemory,
            ActionID::CarefulSynthesis,
            ActionID::CarefulSynthesis,
            ActionID::FinalAppraisal,
            ActionID::CarefulSynthesis,
            ActionID::WasteNotII,
            ActionID::PreparatoryTouch,
            ActionID::PreparatoryTouch,
            ActionID::MastersMend,
            ActionID::MastersMend,
            ActionID::ByregotsBlessing,
            ActionID::CarefulSynthesis,
        ];

        for a in actions_to_execute {
            assert!(craft_state.play_action(a as usize));
        }

        assert!(craft_state.is_finished());
        assert_eq!(craft_state.progress, 4614);
        assert_eq!(craft_state.quality, 5935);
        assert_eq!(craft_state.durability, 70);
        assert_eq!(craft_state.cp, 32);
    }

    #[test]
    fn expert_rotation() {
        let mut craft_state = GENERIC_PARAMS.new_craft();
        let actions_to_execute = vec![
            ActionID::Reflect,
            ActionID::WasteNotII,
            ActionID::Innovation,
            ActionID::PreparatoryTouch,
            ActionID::PreparatoryTouch,
            ActionID::PreparatoryTouch,
            ActionID::ByregotsBlessing,
            ActionID::Veneration,
            ActionID::Groundwork,
            ActionID::Groundwork,
        ];

        for a in actions_to_execute {
            assert!(craft_state.play_action(a as usize));
        }

        assert!(craft_state.is_finished());
        assert_eq!(craft_state.progress, 4032);
        assert_eq!(craft_state.quality, 21844);
        assert_eq!(craft_state.durability, 15);
        assert_eq!(craft_state.cp, 125);
    }

    #[test]
    fn reproduces_r510_rotation() {
        let mut craft_state = R510_PARAMS.new_craft();
        let actions_to_execute = vec![
            ActionID::MuscleMemory,
            ActionID::Manipulation,
            ActionID::Veneration,
            ActionID::WasteNot,
            ActionID::Groundwork,
            ActionID::Groundwork,
            ActionID::CarefulSynthesis,
            ActionID::PreparatoryTouch,
            ActionID::PrudentTouch,
            ActionID::PrudentTouch,
            ActionID::PrudentTouch,
            ActionID::PrudentTouch,
            ActionID::Innovation,
            ActionID::PrudentTouch,
            ActionID::PrudentTouch,
            ActionID::PrudentTouch,
            ActionID::PrudentTouch,
            ActionID::GreatStrides,
            ActionID::Innovation,
            ActionID::Observe,
            ActionID::FocusedTouch,
            ActionID::GreatStrides,
            ActionID::ByregotsBlessing,
            ActionID::Observe,
            ActionID::FocusedSynthesis,
        ];

        for a in actions_to_execute {
            assert!(craft_state.play_action(a as usize));
        }

        assert_eq!(craft_state.progress, 8642);
        assert_eq!(craft_state.quality, 48787);
        assert_eq!(craft_state.cp, 7);
        assert!(craft_state.is_finished());
    }

    #[test]
    fn rotation_with_lower_stats() {
        let craft_params = CraftParams {
            craftsmanship: 1822,
            control: 1696,
            cp: 421,
            ..GENERIC_PARAMS
        };
        let mut craft_state = craft_params.new_craft();
        let actions_to_execute = vec![ActionID::Manipulation, ActionID::BasicTouch];

        for a in actions_to_execute {
            assert!(craft_state.play_action(a as usize));
        }

        assert_eq!(craft_state.progress, 1149);
        assert_eq!(craft_state.quality, 626);
        assert_eq!(craft_state.durability, 70);
        assert_eq!(craft_state.is_finished(), false);
    }

    #[test]
    fn floor_control_bonuses_test() {
        let craft_params = CraftParams {
            job_level: 80,
            craftsmanship: 2486,
            control: 2318,
            cp: 613,

            recipe_level: 480,
            suggested_craftsmanship: 2480,
            suggested_control: 2195,

            progress: 6178,
            quality: 36208,
            durability: 80,
        };
        let mut craft_state = craft_params.new_craft();
        let actions_to_execute = vec![
            ActionID::PrudentTouch,
            ActionID::PrudentTouch,
            ActionID::PrudentTouch,
            ActionID::PrudentTouch,
        ];

        for a in actions_to_execute {
            assert!(craft_state.play_action(a as usize));
        }

        assert_eq!(craft_state.quality, 2806);
    }

    #[test]
    fn muscle_memory_test() {
        let mut craft_state = GENERIC_PARAMS.new_craft();
        let actions_to_execute = vec![ActionID::MuscleMemory];

        for a in actions_to_execute {
            assert!(craft_state.play_action(a as usize));
        }

        assert_eq!(craft_state.progress, 1344);
    }

    #[test]
    fn final_appraisal_should_not_tick_buffs() {
        let mut craft_state = GENERIC_PARAMS.new_craft();

        let actions_to_execute = vec![ActionID::GreatStrides, ActionID::FinalAppraisal];

        for a in actions_to_execute {
            assert!(craft_state.play_action(a as usize));
        }

        assert_eq!(craft_state.buffs.great_strides, 3);
        assert_eq!(craft_state.buffs.final_appraisal, 5);
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
            assert!(craft_state.play_action(a as usize));
        }

        assert_eq!(craft_state.max_cp - craft_state.cp, 18 + 18 + 32);
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
            assert!(craft_state.play_action(a as usize));
        }

        assert_eq!(craft_state.buffs.manipulation, 5);
        assert_eq!(craft_state.buffs.inner_quiet, 1);
        assert_eq!(craft_state.buffs.veneration, 3);
    }

    #[test]
    fn focused_synthesis_should_chain_from_observe() {
        let mut craft_state = GENERIC_PARAMS.new_craft();

        assert!(craft_state.play_action(ActionID::MuscleMemory as usize));

        craft_state.set_next_step_outcome(1.0, StepState::Normal);
        assert!(craft_state.play_action(ActionID::FocusedSynthesis as usize));

        assert_eq!(craft_state.progress, 1344);

        assert!(craft_state.play_action(ActionID::Observe as usize));

        craft_state.set_next_step_outcome(1.0, StepState::Normal);
        assert!(craft_state.play_action(ActionID::FocusedSynthesis as usize));

        assert_eq!(craft_state.max_cp - craft_state.cp, 6 + 5 + 7 + 5);
        assert_eq!(craft_state.progress, 3136);
        assert_eq!(craft_state.quality, 0);
    }

    #[test]
    fn focused_touch_should_chain_from_observe() {
        let mut craft_state = GENERIC_PARAMS.new_craft();

        assert!(craft_state.play_action(ActionID::MuscleMemory as usize));

        craft_state.set_next_step_outcome(1.0, StepState::Normal);
        assert!(craft_state.play_action(ActionID::FocusedTouch as usize));

        assert_eq!(craft_state.quality, 0);

        assert!(craft_state.play_action(ActionID::Observe as usize));

        craft_state.set_next_step_outcome(1.0, StepState::Normal);
        assert!(craft_state.play_action(ActionID::FocusedTouch as usize));

        assert_eq!(craft_state.max_cp - craft_state.cp, 6 + 18 + 7 + 18);
        assert_eq!(craft_state.progress, 1344);
        assert_eq!(craft_state.quality, 1047);
    }

    #[test]
    fn excellent_test() {
        let mut craft_state = GENERIC_PARAMS.new_craft();

        assert!(craft_state.play_action(ActionID::MuscleMemory as usize));

        craft_state.set_next_step_outcome(0.0, StepState::Excellent);

        assert!(craft_state.play_action(ActionID::BasicTouch as usize));

        assert_eq!(craft_state.max_cp - craft_state.cp, 6 + 18);
        assert_eq!(craft_state.progress, 1344);
        assert_eq!(craft_state.quality, 2795);
        assert_eq!(craft_state.step_state, StepState::Poor);

        assert!(craft_state.play_action(ActionID::BasicTouch as usize));

        assert_eq!(craft_state.max_cp - craft_state.cp, 6 + 18 + 18);
        assert_eq!(craft_state.progress, 1344);
        assert_eq!(craft_state.quality, 3144);
        assert_eq!(craft_state.step_state, StepState::Normal);
    }

    #[test]
    fn pliant_step_state_should_reduce_cp() {
        let mut craft_state = GENERIC_PARAMS.new_craft();

        assert!(craft_state.play_action(ActionID::MuscleMemory as usize));

        craft_state.set_next_step_outcome(0.0, StepState::Pliant);

        assert!(craft_state.play_action(ActionID::WasteNot as usize));

        assert_eq!(craft_state.max_cp - craft_state.cp, 6 + (56 / 2));
    }

    #[test]
    fn pliant_step_state_should_reduce_cp2() {
        let mut craft_state = GENERIC_PARAMS.new_craft();

        craft_state.set_next_step_outcome(0.0, StepState::Pliant);

        assert!(craft_state.play_action(ActionID::PrudentTouch as usize));

        assert_eq!(craft_state.max_cp - craft_state.cp, 13);
    }

    #[test]
    fn sturdy_step_state_should_reduce_durability_cost() {
        let mut craft_state = GENERIC_PARAMS.new_craft();

        craft_state.set_next_step_outcome(0.0, StepState::Sturdy);

        assert!(craft_state.play_action(ActionID::PrudentTouch as usize));

        assert_eq!(
            craft_state.max_durability as i32 - craft_state.durability,
            3
        );
    }

    #[test]
    fn sturdy_step_state_should_reduce_durability_cost_with_waste_not() {
        let mut craft_state = GENERIC_PARAMS.new_craft();

        assert!(craft_state.play_action(ActionID::WasteNot as usize));

        craft_state.set_next_step_outcome(0.0, StepState::Sturdy);

        assert!(craft_state.play_action(ActionID::CarefulSynthesis as usize));

        assert_eq!(
            craft_state.max_durability as i32 - craft_state.durability,
            3
        );
    }

    #[test]
    fn malleable_step_state_should_floor_progress_bonuses() {
        let mut craft_state = R513_PARAMS.new_craft();

        assert!(craft_state.play_action(ActionID::Veneration as usize));

        craft_state.set_next_step_outcome(0.0, StepState::Malleable);

        assert!(craft_state.play_action(ActionID::RapidSynthesis as usize));

        assert_eq!(craft_state.progress, 5298);
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

        assert!(craft_state.play_action(ActionID::MuscleMemory as usize));
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

        assert!(craft_state.play_action(ActionID::MuscleMemory as usize));
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

        assert!(craft_state.play_action(ActionID::MuscleMemory as usize));

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

            assert!(craft_state.play_action(ActionID::MuscleMemory as usize));

            craft_state.set_next_step_outcome(success_rng, StepState::Normal);

            assert!(craft_state.play_action(ActionID::HastyTouch as usize));

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

            assert!(craft_state.play_action(ActionID::MuscleMemory as usize));

            craft_state.set_next_step_outcome(success_rng, StepState::Centered);

            assert!(craft_state.play_action(ActionID::HastyTouch as usize));

            assert_eq!(craft_state.quality > 0, should_succeed);
        }
    }

    #[test]
    fn full_expert_rotation0() {
        let mut craft_state = R513_PARAMS.new_craft();
        // Action, Current State, Fail step?
        let actions_to_execute = vec![
            (ActionID::MuscleMemory, StepState::Normal, false),
            (ActionID::RapidSynthesis, StepState::Normal, false),
            (ActionID::Manipulation, StepState::Pliant, false),
            (ActionID::RapidSynthesis, StepState::Normal, true),
            (ActionID::RapidSynthesis, StepState::Malleable, false),
            (ActionID::FinalAppraisal, StepState::Sturdy, false),
            (ActionID::RapidSynthesis, StepState::Sturdy, true),
            (ActionID::RapidSynthesis, StepState::Normal, false),
            (ActionID::PrudentTouch, StepState::Normal, false),
            (ActionID::HastyTouch, StepState::Sturdy, true),
            (ActionID::PrudentTouch, StepState::Normal, false),
            (ActionID::Manipulation, StepState::Pliant, false),
            (ActionID::PreciseTouch, StepState::Good, false),
            (ActionID::HastyTouch, StepState::Normal, false),
            (ActionID::Innovation, StepState::Normal, false),
            (ActionID::PrudentTouch, StepState::Primed, false),
            (ActionID::MastersMend, StepState::Pliant, false),
            (ActionID::HastyTouch, StepState::Normal, false),
            (ActionID::PreciseTouch, StepState::Good, false),
            (ActionID::Innovation, StepState::Normal, false),
            (ActionID::Observe, StepState::Normal, false),
            (ActionID::FocusedTouch, StepState::Malleable, false),
            (ActionID::Observe, StepState::Malleable, false),
            (ActionID::FocusedTouch, StepState::Normal, false),
            (ActionID::Innovation, StepState::Normal, false),
            (ActionID::PreciseTouch, StepState::Good, false),
            (ActionID::Manipulation, StepState::Pliant, false),
            (ActionID::GreatStrides, StepState::Normal, false),
            (ActionID::Innovation, StepState::Normal, false),
            (ActionID::PreparatoryTouch, StepState::Sturdy, false),
            (ActionID::Observe, StepState::Normal, false),
            (ActionID::PreciseTouch, StepState::Good, false),
            (ActionID::HastyTouch, StepState::Malleable, true),
            (ActionID::Innovation, StepState::Normal, false),
            (ActionID::BasicTouch, StepState::Normal, false),
            (ActionID::GreatStrides, StepState::Normal, false),
            (ActionID::ByregotsBlessing, StepState::Malleable, false),
            (ActionID::BasicSynthesis, StepState::Pliant, false),
        ];

        for (a, next_state, next_rng) in actions_to_execute {
            craft_state.set_next_step_outcome(next_rng as u8 as f32, next_state);

            let action_mask = craft_state.get_valid_action_mask();
            assert!(action_mask[a as usize]);
            craft_state.play_action(a as usize);
        }

        assert!(craft_state.is_finished());
        assert_eq!(craft_state.progress, 12575);
        assert_eq!(craft_state.quality, 84289);
        assert_eq!(craft_state.cp, 7);
    }

    #[test]
    fn full_expert_rotation1() {
        let mut craft_state = R513_PARAMS.new_craft();
        // Action, Current State, Fail step?
        let actions_to_execute = vec![
            (ActionID::MuscleMemory, StepState::Normal, false),
            (ActionID::RapidSynthesis, StepState::Normal, true),
            (ActionID::RapidSynthesis, StepState::Sturdy, false),
            (ActionID::Manipulation, StepState::Pliant, false),
            (ActionID::RapidSynthesis, StepState::Normal, true),
            (ActionID::RapidSynthesis, StepState::Primed, false),
            (ActionID::TricksOfTheTrade, StepState::Good, false),
            (ActionID::RapidSynthesis, StepState::Malleable, true),
            (ActionID::TricksOfTheTrade, StepState::Good, false),
            (ActionID::RapidSynthesis, StepState::Malleable, false),
            (ActionID::PreciseTouch, StepState::Good, false),
            (ActionID::HastyTouch, StepState::Sturdy, false),
            (ActionID::Manipulation, StepState::Primed, false),
            (ActionID::MastersMend, StepState::Pliant, false),
            (ActionID::HastyTouch, StepState::Normal, true),
            (ActionID::HastyTouch, StepState::Normal, false),
            (ActionID::PrudentTouch, StepState::Malleable, false),
            (ActionID::PreciseTouch, StepState::Good, false),
            (ActionID::Innovation, StepState::Normal, false),
            (ActionID::PrudentTouch, StepState::Primed, false),
            (ActionID::PrudentTouch, StepState::Normal, false),
            (ActionID::Manipulation, StepState::Pliant, false),
            (ActionID::HastyTouch, StepState::Normal, true),
            (ActionID::Innovation, StepState::Normal, false),
            (ActionID::Observe, StepState::Primed, false),
            (ActionID::FocusedTouch, StepState::Normal, false),
            (ActionID::GreatStrides, StepState::Pliant, false),
            (ActionID::Innovation, StepState::Malleable, false),
            (ActionID::Observe, StepState::Normal, false),
            (ActionID::FocusedTouch, StepState::Malleable, false),
            (ActionID::HastyTouch, StepState::Sturdy, false),
            (ActionID::Innovation, StepState::Primed, false),
            (ActionID::GreatStrides, StepState::Normal, false),
            (ActionID::PreparatoryTouch, StepState::Sturdy, false),
            (ActionID::Observe, StepState::Malleable, false),
            (ActionID::FocusedTouch, StepState::Normal, false),
            (ActionID::BasicTouch, StepState::Normal, false),
            (ActionID::GreatStrides, StepState::Normal, false),
            (ActionID::Innovation, StepState::Malleable, false),
            (ActionID::ByregotsBlessing, StepState::Sturdy, false),
            (ActionID::BasicSynthesis, StepState::Good, false),
        ];

        for (a, next_state, next_rng) in actions_to_execute {
            craft_state.set_next_step_outcome(next_rng as u8 as f32, next_state);
            assert!(craft_state.play_action(a as usize));
        }

        assert!(craft_state.is_finished());
        assert_eq!(craft_state.progress, 12575);
        assert_eq!(craft_state.quality, 78307);
        assert_eq!(craft_state.cp, 2);
    }

    #[test]
    fn full_expert_rotation2() {
        let mut craft_state = R513_PARAMS.new_craft();
        // Action, Current State, Fail step?
        let actions_to_execute = vec![
            (ActionID::MuscleMemory, StepState::Normal, false),
            (ActionID::TricksOfTheTrade, StepState::Good, false),
            (ActionID::PreciseTouch, StepState::Good, false),
            (ActionID::RapidSynthesis, StepState::Normal, false),
            (ActionID::PrudentTouch, StepState::Normal, false),
            (ActionID::TricksOfTheTrade, StepState::Good, false),
            (ActionID::RapidSynthesis, StepState::Malleable, false),
            (ActionID::Manipulation, StepState::Normal, false),
            (ActionID::Observe, StepState::Normal, false),
            (ActionID::FinalAppraisal, StepState::Sturdy, false),
            (ActionID::RapidSynthesis, StepState::Sturdy, false),
            (ActionID::MastersMend, StepState::Pliant, false),
            (ActionID::HastyTouch, StepState::Sturdy, false),
            (ActionID::Innovation, StepState::Malleable, false),
            (ActionID::PrudentTouch, StepState::Pliant, false),
            (ActionID::PrudentTouch, StepState::Primed, false),
            (ActionID::Manipulation, StepState::Pliant, false),
            (ActionID::PreparatoryTouch, StepState::Sturdy, false),
            (ActionID::Innovation, StepState::Normal, false),
            (ActionID::HastyTouch, StepState::Malleable, false),
            (ActionID::HastyTouch, StepState::Normal, false),
            (ActionID::PreciseTouch, StepState::Good, false),
            (ActionID::PreparatoryTouch, StepState::Sturdy, false),
            (ActionID::GreatStrides, StepState::Pliant, false),
            (ActionID::Innovation, StepState::Malleable, false),
            (ActionID::Observe, StepState::Normal, false),
            (ActionID::FocusedTouch, StepState::Sturdy, false),
            (ActionID::GreatStrides, StepState::Pliant, false),
            (ActionID::Innovation, StepState::Normal, false),
            (ActionID::Observe, StepState::Malleable, false),
            (ActionID::PreparatoryTouch, StepState::Good, false),
            (ActionID::BasicTouch, StepState::Sturdy, false),
            (ActionID::GreatStrides, StepState::Good, false),
            (ActionID::Innovation, StepState::Normal, false),
            (ActionID::ByregotsBlessing, StepState::Malleable, false),
            (ActionID::CarefulSynthesis, StepState::Normal, false),
        ];

        for (a, next_state, next_rng) in actions_to_execute {
            craft_state.set_next_step_outcome(next_rng as u8 as f32, next_state);
            assert!(craft_state.play_action(a as usize));
        }

        assert!(craft_state.is_finished());
        assert_eq!(craft_state.progress, 12716);
        assert_eq!(craft_state.quality, 92150);
        assert_eq!(craft_state.cp, 12);
    }
}
