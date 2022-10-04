use cached::proc_macro::once;
use serde::{Serialize, Deserialize};

#[once]
/// deserializes weapon-overclocks.toml into a nice struct
pub fn get_weapon_overclocks() -> DeserializedWeaponOverclockTOML {
    let mut path = project_root::get_project_root().expect("Error Obtaining Project Root").join("res");
    path = path.join("weapon-overclocks.toml");
    toml::from_str( std::fs::read_to_string(&path).expect("Error finding weapon-overclocks.toml").as_str()).expect("Error deserializing weapon-overclocks.toml")
}

#[derive(Serialize,Deserialize, Clone)]
pub struct DeserializedWeaponOverclockTOML {
    pub driller: Class,
    pub engineer: Class,
    pub gunner: Class,
    pub scout: Class,
}
#[derive(Serialize,Deserialize, Clone)]
pub struct Class {
    pub name: String,
    pub weapons: Vec<Weapon>,
}

#[derive(Serialize,Deserialize, Clone)]
pub struct Weapon {
    pub name: String,
    pub overclocks: Vec<Overclock>,
}

#[derive(Serialize,Deserialize, Clone)]
pub struct Overclock {
    //information given by TOML
    pub name: String,
    pub guid: String,
    pub cost: Cost,

    //other information
    #[serde(skip)]
    pub selected: bool,
    #[serde(skip)]
    status: OcStatus,
}
impl Overclock {
    pub fn status(&self) -> String {
        self.status.string()
    }
}

#[derive(Serialize,Deserialize, Clone)]
pub struct Cost {
    pub credits: usize,
    pub bismor: usize,
    pub croppa: usize,
    pub enor: usize,
    pub jadiz: usize,
    pub magnite: usize,
    pub umanite: usize,
}

#[derive(Clone)]
enum OcStatus {
    Forged,
    Unforged,
}
impl OcStatus {
    fn string(&self) -> String {
        match self {
            OcStatus::Forged =>   String::from("Forged  "),
            OcStatus::Unforged => String::from("Unforged"),
        }
    }
}
impl Default for OcStatus {
    fn default() -> Self {
        OcStatus::Unforged
    }
}