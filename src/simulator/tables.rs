use std::cmp;

static MASTER_LEVELS: &[u32; 30] = &[
    120, 125, 130, 133, 136, 139, 142, 145, 148, 150, 260, 265, 270, 273, 276, 279, 282, 285, 288,
    290, 390, 395, 400, 403, 406, 409, 412, 415, 418, 420,
];

pub(super) fn get_crafter_level(job_level: u32) -> u32 {
    if job_level < 51 {
        return job_level;
    } else {
        return MASTER_LEVELS[(job_level - 51) as usize];
    }
}

pub(super) struct CraftLevelDifference {
    pub(super) progress_factor: u32,
    pub(super) quality_factor: u32,
}

pub(super) fn get_craft_level_difference(
    recipe_level: u32,
    crafter_level: u32,
) -> CraftLevelDifference {
    let mut level_difference: i32 = crafter_level as i32 - recipe_level as i32;
    level_difference = cmp::min(49, cmp::max(-30, level_difference));
    match level_difference {
        -30 => CraftLevelDifference {
            progress_factor: 80,
            quality_factor: 60,
        },
        -29 => CraftLevelDifference {
            progress_factor: 82,
            quality_factor: 64,
        },
        -28 => CraftLevelDifference {
            progress_factor: 84,
            quality_factor: 68,
        },
        -27 => CraftLevelDifference {
            progress_factor: 86,
            quality_factor: 72,
        },
        -26 => CraftLevelDifference {
            progress_factor: 88,
            quality_factor: 76,
        },
        -25 => CraftLevelDifference {
            progress_factor: 90,
            quality_factor: 80,
        },
        -24 => CraftLevelDifference {
            progress_factor: 92,
            quality_factor: 84,
        },
        -23 => CraftLevelDifference {
            progress_factor: 94,
            quality_factor: 88,
        },
        -22 => CraftLevelDifference {
            progress_factor: 96,
            quality_factor: 92,
        },
        -21 => CraftLevelDifference {
            progress_factor: 98,
            quality_factor: 96,
        },
        -20 => CraftLevelDifference {
            progress_factor: 100,
            quality_factor: 100,
        },
        -19 => CraftLevelDifference {
            progress_factor: 100,
            quality_factor: 100,
        },
        -18 => CraftLevelDifference {
            progress_factor: 100,
            quality_factor: 100,
        },
        -17 => CraftLevelDifference {
            progress_factor: 100,
            quality_factor: 100,
        },
        -16 => CraftLevelDifference {
            progress_factor: 100,
            quality_factor: 100,
        },
        -15 => CraftLevelDifference {
            progress_factor: 100,
            quality_factor: 100,
        },
        -14 => CraftLevelDifference {
            progress_factor: 100,
            quality_factor: 100,
        },
        -13 => CraftLevelDifference {
            progress_factor: 100,
            quality_factor: 100,
        },
        -12 => CraftLevelDifference {
            progress_factor: 100,
            quality_factor: 100,
        },
        -11 => CraftLevelDifference {
            progress_factor: 100,
            quality_factor: 100,
        },
        -10 => CraftLevelDifference {
            progress_factor: 100,
            quality_factor: 100,
        },
        -9 => CraftLevelDifference {
            progress_factor: 100,
            quality_factor: 100,
        },
        -8 => CraftLevelDifference {
            progress_factor: 100,
            quality_factor: 100,
        },
        -7 => CraftLevelDifference {
            progress_factor: 100,
            quality_factor: 100,
        },
        -6 => CraftLevelDifference {
            progress_factor: 100,
            quality_factor: 100,
        },
        -5 => CraftLevelDifference {
            progress_factor: 100,
            quality_factor: 100,
        },
        -4 => CraftLevelDifference {
            progress_factor: 100,
            quality_factor: 100,
        },
        -3 => CraftLevelDifference {
            progress_factor: 100,
            quality_factor: 100,
        },
        -2 => CraftLevelDifference {
            progress_factor: 100,
            quality_factor: 100,
        },
        -1 => CraftLevelDifference {
            progress_factor: 100,
            quality_factor: 100,
        },
        0 => CraftLevelDifference {
            progress_factor: 100,
            quality_factor: 100,
        },
        1 => CraftLevelDifference {
            progress_factor: 105,
            quality_factor: 100,
        },
        2 => CraftLevelDifference {
            progress_factor: 110,
            quality_factor: 100,
        },
        3 => CraftLevelDifference {
            progress_factor: 115,
            quality_factor: 100,
        },
        4 => CraftLevelDifference {
            progress_factor: 120,
            quality_factor: 100,
        },
        5 => CraftLevelDifference {
            progress_factor: 125,
            quality_factor: 100,
        },
        6 => CraftLevelDifference {
            progress_factor: 127,
            quality_factor: 100,
        },
        7 => CraftLevelDifference {
            progress_factor: 129,
            quality_factor: 100,
        },
        8 => CraftLevelDifference {
            progress_factor: 131,
            quality_factor: 100,
        },
        9 => CraftLevelDifference {
            progress_factor: 133,
            quality_factor: 100,
        },
        10 => CraftLevelDifference {
            progress_factor: 135,
            quality_factor: 100,
        },
        11 => CraftLevelDifference {
            progress_factor: 137,
            quality_factor: 100,
        },
        12 => CraftLevelDifference {
            progress_factor: 139,
            quality_factor: 100,
        },
        13 => CraftLevelDifference {
            progress_factor: 141,
            quality_factor: 100,
        },
        14 => CraftLevelDifference {
            progress_factor: 143,
            quality_factor: 100,
        },
        15 => CraftLevelDifference {
            progress_factor: 145,
            quality_factor: 100,
        },
        16 => CraftLevelDifference {
            progress_factor: 147,
            quality_factor: 100,
        },
        17 => CraftLevelDifference {
            progress_factor: 147,
            quality_factor: 100,
        },
        18 => CraftLevelDifference {
            progress_factor: 148,
            quality_factor: 100,
        },
        19 => CraftLevelDifference {
            progress_factor: 149,
            quality_factor: 100,
        },
        20 => CraftLevelDifference {
            progress_factor: 150,
            quality_factor: 100,
        },
        21 => CraftLevelDifference {
            progress_factor: 150,
            quality_factor: 100,
        },
        22 => CraftLevelDifference {
            progress_factor: 150,
            quality_factor: 100,
        },
        23 => CraftLevelDifference {
            progress_factor: 150,
            quality_factor: 100,
        },
        24 => CraftLevelDifference {
            progress_factor: 150,
            quality_factor: 100,
        },
        25 => CraftLevelDifference {
            progress_factor: 150,
            quality_factor: 100,
        },
        26 => CraftLevelDifference {
            progress_factor: 150,
            quality_factor: 100,
        },
        27 => CraftLevelDifference {
            progress_factor: 150,
            quality_factor: 100,
        },
        28 => CraftLevelDifference {
            progress_factor: 150,
            quality_factor: 100,
        },
        29 => CraftLevelDifference {
            progress_factor: 150,
            quality_factor: 100,
        },
        30 => CraftLevelDifference {
            progress_factor: 150,
            quality_factor: 100,
        },
        31 => CraftLevelDifference {
            progress_factor: 150,
            quality_factor: 100,
        },
        32 => CraftLevelDifference {
            progress_factor: 150,
            quality_factor: 100,
        },
        33 => CraftLevelDifference {
            progress_factor: 150,
            quality_factor: 100,
        },
        34 => CraftLevelDifference {
            progress_factor: 150,
            quality_factor: 100,
        },
        35 => CraftLevelDifference {
            progress_factor: 150,
            quality_factor: 100,
        },
        36 => CraftLevelDifference {
            progress_factor: 150,
            quality_factor: 100,
        },
        37 => CraftLevelDifference {
            progress_factor: 150,
            quality_factor: 100,
        },
        38 => CraftLevelDifference {
            progress_factor: 150,
            quality_factor: 100,
        },
        39 => CraftLevelDifference {
            progress_factor: 150,
            quality_factor: 100,
        },
        40 => CraftLevelDifference {
            progress_factor: 150,
            quality_factor: 100,
        },
        41 => CraftLevelDifference {
            progress_factor: 150,
            quality_factor: 100,
        },
        42 => CraftLevelDifference {
            progress_factor: 150,
            quality_factor: 100,
        },
        43 => CraftLevelDifference {
            progress_factor: 150,
            quality_factor: 100,
        },
        44 => CraftLevelDifference {
            progress_factor: 150,
            quality_factor: 100,
        },
        45 => CraftLevelDifference {
            progress_factor: 150,
            quality_factor: 100,
        },
        46 => CraftLevelDifference {
            progress_factor: 150,
            quality_factor: 100,
        },
        47 => CraftLevelDifference {
            progress_factor: 150,
            quality_factor: 100,
        },
        48 => CraftLevelDifference {
            progress_factor: 150,
            quality_factor: 100,
        },
        49 => CraftLevelDifference {
            progress_factor: 150,
            quality_factor: 100,
        },
        _ => CraftLevelDifference {
            progress_factor: 0,
            quality_factor: 0,
        },
    }
}
