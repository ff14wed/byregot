use std::hash::{Hash, Hasher};

use super::{tables, Action, Change};

use rand::distributions::WeightedIndex;
use rand::prelude::*;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum StepState {
    Normal = 0,
    Good,
    Excellent,
    Poor,
    Centered,
    Sturdy,
    Pliant,
    Malleable,
    Primed,
    GoodOmen,
}

pub struct CraftParams {
    pub job_level: u32,
    pub craftsmanship: u32,
    pub control: u32,
    pub cp: u32,

    pub recipe_level: u32,

    pub progress: u32,
    pub quality: u32,
    pub durability: u32,

    pub gear_effects: GearEffects,
}

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone, Hash)]
pub struct GearEffects {
    pub splendorous: bool,
}

impl CraftParams {
    pub fn new_craft(&self) -> CraftState {
        CraftState {
            step_num: 0,

            progress: 0,
            max_progress: self.progress,

            quality: 0,
            max_quality: self.quality,

            durability: self.durability as i32,
            max_durability: self.durability,

            cp: self.cp,
            max_cp: self.cp,

            gear_effects: self.gear_effects,

            buffs: Default::default(),

            did_trained_perfection: false,

            did_observe: false,
            prev_basic_touch_combo: 0,
            basic_touch_combo: 0,

            step_state: StepState::Normal,
            next_success_rng: 0.0,

            was_primed: false,
            was_excellent: false,
            was_good_omen: false,

            recipe_level: self.recipe_level,
            base_progress: self.get_base_progress().floor(),
            base_quality: self.get_base_quality().floor(),
        }
    }

    fn get_base_progress(&self) -> f64 {
        let crafter_level = tables::get_crafter_level(self.job_level);
        // Will panic if recipe_level is out of range
        let modifiers = &tables::RECIPE_LEVEL_TABLE[self.recipe_level as usize];

        let base_value = (self.craftsmanship as f64 * 10.) / modifiers.progress_divider() + 2.;
        if crafter_level <= self.recipe_level {
            return base_value * modifiers.progress_modifier() * (0.01_f32 as f64);
        }
        base_value
    }

    fn get_base_quality(&self) -> f64 {
        let crafter_level = tables::get_crafter_level(self.job_level);
        // Will panic if recipe_level is out of range
        let modifiers = &tables::RECIPE_LEVEL_TABLE[self.recipe_level as usize];

        let base_value = (self.control as f64 * 10.) / modifiers.quality_divider() + 35.;
        if crafter_level <= self.recipe_level {
            return base_value * modifiers.quality_modifier() * (0.01_f32 as f64);
        }
        base_value
    }
}

#[derive(Debug, Copy, Clone)]
pub struct CraftState {
    pub step_num: u32,

    pub progress: u32,
    pub max_progress: u32,

    pub quality: u32,
    pub max_quality: u32,

    pub durability: i32,
    pub max_durability: u32,

    pub cp: u32,
    pub max_cp: u32,

    pub gear_effects: GearEffects,

    pub buffs: BuffStacks,

    pub did_trained_perfection: bool,

    pub did_observe: bool,
    pub prev_basic_touch_combo: u8,
    pub basic_touch_combo: u8,

    pub step_state: StepState,
    pub next_success_rng: f32,

    pub(super) was_primed: bool,
    pub(super) was_excellent: bool,
    pub(super) was_good_omen: bool,

