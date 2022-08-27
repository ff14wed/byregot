static LEVEL_TABLE: &[u32; 40] = &[
    120, 125, 130, 133, 136, 139, 142, 145, 148, 150, 260, 265, 270, 273, 276, 279, 282, 285, 288,
    290, 390, 395, 400, 403, 406, 409, 412, 415, 418, 420, 517, 520, 525, 530, 535, 540, 545, 550,
    555, 560,
];

pub(super) fn get_crafter_level(job_level: u32) -> u32 {
    if job_level < 51 {
        return job_level;
    } else {
        return LEVEL_TABLE[(job_level - 51) as usize];
    }
}

pub(super) struct RecipeLevelModifiers(u8, u8, u8, u8);

impl RecipeLevelModifiers {
    pub fn progress_divider(&self) -> f32 {
        self.0 as f32
    }
    pub fn quality_divider(&self) -> f32 {
        self.1 as f32
    }
    pub fn progress_modifier(&self) -> f32 {
        self.2 as f32
    }
    pub fn quality_modifier(&self) -> f32 {
        self.3 as f32
    }
}

pub(super) static RECIPE_LEVEL_TABLE: &[RecipeLevelModifiers; 651] = &[
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 100, 100),
    RecipeLevelModifiers(50, 30, 80, 70),
    RecipeLevelModifiers(50, 30, 80, 70),
    RecipeLevelModifiers(50, 30, 80, 70),
    RecipeLevelModifiers(50, 30, 80, 70),
    RecipeLevelModifiers(50, 30, 80, 70),
    RecipeLevelModifiers(50, 30, 80, 70),
    RecipeLevelModifiers(50, 30, 80, 70),
    RecipeLevelModifiers(50, 30, 80, 70),
    RecipeLevelModifiers(50, 30, 80, 70),
    RecipeLevelModifiers(50, 30, 80, 70),
    RecipeLevelModifiers(50, 30, 80, 70),
    RecipeLevelModifiers(50, 30, 80, 70),
    RecipeLevelModifiers(50, 30, 80, 70),
    RecipeLevelModifiers(50, 30, 80, 70),
    RecipeLevelModifiers(50, 30, 80, 70),
    RecipeLevelModifiers(50, 30, 80, 70),
    RecipeLevelModifiers(50, 30, 80, 70),
    RecipeLevelModifiers(50, 30, 80, 70),
    RecipeLevelModifiers(50, 30, 80, 70),
    RecipeLevelModifiers(50, 30, 80, 70),
    RecipeLevelModifiers(50, 30, 80, 70),
    RecipeLevelModifiers(50, 30, 80, 70),
    RecipeLevelModifiers(50, 30, 80, 70),
    RecipeLevelModifiers(50, 30, 80, 70),
    RecipeLevelModifiers(50, 30, 80, 70),
    RecipeLevelModifiers(50, 30, 80, 70),
    RecipeLevelModifiers(50, 30, 80, 70),
    RecipeLevelModifiers(50, 30, 80, 70),
    RecipeLevelModifiers(50, 30, 80, 70),
    RecipeLevelModifiers(50, 30, 80, 70),
    RecipeLevelModifiers(50, 30, 80, 70),
    RecipeLevelModifiers(50, 30, 80, 70),
    RecipeLevelModifiers(50, 30, 80, 70),
    RecipeLevelModifiers(50, 30, 80, 70),
    RecipeLevelModifiers(50, 30, 80, 70),
    RecipeLevelModifiers(50, 30, 80, 70),
    RecipeLevelModifiers(50, 30, 80, 70),
    RecipeLevelModifiers(50, 30, 80, 70),
    RecipeLevelModifiers(50, 30, 80, 70),
    RecipeLevelModifiers(50, 30, 80, 70),
    RecipeLevelModifiers(50, 30, 80, 70),
    RecipeLevelModifiers(50, 30, 80, 70),
    RecipeLevelModifiers(50, 30, 80, 70),
    RecipeLevelModifiers(50, 30, 80, 70),
    RecipeLevelModifiers(50, 30, 80, 70),
    RecipeLevelModifiers(61, 39, 100, 100),
    RecipeLevelModifiers(61, 39, 100, 100),
    RecipeLevelModifiers(61, 39, 100, 100),
    RecipeLevelModifiers(61, 39, 100, 100),
    RecipeLevelModifiers(61, 39, 100, 100),
    RecipeLevelModifiers(61, 39, 100, 100),
    RecipeLevelModifiers(61, 39, 100, 100),
    RecipeLevelModifiers(61, 39, 100, 100),
    RecipeLevelModifiers(61, 39, 100, 100),
    RecipeLevelModifiers(61, 39, 100, 100),
    RecipeLevelModifiers(62, 43, 100, 100),
    RecipeLevelModifiers(62, 43, 100, 100),
    RecipeLevelModifiers(62, 43, 100, 100),
    RecipeLevelModifiers(62, 43, 100, 100),
    RecipeLevelModifiers(62, 43, 100, 100),
    RecipeLevelModifiers(63, 43, 100, 100),
    RecipeLevelModifiers(63, 43, 100, 100),
    RecipeLevelModifiers(63, 43, 100, 100),
    RecipeLevelModifiers(64, 44, 100, 100),
    RecipeLevelModifiers(64, 44, 100, 100),
    RecipeLevelModifiers(64, 44, 100, 100),
    RecipeLevelModifiers(65, 45, 100, 100),
    RecipeLevelModifiers(65, 45, 100, 100),
    RecipeLevelModifiers(65, 45, 100, 100),
    RecipeLevelModifiers(66, 46, 100, 100),
    RecipeLevelModifiers(66, 46, 100, 100),
    RecipeLevelModifiers(66, 46, 100, 100),
    RecipeLevelModifiers(67, 47, 100, 100),
    RecipeLevelModifiers(67, 47, 100, 100),
    RecipeLevelModifiers(67, 47, 100, 100),
    RecipeLevelModifiers(68, 48, 100, 100),
    RecipeLevelModifiers(68, 48, 100, 100),
    RecipeLevelModifiers(68, 48, 100, 100),
    RecipeLevelModifiers(69, 49, 100, 100),
    RecipeLevelModifiers(69, 49, 100, 100),
    RecipeLevelModifiers(70, 50, 100, 100),
    RecipeLevelModifiers(70, 50, 100, 100),
    RecipeLevelModifiers(70, 50, 100, 100),
    RecipeLevelModifiers(70, 50, 100, 100),
    RecipeLevelModifiers(70, 50, 100, 100),
    RecipeLevelModifiers(70, 50, 100, 100),
    RecipeLevelModifiers(70, 50, 100, 100),
    RecipeLevelModifiers(70, 50, 100, 100),
    RecipeLevelModifiers(70, 50, 100, 100),
    RecipeLevelModifiers(70, 50, 100, 100),
    RecipeLevelModifiers(70, 50, 100, 100),
    RecipeLevelModifiers(70, 50, 100, 100),
    RecipeLevelModifiers(70, 50, 100, 100),
    RecipeLevelModifiers(70, 50, 100, 100),
    RecipeLevelModifiers(70, 50, 100, 100),
    RecipeLevelModifiers(70, 50, 100, 100),
    RecipeLevelModifiers(70, 50, 100, 100),
    RecipeLevelModifiers(70, 50, 100, 100),
    RecipeLevelModifiers(70, 50, 100, 100),
    RecipeLevelModifiers(70, 50, 100, 100),
    RecipeLevelModifiers(70, 50, 100, 100),
    RecipeLevelModifiers(70, 50, 100, 100),
    RecipeLevelModifiers(70, 50, 100, 100),
    RecipeLevelModifiers(70, 50, 100, 100),
    RecipeLevelModifiers(70, 50, 100, 100),
    RecipeLevelModifiers(70, 50, 100, 100),
    RecipeLevelModifiers(70, 50, 100, 100),
    RecipeLevelModifiers(70, 50, 100, 100),
    RecipeLevelModifiers(70, 50, 100, 100),
    RecipeLevelModifiers(70, 50, 100, 100),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(70, 50, 80, 70),
    RecipeLevelModifiers(81, 58, 100, 100),
    RecipeLevelModifiers(81, 58, 100, 100),
    RecipeLevelModifiers(81, 58, 100, 100),
    RecipeLevelModifiers(81, 58, 100, 100),
    RecipeLevelModifiers(81, 58, 100, 100),
    RecipeLevelModifiers(81, 58, 100, 100),
    RecipeLevelModifiers(81, 58, 100, 100),
    RecipeLevelModifiers(81, 58, 100, 100),
    RecipeLevelModifiers(81, 58, 100, 100),
    RecipeLevelModifiers(81, 58, 100, 100),
    RecipeLevelModifiers(82, 61, 100, 100),
    RecipeLevelModifiers(82, 61, 100, 100),
    RecipeLevelModifiers(82, 61, 100, 100),
    RecipeLevelModifiers(82, 61, 100, 100),
    RecipeLevelModifiers(82, 61, 100, 100),
    RecipeLevelModifiers(83, 64, 100, 100),
    RecipeLevelModifiers(83, 64, 100, 100),
    RecipeLevelModifiers(83, 64, 100, 100),
    RecipeLevelModifiers(84, 65, 100, 100),
    RecipeLevelModifiers(84, 65, 100, 100),
    RecipeLevelModifiers(84, 65, 100, 100),
    RecipeLevelModifiers(85, 66, 100, 100),
    RecipeLevelModifiers(85, 66, 100, 100),
    RecipeLevelModifiers(85, 66, 100, 100),
    RecipeLevelModifiers(86, 66, 100, 100),
    RecipeLevelModifiers(86, 66, 100, 100),
    RecipeLevelModifiers(86, 66, 100, 100),
    RecipeLevelModifiers(87, 68, 100, 100),
    RecipeLevelModifiers(87, 68, 100, 100),
    RecipeLevelModifiers(87, 68, 100, 100),
    RecipeLevelModifiers(88, 68, 100, 100),
    RecipeLevelModifiers(88, 68, 100, 100),
    RecipeLevelModifiers(88, 68, 100, 100),
    RecipeLevelModifiers(89, 68, 100, 100),
    RecipeLevelModifiers(89, 68, 100, 100),
    RecipeLevelModifiers(90, 70, 100, 100),
    RecipeLevelModifiers(90, 70, 100, 100),
    RecipeLevelModifiers(90, 70, 100, 100),
    RecipeLevelModifiers(90, 70, 100, 100),
    RecipeLevelModifiers(90, 70, 100, 100),
    RecipeLevelModifiers(90, 70, 100, 100),
    RecipeLevelModifiers(90, 70, 100, 100),
    RecipeLevelModifiers(90, 70, 100, 100),
    RecipeLevelModifiers(90, 70, 100, 100),
    RecipeLevelModifiers(90, 70, 100, 100),
    RecipeLevelModifiers(90, 70, 100, 100),
    RecipeLevelModifiers(90, 70, 100, 100),
    RecipeLevelModifiers(90, 70, 100, 100),
    RecipeLevelModifiers(90, 70, 100, 100),
    RecipeLevelModifiers(90, 70, 100, 100),
    RecipeLevelModifiers(90, 70, 100, 100),
    RecipeLevelModifiers(90, 70, 100, 100),
    RecipeLevelModifiers(90, 70, 100, 100),
    RecipeLevelModifiers(90, 70, 100, 100),
    RecipeLevelModifiers(90, 70, 100, 100),
    RecipeLevelModifiers(90, 70, 100, 100),
    RecipeLevelModifiers(90, 70, 100, 100),
    RecipeLevelModifiers(90, 70, 100, 100),
    RecipeLevelModifiers(90, 70, 100, 100),
    RecipeLevelModifiers(90, 70, 100, 100),
    RecipeLevelModifiers(90, 70, 100, 100),
    RecipeLevelModifiers(90, 70, 100, 100),
    RecipeLevelModifiers(90, 70, 100, 100),
    RecipeLevelModifiers(90, 70, 100, 100),
    RecipeLevelModifiers(90, 70, 100, 100),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(90, 70, 80, 70),
    RecipeLevelModifiers(101, 81, 100, 100),
    RecipeLevelModifiers(101, 81, 100, 100),
    RecipeLevelModifiers(101, 81, 100, 100),
    RecipeLevelModifiers(101, 81, 100, 100),
    RecipeLevelModifiers(101, 81, 100, 100),
    RecipeLevelModifiers(101, 81, 100, 100),
    RecipeLevelModifiers(101, 81, 100, 100),
    RecipeLevelModifiers(101, 81, 100, 100),
    RecipeLevelModifiers(101, 81, 100, 100),
    RecipeLevelModifiers(101, 81, 100, 100),
    RecipeLevelModifiers(101, 81, 100, 100),
    RecipeLevelModifiers(101, 81, 100, 100),
    RecipeLevelModifiers(101, 81, 100, 100),
    RecipeLevelModifiers(101, 81, 100, 100),
    RecipeLevelModifiers(102, 82, 100, 100),
    RecipeLevelModifiers(102, 82, 100, 100),
    RecipeLevelModifiers(102, 82, 100, 100),
    RecipeLevelModifiers(102, 82, 100, 100),
    RecipeLevelModifiers(102, 82, 100, 100),
    RecipeLevelModifiers(103, 83, 100, 100),
    RecipeLevelModifiers(103, 83, 100, 100),
    RecipeLevelModifiers(103, 83, 100, 100),
    RecipeLevelModifiers(104, 84, 100, 100),
    RecipeLevelModifiers(104, 84, 100, 100),
    RecipeLevelModifiers(104, 84, 100, 100),
    RecipeLevelModifiers(105, 85, 100, 100),
    RecipeLevelModifiers(105, 85, 100, 100),
    RecipeLevelModifiers(105, 85, 100, 100),
    RecipeLevelModifiers(106, 86, 100, 100),
    RecipeLevelModifiers(106, 86, 100, 100),
    RecipeLevelModifiers(106, 86, 100, 100),
    RecipeLevelModifiers(107, 87, 100, 100),
    RecipeLevelModifiers(107, 87, 100, 100),
    RecipeLevelModifiers(107, 87, 100, 100),
    RecipeLevelModifiers(108, 88, 100, 100),
    RecipeLevelModifiers(108, 88, 100, 100),
    RecipeLevelModifiers(108, 88, 100, 100),
    RecipeLevelModifiers(109, 89, 100, 100),
    RecipeLevelModifiers(109, 89, 100, 100),
    RecipeLevelModifiers(109, 89, 100, 100),
    RecipeLevelModifiers(109, 89, 100, 100),
    RecipeLevelModifiers(109, 89, 100, 100),
    RecipeLevelModifiers(109, 89, 100, 100),
    RecipeLevelModifiers(109, 89, 100, 100),
    RecipeLevelModifiers(109, 89, 100, 100),
    RecipeLevelModifiers(109, 89, 100, 100),
    RecipeLevelModifiers(109, 89, 100, 100),
    RecipeLevelModifiers(109, 89, 100, 100),
    RecipeLevelModifiers(109, 89, 100, 100),
    RecipeLevelModifiers(110, 90, 100, 100),
    RecipeLevelModifiers(110, 90, 100, 100),
    RecipeLevelModifiers(110, 90, 100, 100),
    RecipeLevelModifiers(110, 90, 100, 100),
    RecipeLevelModifiers(110, 90, 100, 100),
    RecipeLevelModifiers(110, 90, 100, 100),
    RecipeLevelModifiers(110, 90, 100, 100),
    RecipeLevelModifiers(110, 90, 100, 100),
    RecipeLevelModifiers(110, 90, 100, 100),
    RecipeLevelModifiers(110, 90, 100, 100),
    RecipeLevelModifiers(110, 90, 100, 100),
    RecipeLevelModifiers(110, 90, 100, 100),
    RecipeLevelModifiers(110, 90, 100, 100),
    RecipeLevelModifiers(110, 90, 100, 100),
    RecipeLevelModifiers(110, 90, 100, 100),
    RecipeLevelModifiers(110, 90, 100, 100),
    RecipeLevelModifiers(110, 90, 100, 100),
    RecipeLevelModifiers(110, 90, 100, 100),
    RecipeLevelModifiers(110, 90, 100, 100),
    RecipeLevelModifiers(110, 90, 100, 100),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(140, 90, 100, 100),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(140, 130, 100, 100),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(110, 90, 80, 70),
    RecipeLevelModifiers(140, 130, 100, 100),
    RecipeLevelModifiers(140, 130, 100, 100),
    RecipeLevelModifiers(140, 130, 100, 100),
    RecipeLevelModifiers(140, 130, 100, 100),
    RecipeLevelModifiers(140, 130, 100, 100),
    RecipeLevelModifiers(140, 130, 100, 100),
    RecipeLevelModifiers(121, 105, 100, 100),
    RecipeLevelModifiers(121, 105, 100, 100),
    RecipeLevelModifiers(121, 105, 100, 100),
    RecipeLevelModifiers(122, 106, 100, 100),
    RecipeLevelModifiers(122, 106, 100, 100),
    RecipeLevelModifiers(122, 106, 100, 100),
    RecipeLevelModifiers(122, 106, 100, 100),
    RecipeLevelModifiers(122, 106, 100, 100),
    RecipeLevelModifiers(123, 107, 100, 100),
    RecipeLevelModifiers(123, 107, 100, 100),
    RecipeLevelModifiers(123, 107, 100, 100),
    RecipeLevelModifiers(123, 107, 100, 100),
    RecipeLevelModifiers(123, 107, 100, 100),
    RecipeLevelModifiers(124, 108, 100, 100),
    RecipeLevelModifiers(124, 108, 100, 100),
    RecipeLevelModifiers(124, 108, 100, 100),
    RecipeLevelModifiers(124, 108, 100, 100),
    RecipeLevelModifiers(124, 108, 100, 100),
    RecipeLevelModifiers(125, 109, 100, 100),
    RecipeLevelModifiers(125, 109, 100, 100),
    RecipeLevelModifiers(125, 109, 100, 100),
    RecipeLevelModifiers(125, 109, 100, 100),
    RecipeLevelModifiers(125, 109, 100, 100),
    RecipeLevelModifiers(126, 110, 100, 100),
    RecipeLevelModifiers(126, 110, 100, 100),
    RecipeLevelModifiers(126, 110, 100, 100),
    RecipeLevelModifiers(126, 110, 100, 100),
    RecipeLevelModifiers(126, 110, 100, 100),
    RecipeLevelModifiers(127, 111, 100, 100),
    RecipeLevelModifiers(127, 111, 100, 100),
    RecipeLevelModifiers(127, 111, 100, 100),
    RecipeLevelModifiers(127, 111, 100, 100),
    RecipeLevelModifiers(127, 111, 100, 100),
    RecipeLevelModifiers(128, 112, 100, 100),
    RecipeLevelModifiers(128, 112, 100, 100),
    RecipeLevelModifiers(128, 112, 100, 100),
    RecipeLevelModifiers(128, 112, 100, 100),
    RecipeLevelModifiers(128, 112, 100, 100),
    RecipeLevelModifiers(129, 113, 100, 100),
    RecipeLevelModifiers(129, 113, 100, 100),
    RecipeLevelModifiers(129, 113, 100, 100),
    RecipeLevelModifiers(129, 113, 100, 100),
    RecipeLevelModifiers(129, 113, 100, 100),
    RecipeLevelModifiers(130, 115, 90, 80),
    RecipeLevelModifiers(130, 115, 90, 80),
    RecipeLevelModifiers(130, 115, 90, 80),
    RecipeLevelModifiers(130, 115, 90, 80),
    RecipeLevelModifiers(130, 115, 90, 80),
    RecipeLevelModifiers(130, 115, 90, 80),
    RecipeLevelModifiers(130, 115, 90, 80),
    RecipeLevelModifiers(130, 115, 90, 80),
    RecipeLevelModifiers(130, 115, 90, 80),
    RecipeLevelModifiers(130, 115, 90, 80),
    RecipeLevelModifiers(130, 115, 90, 80),
    RecipeLevelModifiers(130, 115, 90, 80),
    RecipeLevelModifiers(130, 115, 90, 80),
    RecipeLevelModifiers(130, 115, 90, 80),
    RecipeLevelModifiers(130, 115, 90, 80),
    RecipeLevelModifiers(130, 115, 90, 80),
    RecipeLevelModifiers(130, 115, 90, 80),
    RecipeLevelModifiers(130, 115, 90, 80),
    RecipeLevelModifiers(130, 115, 90, 80),
    RecipeLevelModifiers(130, 115, 90, 80),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(180, 180, 100, 100),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
    RecipeLevelModifiers(130, 115, 80, 70),
];
