use super::state;
use std::cmp;

use strum::{AsRefStr, EnumCount, EnumString, FromRepr, VariantArray};

pub(super) trait Change {
    fn execute(&self, state: &mut state::CraftState);
    fn validate(&self, _state: &state::CraftState) -> bool {
        true
    }
}

#[derive(Copy, Clone)]
struct ChangeSet<T: Change, U: Change>(T, U);

impl<T: Change, U: Change> Change for ChangeSet<T, U> {
    fn execute(&self, state: &mut state::CraftState) {
        self.0.execute(state);
        self.1.execute(state);
    }

    fn validate(&self, state: &state::CraftState) -> bool {
        self.0.validate(state) && self.1.validate(state)
    }
}

macro_rules! change_set_internal {
    ( $x:expr ) => { $x };
    ( $x:expr, $( $rest:expr ), + ) => {
       ChangeSet($x, change_set_internal!($( $rest ), + ) )
    };
}

macro_rules! change_set {
  ( $($x:tt)* ) => { Action(&change_set_internal!($($x)*)) };
}

pub struct Action(&'static dyn Change);

impl Change for Action {
    fn execute(&self, state: &mut state::CraftState) {
        self.0.execute(state);
    }
    fn validate(&self, state: &state::CraftState) -> bool {
        self.0.validate(state)
    }
}

#[derive(Copy, Clone)]
struct Step;
impl Change for Step {
    fn execute(&self, state: &mut state::CraftState) {
        state.step_num += 1;

        // Tick duration-based buffs
        if state.buffs.innovation > 0 {
            state.buffs.innovation -= 1;
        }
        if state.buffs.great_strides > 0 {
            state.buffs.great_strides -= 1;
        }
        if state.buffs.muscle_memory > 0 {
            state.buffs.muscle_memory -= 1;
        }
        if state.buffs.veneration > 0 {
            state.buffs.veneration -= 1;
        }
        if state.buffs.manipulation > 0 {
            state.buffs.manipulation -= 1;
            IncreaseDurability(5).execute(state);
        }
        if state.buffs.waste_not > 0 {
            state.buffs.waste_not -= 1;
        }
        if state.buffs.waste_not2 > 0 {
            state.buffs.waste_not2 -= 1;
        }
        if state.buffs.final_appraisal > 0 {
            state.buffs.final_appraisal -= 1;
        }
        if state.buffs.expedience > 0 {
            state.buffs.expedience -= 1;
        }

        state.did_observe = false;
        state.prev_basic_touch_combo = state.basic_touch_combo;
        state.basic_touch_combo = 0;

        state.was_excellent = state.step_state == state::StepState::Excellent;
        state.was_primed = state.step_state == state::StepState::Primed;
        state.was_good_omen = state.step_state == state::StepState::GoodOmen;

        state.reset_rng();

        if state.was_excellent {
            state.step_state = state::StepState::Poor;
        }
        if state.was_good_omen {
            state.step_state = state::StepState::Good;
        }
    }

    fn validate(&self, state: &state::CraftState) -> bool {
        !state.is_finished()
    }
}

struct IncreaseProgress(u32);
impl Change for IncreaseProgress {
    fn execute(&self, state: &mut state::CraftState) {
        state.increase_progress(self.0, 1.);
    }
}

struct IncreaseQuality(u32);
impl Change for IncreaseQuality {
    fn execute(&self, state: &mut state::CraftState) {
        state.increase_quality(self.0, 1.);
    }
}

struct ConditionalIncreaseProgress(u32, f32);
impl Change for ConditionalIncreaseProgress {
    fn execute(&self, state: &mut state::CraftState) {
        if state.is_step_success(self.1) {
            state.increase_progress(self.0, 1.);
        }
    }
}

struct ConditionalIncreaseQuality(u32, f32);
impl Change for ConditionalIncreaseQuality {
    fn execute(&self, state: &mut state::CraftState) {
        if state.is_step_success(self.1) {
            state.increase_quality(self.0, 1.);
        }
    }
}

struct CPCost(u32);
impl Change for CPCost {
    fn execute(&self, state: &mut state::CraftState) {
        state.cp -= state.get_cp_cost(self.0);
    }

    fn validate(&self, state: &state::CraftState) -> bool {
        state.cp >= state.get_cp_cost(self.0)
    }
}

struct DurabilityCost(u32);
impl Change for DurabilityCost {
    fn execute(&self, state: &mut state::CraftState) {
        let cost = state.get_durability_cost(self.0) as i32;
        if state.buffs.trained_perfection > 0 && cost > 0 {
            state.buffs.trained_perfection = 0;
            return;
        }
        state.durability -= cost;
    }
}

struct IncreaseDurability(i32);
impl Change for IncreaseDurability {
    fn execute(&self, state: &mut state::CraftState) {
        if !state.is_finished() {
            state.durability = cmp::min(state.max_durability as i32, state.durability + self.0);
        }
    }
}

struct IncreaseInnerQuiet(u8);
impl Change for IncreaseInnerQuiet {
    fn execute(&self, state: &mut state::CraftState) {
        state.buffs.inner_quiet = cmp::min(10, state.buffs.inner_quiet + self.0);
    }
}

struct ConditionalIncreaseInnerQuiet(u8, f32);
impl Change for ConditionalIncreaseInnerQuiet {
    fn execute(&self, state: &mut state::CraftState) {
        if state.is_step_success(self.1) {
            state.buffs.inner_quiet = cmp::min(10, state.buffs.inner_quiet + self.0);
        }
    }
}

struct BasicTouchCombo;
impl Change for BasicTouchCombo {
    fn execute(&self, state: &mut state::CraftState) {
        state.basic_touch_combo = 1;
    }
}

struct StandardTouchCPCost;
impl Change for StandardTouchCPCost {
    fn execute(&self, state: &mut state::CraftState) {
        if state.basic_touch_combo == 1 {
            state.cp -= 18;
        } else {
            state.cp -= 32;
        }
    }

    fn validate(&self, state: &state::CraftState) -> bool {
        if state.basic_touch_combo == 1 {
            state.cp >= 18
        } else {
            state.cp >= 32
        }
    }
}

struct StandardTouchCombo;
impl Change for StandardTouchCombo {
    fn execute(&self, state: &mut state::CraftState) {
        if state.prev_basic_touch_combo == 1 {
            state.basic_touch_combo = 2;
        }
    }
}

struct AdvancedTouchCPCost;
impl Change for AdvancedTouchCPCost {
    fn execute(&self, state: &mut state::CraftState) {
        if state.basic_touch_combo == 2 || state.did_observe {
            state.cp -= 18;
        } else {
            state.cp -= 46;
        }
    }

    fn validate(&self, state: &state::CraftState) -> bool {
        if state.basic_touch_combo == 2 {
            state.cp >= 18
        } else {
            state.cp >= 46
        }
    }
}

struct Observe;
impl Change for Observe {
    fn execute(&self, state: &mut state::CraftState) {
        state.did_observe = true;
    }
}

struct TricksOfTheTrade;
impl Change for TricksOfTheTrade {
    fn execute(&self, state: &mut state::CraftState) {
        state.cp = cmp::min(state.cp + 20, state.max_cp);
    }
}

struct GoodOrExcellentRequirement;
impl Change for GoodOrExcellentRequirement {
    fn execute(&self, _state: &mut state::CraftState) {}
    fn validate(&self, state: &state::CraftState) -> bool {
        (state.step_state == state::StepState::Good)
            || (state.step_state == state::StepState::Excellent)
    }
}

struct WasteNot;
impl Change for WasteNot {
    fn execute(&self, state: &mut state::CraftState) {
        state.buffs.waste_not = 4 + state.get_buff_duration_bonus();
        state.buffs.waste_not2 = 0;
    }
}

struct Veneration;
impl Change for Veneration {
    fn execute(&self, state: &mut state::CraftState) {
        state.buffs.veneration = 4 + state.get_buff_duration_bonus();
    }
}

struct GreatStrides;
impl Change for GreatStrides {
    fn execute(&self, state: &mut state::CraftState) {
        state.buffs.great_strides = 3 + state.get_buff_duration_bonus();
    }
}

struct Innovation;
impl Change for Innovation {
    fn execute(&self, state: &mut state::CraftState) {
        state.buffs.innovation = 4 + state.get_buff_duration_bonus();
    }
}

struct FinalAppraisal;
impl Change for FinalAppraisal {
    fn execute(&self, state: &mut state::CraftState) {
        state.buffs.final_appraisal = 5 + {
            if state.step_state == state::StepState::Primed {
                2
            } else {
                0
            }
        };
        // Final Appraisal breaks combos
        state.did_observe = false;
        state.basic_touch_combo = 0;
    }
    fn validate(&self, state: &state::CraftState) -> bool {
        state.buffs.final_appraisal == 0 && !state.is_finished()
    }
}

struct WasteNotII;
impl Change for WasteNotII {
    fn execute(&self, state: &mut state::CraftState) {
        state.buffs.waste_not = 0;
        state.buffs.waste_not2 = 8 + state.get_buff_duration_bonus();
    }
}

struct ByregotsBlessing;
impl Change for ByregotsBlessing {
    fn execute(&self, state: &mut state::CraftState) {
        let potency = cmp::min(100 + 20 * state.buffs.inner_quiet as u32, 300);
        state.increase_quality(potency, 1.);
        state.buffs.inner_quiet = 0;
    }
    fn validate(&self, state: &state::CraftState) -> bool {
        state.buffs.inner_quiet > 0
    }
}

struct MuscleMemory;
impl Change for MuscleMemory {
    fn execute(&self, state: &mut state::CraftState) {
        state.buffs.muscle_memory = 5;
        state.step_num += 1;
    }
    fn validate(&self, state: &state::CraftState) -> bool {
        state.step_num == 0 && !state.is_finished()
    }
}

struct Manipulation;
impl Change for Manipulation {
    fn execute(&self, state: &mut state::CraftState) {
        state.buffs.manipulation = 0;
        Step.execute(state);
        state.buffs.manipulation = 8 + state.get_buff_duration_bonus();
    }
    fn validate(&self, state: &state::CraftState) -> bool {
        !state.is_finished()
    }
}

struct PrudentRequirement;
impl Change for PrudentRequirement {
    fn execute(&self, _state: &mut state::CraftState) {}
    fn validate(&self, state: &state::CraftState) -> bool {
        state.buffs.waste_not == 0 && state.buffs.waste_not2 == 0
    }
}

struct Reflect;
impl Change for Reflect {
    fn execute(&self, state: &mut state::CraftState) {
        state.buffs.inner_quiet = 2;
        state.step_num += 1;
    }
    fn validate(&self, state: &state::CraftState) -> bool {
        state.step_num == 0 && !state.is_finished()
    }
}

struct Groundwork;
impl Change for Groundwork {
    fn execute(&self, state: &mut state::CraftState) {
        if state.durability < state.get_durability_cost(20) as i32 {
            state.increase_progress(180, 1.);
        } else {
            state.increase_progress(360, 1.);
        }
    }
}

struct TrainedFinesseRequirement;
impl Change for TrainedFinesseRequirement {
    fn execute(&self, _state: &mut state::CraftState) {}
    fn validate(&self, state: &state::CraftState) -> bool {
        state.buffs.inner_quiet == 10
    }
}

struct RefinedTouchConditionalInnerQuiet;
impl Change for RefinedTouchConditionalInnerQuiet {
    fn execute(&self, state: &mut state::CraftState) {
        if state.basic_touch_combo == 1 {
            state.buffs.inner_quiet += 2;
        } else {
            state.buffs.inner_quiet += 1;
        }
    }
}

struct Expedience;
impl Change for Expedience {
    fn execute(&self, state: &mut state::CraftState) {
        // Buff duration bonuses do not apply to Expedience
        state.buffs.expedience = 1;
    }
}

struct ExpedienceRequirement;
impl Change for ExpedienceRequirement {
    fn execute(&self, _state: &mut state::CraftState) {}
    fn validate(&self, state: &state::CraftState) -> bool {
        state.buffs.expedience == 1
    }
}

struct TrainedPerfection;
impl Change for TrainedPerfection {
    fn execute(&self, state: &mut state::CraftState) {
        state.did_trained_perfection = true;
        state.buffs.trained_perfection = 1;
    }

    fn validate(&self, state: &state::CraftState) -> bool {
        !state.did_trained_perfection
    }
}

#[derive(Debug, Copy, Clone, FromRepr, AsRefStr, EnumString, EnumCount, VariantArray)]
pub enum Actions {
    #[strum(serialize = "Basic Synthesis")]
    BasicSynthesis,
    #[strum(serialize = "Basic Touch")]
    BasicTouch,
    #[strum(serialize = "Master's Mend")]
    MastersMend,
    #[strum(serialize = "Hasty Touch")]
    HastyTouch,
    #[strum(serialize = "Rapid Synthesis")]
    RapidSynthesis,
    #[strum(serialize = "Observe")]
    Observe,
    #[strum(serialize = "Tricks of the Trade")]
    TricksOfTheTrade,
    #[strum(serialize = "Waste Not")]
    WasteNot,
    #[strum(serialize = "Veneration")]
    Veneration,
    #[strum(serialize = "Standard Touch")]
    StandardTouch,
    #[strum(serialize = "Great Strides")]
    GreatStrides,
    #[strum(serialize = "Innovation")]
    Innovation,
    #[strum(serialize = "Final Appraisal")]
    FinalAppraisal,
    #[strum(serialize = "Waste Not II")]
    WasteNotII,
    #[strum(serialize = "Byregot's Blessing")]
    ByregotsBlessing,
    #[strum(serialize = "Precise Touch")]
    PreciseTouch,
    #[strum(serialize = "Muscle Memory")]
    MuscleMemory,
    #[strum(serialize = "Careful Synthesis")]
    CarefulSynthesis,
    #[strum(serialize = "Manipulation")]
    Manipulation,
    #[strum(serialize = "Prudent Touch")]
    PrudentTouch,
    #[strum(serialize = "Reflect")]
    Reflect,
    #[strum(serialize = "Preparatory Touch")]
    PreparatoryTouch,
    #[strum(serialize = "Groundwork")]
    Groundwork,
    #[strum(serialize = "Delicate Synthesis")]
    DelicateSynthesis,
    #[strum(serialize = "Intensive Synthesis")]
    IntensiveSynthesis,
    #[strum(serialize = "Advanced Touch")]
    AdvancedTouch,
    #[strum(serialize = "Prudent Synthesis")]
    PrudentSynthesis,
    #[strum(serialize = "Trained Finesse")]
    TrainedFinesse,
    #[strum(serialize = "Refined Touch")]
    RefinedTouch,
    #[strum(serialize = "Daring Touch")]
    DaringTouch,
    #[strum(serialize = "Immaculate Mend")]
    ImmaculateMend,
    #[strum(serialize = "Trained Perfection")]
    TrainedPerfection,
}

impl Actions {
    pub(super) const fn get(&self) -> Action {
        match *self {
            Self::BasicSynthesis => {
                change_set!(CPCost(0), IncreaseProgress(120), DurabilityCost(10), Step)
            }
            Self::BasicTouch => change_set!(
                CPCost(18),
                IncreaseQuality(100),
                IncreaseInnerQuiet(1),
                DurabilityCost(10),
                Step,
                BasicTouchCombo
            ),
            Self::MastersMend => change_set!(CPCost(88), IncreaseDurability(30), Step),
            Self::HastyTouch => change_set!(
                CPCost(0),
                ConditionalIncreaseQuality(100, 0.6),
                ConditionalIncreaseInnerQuiet(1, 0.6),
                DurabilityCost(10),
                Step,
                Expedience
            ),
            Self::RapidSynthesis => change_set!(
                CPCost(0),
                ConditionalIncreaseProgress(500, 0.5),
                DurabilityCost(10),
                Step
            ),
            Self::Observe => change_set!(CPCost(7), Step, Observe),
            Self::TricksOfTheTrade => {
                change_set!(GoodOrExcellentRequirement, TricksOfTheTrade, Step)
            }
            Self::WasteNot => change_set!(CPCost(56), Step, WasteNot),
            Self::Veneration => change_set!(CPCost(18), Step, Veneration),
            Self::StandardTouch => change_set!(
                StandardTouchCPCost,
                IncreaseQuality(125),
                IncreaseInnerQuiet(1),
                DurabilityCost(10),
                Step,
                StandardTouchCombo
            ),
            Self::GreatStrides => change_set!(CPCost(32), Step, GreatStrides),
            Self::Innovation => change_set!(CPCost(18), Step, Innovation),
            Self::FinalAppraisal => change_set!(CPCost(1), FinalAppraisal),
            Self::WasteNotII => change_set!(CPCost(98), Step, WasteNotII),
            Self::ByregotsBlessing => {
                change_set!(CPCost(24), ByregotsBlessing, DurabilityCost(10), Step)
            }
            Self::PreciseTouch => change_set!(
                CPCost(18),
                GoodOrExcellentRequirement,
                IncreaseQuality(150),
                IncreaseInnerQuiet(2),
                DurabilityCost(10),
                Step
            ),
            Self::MuscleMemory => change_set!(
                CPCost(6),
                IncreaseProgress(300),
                DurabilityCost(10),
                MuscleMemory
            ),
            Self::CarefulSynthesis => {
                change_set!(CPCost(7), IncreaseProgress(180), DurabilityCost(10), Step)
            }
            Self::Manipulation => change_set!(CPCost(96), Manipulation),
            Self::PrudentTouch => change_set!(
                CPCost(25),
                PrudentRequirement,
                IncreaseQuality(100),
                IncreaseInnerQuiet(1),
                DurabilityCost(5),
                Step
            ),
            Self::Reflect => {
                change_set!(CPCost(6), IncreaseQuality(100), DurabilityCost(10), Reflect)
            }
            Self::PreparatoryTouch => change_set!(
                CPCost(40),
                IncreaseQuality(200),
                IncreaseInnerQuiet(2),
                DurabilityCost(20),
                Step
            ),
            Self::Groundwork => change_set!(CPCost(18), Groundwork, DurabilityCost(20), Step),
            Self::DelicateSynthesis => change_set!(
                CPCost(32),
                IncreaseProgress(100),
                IncreaseQuality(100),
                IncreaseInnerQuiet(1),
                DurabilityCost(10),
                Step
            ),
            Self::IntensiveSynthesis => change_set!(
                CPCost(6),
                GoodOrExcellentRequirement,
                IncreaseProgress(400),
                DurabilityCost(10),
                Step
            ),
            Self::AdvancedTouch => change_set!(
                AdvancedTouchCPCost,
                IncreaseQuality(150),
                IncreaseInnerQuiet(1),
                DurabilityCost(10),
                Step
            ),
            Self::PrudentSynthesis => change_set!(
                CPCost(18),
                PrudentRequirement,
                IncreaseProgress(180),
                DurabilityCost(5),
                Step
            ),
            Self::TrainedFinesse => change_set!(
                CPCost(32),
                TrainedFinesseRequirement,
                IncreaseQuality(100),
                DurabilityCost(0),
                Step
            ),
            Self::RefinedTouch => change_set!(
                CPCost(18),
                IncreaseQuality(100),
                RefinedTouchConditionalInnerQuiet,
                DurabilityCost(10),
                Step
            ),
            Self::DaringTouch => change_set!(
                CPCost(18),
                ExpedienceRequirement,
                ConditionalIncreaseQuality(150, 0.6),
                ConditionalIncreaseInnerQuiet(1, 0.6),
                DurabilityCost(10),
                Step
            ),
            Self::ImmaculateMend => change_set!(CPCost(112), IncreaseDurability(80), Step),
            Self::TrainedPerfection => change_set!(CPCost(0), Step, TrainedPerfection),
        }
    }

    pub(super) fn valid_action_mask(craft_state: &state::CraftState) -> [bool; Self::COUNT] {
        let mut mask: [bool; Self::COUNT] = Default::default();
        let actions1 = &Self::VARIANTS[0..16];
        let actions2 = &Self::VARIANTS[16..];

        for (i, action) in actions1.iter().enumerate() {
            mask[i] = action.get().validate(craft_state);
        }
        for (i, action) in actions2.iter().enumerate() {
            mask[15 + i] = action.get().validate(craft_state);
        }
        mask
    }
}
