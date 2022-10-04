#[derive(Default)]
pub struct Resources {
    pub minerals: Minerals,
    pub brewing: Brewing,
    pub miscellaneous: Miscellaneous,
}
#[derive(Default)]
pub struct Minerals {
    pub bismor: usize,
    pub croppa: usize,
    pub enor_pearl: usize,
    pub jadiz: usize,
    pub magnite: usize,
    pub umanite: usize,
}
#[derive(Default)]
pub struct Brewing {
    pub barley_bulb: usize,
    pub malt_star: usize,
    pub starch_nut: usize,
    pub yeast_cone: usize,
}
#[derive(Default)]
pub struct Miscellaneous {
    pub error_cubes: usize,
    pub blank_cores: usize,
    pub credits: usize,
    pub perk_points: usize,
    pub data_cells: usize,
    pub phazyonite: usize,
}

#[derive(Default)]
pub struct Classes {
    pub rank: usize,
    pub title: String,
    pub driller: Class,
    pub engineer: Class,
    pub gunner: Class,
    pub scout: Class,
}
#[derive(Default)]
pub struct Class {
    pub xp_total: usize,
    pub level: usize,
    pub xp_prog: usize,
    pub promotion: String,
}
