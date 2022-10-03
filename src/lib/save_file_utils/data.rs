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
    pub error_cube: usize,
    pub blank_cores: usize,
    pub credits: usize,
    pub perk_points: usize,
    pub data_cells: usize,
    pub phazyonite: usize,
}