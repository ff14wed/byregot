use criterion::{black_box, criterion_group, criterion_main, Criterion};

use byregot::simulator;
use byregot::simulator::ActionID;
use byregot::simulator::NUM_ACTIONS;

static ACTIONS_TO_EXECUTE: [ActionID; 13] = [
  ActionID::MuscleMemory,
  ActionID::CarefulSynthesis,
  ActionID::CarefulSynthesis,
  ActionID::FinalAppraisal,
  ActionID::CarefulSynthesis,
  ActionID::InnerQuiet,
  ActionID::WasteNotII,
  ActionID::PreparatoryTouch,
  ActionID::PreparatoryTouch,
  ActionID::MastersMend,
  ActionID::MastersMend,
  ActionID::ByregotsBlessing,
  ActionID::CarefulSynthesis,
];

fn full_craft(craft_params: &simulator::CraftParams) {
  let mut craft_state = craft_params.new_craft();
  for a in &ACTIONS_TO_EXECUTE {
    craft_state.play_action(black_box(*a as usize));
  }
}

fn get_valid_actions(craft_state: &simulator::CraftState) -> [bool; NUM_ACTIONS] {
  craft_state.get_valid_action_mask()
}

fn set_next_state_rng(craft_state: &mut simulator::CraftState) {
  craft_state.set_next_state_rng()
}

fn bench_craft(c: &mut Criterion) {
  let craft_params = simulator::CraftParams {
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

  c.bench_function("full_craft", |b| {
    b.iter(|| full_craft(&craft_params))
  });

  let mut craft_state = craft_params.new_craft();

  c.bench_function("get_valid_actions1", |b| {
    b.iter(|| get_valid_actions(black_box(&craft_state)))
  });

  craft_state.play_action(ActionID::MuscleMemory as usize);

  c.bench_function("get_valid_actions2", |b| {
    b.iter(|| get_valid_actions(black_box(&craft_state)))
  });

  c.bench_function("set_next_state_rng", |b| {
    b.iter(|| set_next_state_rng(black_box(&mut craft_state)))
  });
}

criterion_group!(benches, bench_craft);
criterion_main!(benches);
