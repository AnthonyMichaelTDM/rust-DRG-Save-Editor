use cached::proc_macro::once;
use serde::{Serialize, Deserialize};

#[once]
/// deserializes weapon-overclocks.toml into a nice struct
pub fn get_weapon_overclocks() -> DeserializedWeaponOverclockTOML {
    let mut path = project_root::get_project_root().expect("Error Obtaining Project Root").join("res");
    path = path.join("weapon-overclocks.toml");
    return toml::from_str( std::fs::read_to_string(&path).expect("Error finding weapon-overclocks.toml").as_str()).expect("Error deserializing weapon-overclocks.toml");
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
    #[allow(dead_code)]
    Forged,
    #[allow(dead_code)]
    Unforged,
    #[allow(dead_code)]
    Unacquired,
}
impl OcStatus {
    fn string(&self) -> String {
        match self {
            OcStatus::Forged =>     String::from("Forged   "),
            OcStatus::Unforged =>   String::from("Unforged "),
            OcStatus::Unacquired => String::from("Unaquired"),
        }
    }
}
impl Default for OcStatus {
    fn default() -> Self {
        OcStatus::Unforged
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::lib::resources::get_weapon_overclocks;

    //expected constants
    const NUM_WEAPONS_PER_CLASS:usize = 6;

    //unit tests
    #[test]
    /// ensure that the weapon ocs toml is parsed successfully, will fail if weapon-overclocks.toml is missing 1 or more sections.
    fn parsing_weapon_ocs_no_panic() {
        let result = std::panic::catch_unwind(|| get_weapon_overclocks());
        assert!(result.is_ok(), "Failed to parse weapon-overclocks.toml, one or more sections are missing or incomplete");
    }
    #[test]
    /// ensure the classes in the toml are being put into the right class object
    fn parsing_weapon_oc_toml_all_classes() {
        let classes = get_weapon_overclocks();
        assert!(classes.driller.name.eq("Driller"), "Driller mismatched");
        assert!(classes.engineer.name.eq("Engineer"), "Engineer mismatched");
        assert!(classes.gunner.name.eq("Gunner"), "Gunner mismatched");
        assert!(classes.scout.name.eq("Scout"), "Scout mismatched");
    }
    #[test]
    /// ensure every class has 6 weapons
    fn parsing_weapon_oc_toml_all_weapons() {
        let classes = get_weapon_overclocks();
        let classes = vec![&classes.driller,&classes.engineer,&classes.gunner,&classes.scout];
        assert!(classes.iter().map(|c| c.weapons.iter()).all(|w| w.len()==NUM_WEAPONS_PER_CLASS));
    }
    #[test]
    /// ensure there are no duplicate OC guids
    fn parsing_weapon_oc_toml_no_duplicate_weapon_oc_guids() {
        let classes = get_weapon_overclocks();
        let classes = vec![&classes.driller,&classes.engineer,&classes.gunner,&classes.scout];
        
        //get a list of the OCs
        let oc_vec:Vec<String> = classes.iter() // all the classes
            .flat_map(|c| c.weapons.iter()) // all the weapons
            .flat_map(|w| w.overclocks.iter()) //all the overclocks
            .map(|oc| oc.guid.clone()) //the GUIDs of all the overclocks
            .collect();
        //get a set of the OCs
        let mut oc_set:HashSet<&String> = HashSet::new();
        for guid in oc_vec.iter() {oc_set.insert(guid);}

        //make sure they're the same length
        assert!(oc_vec.len() == oc_set.len(), "OC vector length = {}, OC set length = {}", oc_vec.len(), oc_set.len());
    }
}
