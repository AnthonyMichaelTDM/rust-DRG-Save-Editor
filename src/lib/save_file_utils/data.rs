use crate::lib::widgets::val_text::{ValText, self};

#[derive(Default)]
pub struct Resources {
    pub minerals: Minerals,
    pub brewing: Brewing,
    pub miscellaneous: Miscellaneous,
}

pub struct Minerals {
    pub bismor: ValText<usize>,
    pub croppa: ValText<usize>,
    pub enor_pearl: ValText<usize>,
    pub jadiz: ValText<usize>,
    pub magnite: ValText<usize>,
    pub umanite: ValText<usize>,
}
impl Default for Minerals {
    fn default() -> Self {
        Self { 
            bismor: val_text::number_box_validator(),
            croppa: val_text::number_box_validator(),
            enor_pearl: val_text::number_box_validator(),
            jadiz: val_text::number_box_validator(),
            magnite: val_text::number_box_validator(),
            umanite: val_text::number_box_validator(),
        }
    }
}

pub struct Brewing {
    pub barley_bulb: ValText<usize>,
    pub malt_star: ValText<usize>,
    pub starch_nut: ValText<usize>,
    pub yeast_cone: ValText<usize>,
}
impl Default for Brewing {
    fn default() -> Self {
        Self {
            barley_bulb: val_text::number_box_validator(),
            malt_star: val_text::number_box_validator(),
            starch_nut: val_text::number_box_validator(),
            yeast_cone: val_text::number_box_validator(),
        }
    }
}

pub struct Miscellaneous {
    pub error_cube: ValText<usize>,
    pub blank_cores: ValText<usize>,
    pub credits: ValText<usize>,
    pub perk_points: ValText<usize>,
    pub data_cells: ValText<usize>,
    pub phazyonite: ValText<usize>,
}
impl Default for Miscellaneous {
    fn default() -> Self {
        Self {
            error_cube: val_text::number_box_validator(),
            blank_cores: val_text::number_box_validator(),
            credits: val_text::number_box_validator(),
            perk_points: val_text::number_box_validator(),
            data_cells: val_text::number_box_validator(),
            phazyonite: val_text::number_box_validator(),
        }
    }
}