    recipe_level: u32,
    base_progress: f64,
    base_quality: f64,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct BuffStacks {
    pub inner_quiet: u8,
    pub innovation: u8,
    pub great_strides: u8,
    pub muscle_memory: u8,
    pub veneration: u8,
    pub manipulation: u8,
    pub waste_not: u8,
    pub waste_not2: u8,
    pub final_appraisal: u8,
    pub expedience: u8,
    pub trained_perfection: u8,
}

impl CraftState {
    pub(super) fn increase_progress(&mut self, potency: u32, base_bonus: f64) {
        let mut bonus = base_bonus;
        if self.buffs.muscle_memory > 0 {
            bonus += 1.;
            self.buffs.muscle_memory = 0;
        }
        if self.buffs.veneration > 0 {
            bonus += 0.5;
        }

        let condition_bonus = {
            if self.step_state == StepState::Malleable {
                1.5
            } else {
                1.0
            }
        };

        let efficiency = potency as f64 * bonus;

        let progress_increase = (self.base_progress * condition_bonus * efficiency) / 100.;

        self.progress += progress_increase.floor() as u32;

        if self.buffs.final_appraisal > 0 && self.progress >= self.max_progress {
            self.progress = self.max_progress - 1;
            self.buffs.final_appraisal = 0;
        }
    }

    pub(super) fn increase_quality(&mut self, potency: u32, base_bonus: f64) {
        let mut bonus = base_bonus;
        bonus += (self.buffs.inner_quiet as f64) / 10.;

        let mut buff_bonus = 1.;
        if self.buffs.great_strides > 0 {
            buff_bonus += 1.;
            self.buffs.great_strides = 0;
        }
        if self.buffs.innovation > 0 {
            buff_bonus += 0.5;
        }

        let condition_bonus = {
            match self.step_state {
                StepState::Good => {
                    if self.gear_effects.splendorous {
                        1.75
                    } else {
                        1.5
                    }
                }
                StepState::Excellent => 4.0,
                StepState::Poor => 0.5,
                _ => 1.0,
            }
        };

        let efficiency = potency as f64 * bonus * buff_bonus;
        let quality_increase = (self.base_quality * condition_bonus * efficiency) / 100.;

        self.quality += quality_increase.floor() as u32;
    }

    /// set_next_state_outcome sets the success and/or StepState of the next step.
    /// if success_rng is less than the success threshold for the next action,
    /// step will succeed.
    /// Each step will automatically reset the next step to be successful with a
    /// NORMAL StepState.
    pub fn set_next_step_outcome(&mut self, success_rng: f32, state: StepState) {
        self.next_success_rng = success_rng;
        self.step_state = state;
    }

    pub(super) fn reset_rng(&mut self) {
        self.next_success_rng = 0.0;
        self.step_state = StepState::Normal;
    }

    /// Returns possible next states and their weights
    pub(super) fn possible_next_states(&self) -> (&[StepState], &[f32]) {
        if self.was_excellent {
            return (&[StepState::Poor], &[1.]);
        }
        if self.was_good_omen {
            return (&[StepState::Good], &[1.]);
        }
        match self.recipe_level {
            511 | 512 | 514 => (
                &[
                    StepState::Normal,
                    StepState::Good,
                    StepState::Centered,
                    StepState::Sturdy,
                    StepState::Pliant,
                ],
                &[0.46, 0.12, 0.15, 0.15, 0.12],
            ),
            513 | 515 => (
                &[
                    StepState::Normal,
                    StepState::Good,
                    StepState::Sturdy,
                    StepState::Pliant,
                    StepState::Malleable,
                    StepState::Primed,
                ],
                &[0.37, 0.12, 0.15, 0.12, 0.12, 0.12],
            ),
            516 => (
                &[
                    StepState::Normal,
                    StepState::Good,
                    StepState::Centered,
                    StepState::Sturdy,
                    StepState::Pliant,
                    StepState::Malleable,
                    StepState::Primed,
                ],
                &[0.22, 0.12, 0.15, 0.15, 0.12, 0.12, 0.12],
            ),
            611 => (
                &[
                    StepState::Normal,
                    StepState::Good,
                    StepState::Centered,
                    StepState::Sturdy,
                    StepState::Malleable,
                    StepState::Primed,
                ],
                &[0.34, 0.12, 0.15, 0.15, 0.12, 0.12],
            ),
            621 => (
                &[
                    StepState::Normal,
                    StepState::Good,
                    StepState::Centered,
                    StepState::Sturdy,
                    StepState::Pliant,
                    StepState::Malleable,
                    StepState::Primed,
                ],
                &[0.22, 0.12, 0.15, 0.15, 0.12, 0.12, 0.12],
            ),
            641 => (
                &[
                    StepState::Normal,
                    StepState::Good,
                    StepState::Sturdy,
                    StepState::Pliant,
                    StepState::Malleable,
                    StepState::Primed,
                    StepState::GoodOmen,
                ],
                &[0.33, 0.04, 0.15, 0.10, 0.115, 0.145, 0.12],
            ),
            _ => (
                &[StepState::Normal, StepState::Good, StepState::Excellent],
                &[0.71, 0.25, 0.04],
            ),
        }
    }

