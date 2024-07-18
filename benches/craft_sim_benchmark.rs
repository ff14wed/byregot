use byregot::simulator::{Action, CraftParams, CraftState};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

// Long-winded rotation
static ACTIONS_TO_EXECUTE: [Action; 17] = [
    Action::MuscleMemory,
    Action::Veneration,
    Action::Manipulation,
    Action::WasteNotII,
    Action::Groundwork,
    Action::Groundwork,
    Action::BasicTouch,
    Action::StandardTouch,
    Action::AdvancedTouch,
    Action::PreparatoryTouch,
    Action::PreparatoryTouch,
    Action::Innovation,
    Action::PreparatoryTouch,
    Action::PreparatoryTouch,
    Action::GreatStrides,
    Action::ByregotsBlessing,
    Action::BasicSynthesis,
];

fn full_craft_no_validate(craft_params: &CraftParams) {
    let mut craft_state = craft_params.new_craft();
    for a in &ACTIONS_TO_EXECUTE {
        craft_state.play_action_no_validate(black_box(*a));
    }
}

fn full_craft(craft_params: &CraftParams) {
    let mut craft_state = craft_params.new_craft();
    for a in &ACTIONS_TO_EXECUTE {
        craft_state.play_action(black_box(*a));
    }
}

fn get_valid_actions(craft_state: &CraftState) -> [bool; Action::NUM_ACTIONS] {
    craft_state.get_valid_action_mask()
}

fn set_next_state_rng(craft_state: &mut CraftState) {
    craft_state.set_next_state_rng()
}

fn bench_craft(c: &mut Criterion) {
    let craft_params = CraftParams {
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

    c.bench_function("full_craft_no_validate", |b| {
        b.iter(|| full_craft_no_validate(&craft_params))
    });

    c.bench_function("full_craft", |b| b.iter(|| full_craft(&craft_params)));

    let mut craft_state = craft_params.new_craft();

    c.bench_function("get_valid_actions1", |b| {
        b.iter(|| get_valid_actions(black_box(&craft_state)))
    });

    craft_state.play_action(Action::MuscleMemory);

    c.bench_function("get_valid_actions2", |b| {
        b.iter(|| get_valid_actions(black_box(&craft_state)))
    });

    c.bench_function("set_next_state_rng", |b| {
        b.iter(|| set_next_state_rng(black_box(&mut craft_state)))
    });
}

criterion_group!(benches, bench_craft);
criterion_main!(benches);
