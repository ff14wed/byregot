use super::tables;
use std::hash::{Hash, Hasher};

use super::action::{get_valid_action_mask, Change, ACTIONS, NUM_ACTIONS};

use rand::distributions::WeightedIndex;
use rand::prelude::*;

pub const OBS_SIZE: usize = 22;

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
}

#[derive(Debug, Copy, Clone)]
struct QualityParams {
  control: u32,
  suggested_control: u32,
  quality_factor: f32,
}

impl QualityParams {
  fn get_base_quality(&self, iq_stacks: u8) -> f32 {
    let mut control = self.control as f32;
    if iq_stacks > 0 {
      control += (0.2 * ((iq_stacks - 1) as f32) * control).floor();
    }
    let control_ratio: f32 = (control + 10000.) / (self.suggested_control as f32 + 10000.);
    let control_multiplier: f32 = (control * 35.) / 100. + 35.;
    return control_ratio * control_multiplier * self.quality_factor;
  }
}

pub struct CraftParams {
  pub job_level: u32,
  pub craftsmanship: u32,
  pub control: u32,
  pub cp: u32,

  pub recipe_level: u32,
  pub suggested_craftsmanship: u32,
  pub suggested_control: u32,

  pub progress: u32,
  pub quality: u32,
  pub durability: u32,
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

      buffs: Default::default(),

      did_observe: false,
      prev_basic_touch_combo: 0,
      basic_touch_combo: 0,

      step_state: StepState::Normal,
      was_primed: false,
      next_success_rng: 0.0,

      recipe_level: self.recipe_level,
      base_progress: self.get_base_progress().floor(),
      quality_params: self.get_quality_params(),
    }
  }

  fn get_base_progress(&self) -> f32 {
    let crafter_level = tables::get_crafter_level(self.job_level);
    let cld = tables::get_craft_level_difference(self.recipe_level, crafter_level);
    let progress_factor = cld.progress_factor as f32 / 100.;

    let cms_ratio: f32 =
      (self.craftsmanship as f32 + 10000.) / (self.suggested_craftsmanship as f32 + 10000.);
    let cms_multiplier: f32 = (self.craftsmanship as f32 * 21.) / 100. + 2.;
    return cms_ratio * cms_multiplier * progress_factor;
  }

  fn get_quality_params(&self) -> QualityParams {
    let crafter_level = tables::get_crafter_level(self.job_level);
    let cld = tables::get_craft_level_difference(self.recipe_level, crafter_level);
    let quality_factor = cld.quality_factor as f32 / 100.;

    QualityParams {
      control: self.control,
      suggested_control: self.suggested_control,
      quality_factor: quality_factor,
    }
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

  pub buffs: BuffStacks,

  pub did_observe: bool,
  pub prev_basic_touch_combo: u8,
  pub basic_touch_combo: u8,

  pub step_state: StepState,
  pub was_primed: bool,
  pub next_success_rng: f32,

  recipe_level: u32,
  base_progress: f32,
  quality_params: QualityParams,
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
}

impl CraftState {
  pub(super) fn increase_progress(&mut self, potency: u32, base_bonus: f32) {
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

    bonus *= condition_bonus;

    self.progress += ((self.base_progress * potency as f32 * bonus) / 100.).floor() as u32;

    if self.buffs.final_appraisal > 0 && self.progress >= self.max_progress {
      self.progress = self.max_progress - 1;
      self.buffs.final_appraisal = 0;
    }
  }

  pub(super) fn increase_quality(&mut self, potency: u32, base_bonus: f32) {
    let base_quality = self.quality_params.get_base_quality(self.buffs.inner_quiet);
    let mut bonus = base_bonus;
    if self.buffs.great_strides > 0 {
      bonus += 1.;
      self.buffs.great_strides = 0;
    }
    if self.buffs.innovation > 0 {
      bonus += 0.5;
    }

    let condition_bonus = {
      match self.step_state {
        StepState::Good => 1.5,
        StepState::Excellent => 4.0,
        StepState::Poor => 0.5,
        _ => 1.0,
      }
    };

    let mod_quality = (base_quality * condition_bonus).floor();

    self.quality += ((mod_quality * potency as f32 * bonus) / 100.).floor() as u32;
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
    return self.next_success_rng < (threshold + success_boost);
  }

  pub(super) fn get_durability_cost(&self, base_cost: u32) -> u32 {
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
    return base_cost;
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
  /// step, where "true" means valid.
  pub fn get_valid_action_mask(&self) -> [bool; NUM_ACTIONS] {
    get_valid_action_mask(self)
  }

  /// play_action_no_validate makes no attempt to validate the action; it should
  /// be checked beforehand
  pub fn play_action_no_validate(&mut self, action_id: usize) {
    ACTIONS[action_id].execute(self)
  }

  /// play_action returns true if the action was validated and executed. False
  /// otherwise.
  pub fn play_action(&mut self, action_id: usize) -> bool {
    let valid_action = ACTIONS[action_id].validate(self);
    if valid_action {
      ACTIONS[action_id].execute(self);
      return true;
    }
    return false;
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
    self.buffs.hash(state);
  }
}