    pub(super) fn is_step_success(&self, threshold: f32) -> bool {
        let success_boost: f32 = {
            if self.step_state == StepState::Centered {
                0.25
            } else {
                0.0
            }
        };
        self.next_success_rng < (threshold + success_boost)
    }

    pub(super) fn get_durability_cost(&self, base_cost: u32) -> u32 {
        if self.buffs.trained_perfection > 0 {
            return 0;
        }
        let mut divider: f32 = 1.;
        if self.buffs.waste_not > 0 || self.buffs.waste_not2 > 0 {
            divider *= 2.;
        }
        if self.step_state == StepState::Sturdy {
            divider *= 2.;
        }
        (base_cost as f32 / divider).ceil() as u32
    }

    pub(super) fn get_cp_cost(&self, base_cost: u32) -> u32 {
        if self.step_state == StepState::Pliant {
            return (base_cost as f32 / 2.0).ceil() as u32;
        }
        base_cost
    }

    pub(super) fn get_buff_duration_bonus(&self) -> u8 {
        if self.was_primed {
            2
        } else {
            0
        }
    }

    pub fn step_num(&self) -> i64 {
        self.step_num as i64
    }

    pub fn is_finished(&self) -> bool {
        self.durability <= 0 || self.progress >= self.max_progress
    }

    /// get_valid_action_mask returns a bool mask of valid actions for the next
    /// step, where "true" means valid and the index is the integer representation
    /// of the action.
    pub fn get_valid_action_mask(&self) -> [bool; Action::NUM_ACTIONS] {
        Action::valid_action_mask(self)
    }

    /// play_action_no_validate makes no attempt to validate the action; it should
    /// be checked beforehand
    pub fn play_action_no_validate(&mut self, action: Action) {
        action.execute(self)
    }

    /// play_action returns true if the action was validated and executed. False
    /// otherwise.
    pub fn play_action(&mut self, action: Action) -> bool {
        let valid_action = action.validate(self);
        if valid_action {
            action.execute(self);
            return true;
        }
        false
    }

    // set_next_state_rng is a helper that randomly chooses a step state for the
    // next state.
    pub fn set_next_state_rng(&mut self) {
        let mut rng = rand::thread_rng();
        let (states, weights) = self.possible_next_states();
        let dist = WeightedIndex::new(weights).unwrap();
        let next_state = states[dist.sample(&mut rng)];
        self.set_next_step_outcome(rng.gen(), next_state);
    }
}

impl PartialEq for CraftState {
    fn eq(&self, other: &Self) -> bool {
        self.progress == other.progress
            && self.max_progress == other.max_progress
            && self.quality == other.quality
            && self.max_quality == other.max_quality
            && self.durability == other.durability
            && self.max_durability == other.max_durability
            && self.cp == other.cp
            && self.max_cp == other.max_cp
            && self.gear_effects == other.gear_effects
            && self.buffs == other.buffs
    }
}

impl Eq for CraftState {}

impl Hash for CraftState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.progress.hash(state);
        self.max_progress.hash(state);
        self.quality.hash(state);
        self.max_quality.hash(state);
        self.durability.hash(state);
        self.max_durability.hash(state);
        self.cp.hash(state);
        self.max_cp.hash(state);
        self.gear_effects.hash(state);
        self.buffs.hash(state);
    }
}
