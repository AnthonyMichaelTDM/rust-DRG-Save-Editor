use std::{path::PathBuf, fs};

use eframe::egui::Ui;
use rfd::FileDialog;

use crate::lib::gui;

use super::{save_file_utils::{Save, self}, gui::widgets::val_text::ValText, resources};

pub struct Application<> {
    //the path to the current file being edited
    sav_file_path: Option<PathBuf>,
    //the data in that file
    sav_data: Option<Save>,

    //labels and text validators for resources
    minerals_label_and_validator: Vec<(String,ValText<usize>)>,
    brewing_label_and_validator: Vec<(String,ValText<usize>)>,
    misc_label_and_validator: Vec<(String,ValText<usize>)>,

    //classes
    driller_ui_comps: ClassUIComponents,
    engineer_ui_comps: ClassUIComponents,
    gunner_ui_comps: ClassUIComponents,
    scout_ui_comps: ClassUIComponents,

    //seasonal
    seasonal_progress_validator: ValText<usize>,
    seasonal_level_validator: ValText<usize>,
    seasonal_scrip_validator: ValText<usize>,

    //overclocks
    overclock_filter_combobox_options: Vec<OcFilter>,
    overclock_filter_combobox_selected: usize,

    //other
    weapon_ocs: resources::weapon_overclocks::DeserializedWeaponOverclockTOML,
}

impl Default for Application {
    fn default() -> Self {
        Self {
            //the path to the current file being edited
            sav_file_path: None,
            //the data in that file
            sav_data: None,
            //labels and text validators for resources
            minerals_label_and_validator: Vec::from([ //using a vector so that order is preserved
                (String::from("Bismor     "), Default::default()),
                (String::from("Croppa     "), Default::default()),
                (String::from("Enor Pearl "), Default::default()),
                (String::from("Jadiz      "), Default::default()),
                (String::from("Magnite    "), Default::default()),
                (String::from("Umanite    "), Default::default()),
            ]),
            brewing_label_and_validator: Vec::from([
                (String::from("Barley Bulb"), Default::default()),
                (String::from("Malt Star  "), Default::default()),
                (String::from("Starch Nut "), Default::default()),
                (String::from("Yeast Cone "), Default::default()),
            ]),
            misc_label_and_validator: Vec::from([
                (String::from("Error Cubes"), Default::default()),
                (String::from("Blank Cores"), Default::default()),
                (String::from("Credits    "), Default::default()),
                (String::from("Perk Points"), Default::default()),
                (String::from("Data Cells "), Default::default()),
                (String::from("Phazyonite "), Default::default()),
            ]),
            //classes
            driller_ui_comps:  ClassUIComponents::default(String::from(" Driller")),
            engineer_ui_comps: ClassUIComponents::default(String::from("Engineer")),
            gunner_ui_comps:   ClassUIComponents::default(String::from("  Gunner")),
            scout_ui_comps:    ClassUIComponents::default(String::from("   Scout")),
            //seasonal
            seasonal_progress_validator: Default::default(),
            seasonal_level_validator: Default::default(),
            seasonal_scrip_validator: Default::default(),
            //overclock
            overclock_filter_combobox_options: OcFilter::vec_with_all(),
            overclock_filter_combobox_selected: 0,
            //other
            weapon_ocs: resources::get_weapon_overclocks(),
            
        }
    }
}

impl Application {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut app:Application;
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        //if let Some(storage) = cc.storage {
        //    app = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        //}
        //else {
        //    app = Default::default()
        //}
        app = Default::default();

        //for testing purposes, start up with a default Save
        app.sav_data = Some(Default::default());

        return app;
    }

    /// updates self.sav_data with information from various gui components
    /// TODO: add 
    fn update_save(&mut self) {
        //DATA

        //read from all the validators, if they have a value in them then set the associated sav_data value to it.
        if let Some(save) = &mut self.sav_data {
            // Minerals
            {
                //bismor
                if let Some(val) = self.minerals_label_and_validator[0].1.get_val() {save.resources.minerals.bismor     = *val;}
                //croppa
                if let Some(val) = self.minerals_label_and_validator[1].1.get_val() {save.resources.minerals.croppa     = *val;}
                //enor_pearl
                if let Some(val) = self.minerals_label_and_validator[2].1.get_val() {save.resources.minerals.enor_pearl = *val;}
                //jadiz
                if let Some(val) = self.minerals_label_and_validator[3].1.get_val() {save.resources.minerals.jadiz      = *val;}
                //magnite
                if let Some(val) = self.minerals_label_and_validator[4].1.get_val() {save.resources.minerals.magnite    = *val;}
                //umanite
                if let Some(val) = self.minerals_label_and_validator[5].1.get_val() {save.resources.minerals.umanite    = *val;}
            } //putting this into a separate scope to improve readibility
            // Brewing
            {
                //barley_bulb
                if let Some(val) = self.brewing_label_and_validator[0].1.get_val() {save.resources.brewing.barley_bulb = *val}
                //malt_star
                if let Some(val) = self.brewing_label_and_validator[1].1.get_val() {save.resources.brewing.malt_star   = *val}
                //starch_nut
                if let Some(val) = self.brewing_label_and_validator[2].1.get_val() {save.resources.brewing.starch_nut  = *val}
                //yeast_cone
                if let Some(val) = self.brewing_label_and_validator[3].1.get_val() {save.resources.brewing.yeast_cone  = *val}
            } //putting this into a separate scope to improve readibility
            // Misc
            {
                //error_cubes
                if let Some(val) = self.misc_label_and_validator[0].1.get_val() {save.resources.miscellaneous.error_cubes = *val}
                //blank_cores
                if let Some(val) = self.misc_label_and_validator[1].1.get_val() {save.resources.miscellaneous.blank_cores = *val}
                //credits
                if let Some(val) = self.misc_label_and_validator[2].1.get_val() {save.resources.miscellaneous.credits     = *val}
                //data_cells
                if let Some(val) = self.misc_label_and_validator[3].1.get_val() {save.resources.miscellaneous.data_cells  = *val}
                //perk_points
                if let Some(val) = self.misc_label_and_validator[4].1.get_val() {save.resources.miscellaneous.perk_points = *val}
                //phazyonite
                if let Some(val) = self.misc_label_and_validator[5].1.get_val() {save.resources.miscellaneous.phazyonite  = *val}
            } //putting this into a separate scope to improve readibility

            // TODO: add other stuff
            todo!();            
        }
    }
}

impl eframe::App for Application {
    /// Called by the frame work to save state before shutdown.
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //     eframe::set_value(storage, eframe::APP_KEY, self);
    // }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //DATA
        let Application { 
            sav_file_path, 
            sav_data,
            minerals_label_and_validator,
            brewing_label_and_validator,
            misc_label_and_validator,
            driller_ui_comps: driller,
            engineer_ui_comps: engineer,
            gunner_ui_comps: gunner,
            scout_ui_comps: scout,
            seasonal_progress_validator,
            seasonal_level_validator,
            seasonal_scrip_validator,
            overclock_filter_combobox_options,
            overclock_filter_combobox_selected,
            weapon_ocs,
        } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }

                    if ui.button("Open File").clicked() {
                        //open a file dialog with rfd
                        *sav_file_path = FileDialog::new()
                            .add_filter("saves", &["sav"])
                            .set_directory("~/").pick_file();

                        //if sav_file_path is a valid path to a file with a .sav extension, make a backup of it, and send it to save_file::deserializer to be broken up.
                        if let Some(file_path) = sav_file_path {
                            if file_path.exists() && (file_path.extension().is_some() && file_path.extension().unwrap().eq_ignore_ascii_case("sav")) { // the last two expressions should be replaced with is_some_and() later
                                //create backup
                                fs::copy(&file_path, { let mut new_path = file_path.clone(); new_path.set_extension("sav.old"); new_path }).expect("Error creating backup");
                                //send to deserializer
                                *sav_data = Some(save_file_utils::deserializer::deserialize(&file_path));
                            }
                        }
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            /// function to add the labels and validators in the given Vec to the given UI
            fn add_labels_and_validators(ui: &mut Ui, labels_and_validators: &mut Vec<(String,ValText<usize>)>) {
                ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);        
                for (label,validator) in labels_and_validators.iter_mut() { //this consumes the collection, but that's okay, we can't use an iterator because we need the raw reference being stored, not a point to it
                    ui.horizontal(|ui| {
                        //labels
                        ui.label(label.to_string());
                        //text boxes
                        gui::add_singleline_textedit_with_validator(ui, validator, None);
                    });
                }
            }

            ui.heading("Minerals");
            add_labels_and_validators(ui, minerals_label_and_validator);
            ui.separator();

            ui.heading("Brewing");
            add_labels_and_validators(ui, brewing_label_and_validator);
            ui.separator();

            ui.heading("Miscellaneous");
            add_labels_and_validators(ui, misc_label_and_validator);

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(".");
                });
            });
        });

        egui::SidePanel::right("right_panel").show(ctx, |ui| {
            //right panel, contains the list of 'Aquired but unforged' OC's
            ui.heading("Aquired but unforged");
            
            // add a scrollable area for OCs
            egui::ScrollArea::vertical().id_source("aquired but unforged ocs").show(ui, |ui| {
                //iterate through the unforged OC's in the users save file, for each of them add a checkbox item that, 
                //when toggled, toggles the selected value of the associated Overclock object
                for unaquired_ocs in weapon_ocs.classes_mut().iter_mut().flat_map(|c| c.weapons.iter_mut()).flat_map(|w|w.overclocks.iter_mut()).filter(|oc| oc.status().to_owned() == resources::weapon_overclocks::OcStatus::Unforged) {

                }
            });

            // add the buttons

            todo!();
        });
        
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            
            // classes panel
            egui::ScrollArea::horizontal().id_source("classes panel").show(ui, |ui| {
                //heading
                if let Some(save) = sav_data {
                    ui.heading(format!("Classes - Rank {}, {}",save.classes.rank, save.classes.title));
                } else {
                    ui.heading("Classes");
                }

                //formatting
                ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);
                //contents
                for class in [driller,engineer,gunner,scout] {
                    ui.horizontal(|ui| {
                        //xp total
                        ui.label(&class.class_label);
                        gui::add_singleline_textedit_with_validator(ui, &mut class.xp_total_validator, Some(96.0f32));
                        //level
                        ui.label("Lvl ");
                        gui::add_singleline_textedit_with_validator(ui, &mut class.level_validator, Some(16.0f32));
                        //xp progress
                        ui.label("Progress");
                        gui::add_singleline_textedit_with_validator(ui, &mut class.xp_progress_validator, Some(96f32));
                        //promotion
                        ui.label("Promotion ");
                        egui::ComboBox::from_id_source(&class.id_source).show_index(
                            ui, 
                            &mut class.promotion_selected, 
                            class.promotion_options.len(), 
                            |i| class.promotion_options[i].to_owned()
                        )
                    });
                }
                
            });
            ui.separator();

            // season panel
            egui::ScrollArea::horizontal().id_source("season panel").show(ui, |ui| {
                //heading
                ui.heading("Seasonal");

                //contents
                ui.horizontal(|ui|{
                    ui.label("Lvl Progress");
                    gui::add_singleline_textedit_with_validator(ui, seasonal_progress_validator, Some(64.0f32));

                    ui.label("Lvl");
                    gui::add_singleline_textedit_with_validator(ui, seasonal_level_validator, Some(64.0f32));

                    ui.label("Scrip");
                    gui::add_singleline_textedit_with_validator(ui, seasonal_scrip_validator, Some(64.0f32));
                });
            });
            ui.separator();

            // overclocks panel
            egui::ScrollArea::both().id_source("overclocks panel").show(ui, |ui| {
                ui.spacing_mut().item_spacing.x = 10.0;

                //heading
                ui.horizontal(|ui|{
                    //heading
                    ui.heading("Overclocks");

                    //combo box
                    egui::ComboBox::from_id_source("oc filter").show_index(
                        ui, 
                        overclock_filter_combobox_selected, 
                        overclock_filter_combobox_options.len(), 
                        |i| overclock_filter_combobox_options[i].string()
                    );

                    //button
                    if ui.button("Add Cores To Inventory").clicked() {
                        todo!();
                    }
                });

                //contents (depends on selection in previous selection box)
                ui.scope(|ui| {
                    ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);
                    /// isolated repetitive code to ease with debugging and improve readibility
                    fn populate_collapsing_with_class_overclock_info(ui: &mut Ui, class: &mut resources::weapon_overclocks::Class) {
                        for weapon in class.weapons.iter_mut() {
                            ui.collapsing(&weapon.name, |ui| {
                                for overclock in weapon.overclocks.iter_mut() {
                                    ui.horizontal(|ui| {
                                        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                                            ui.checkbox(&mut overclock.selected, &overclock.name);
                                        });
                                        ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
                                            ui.horizontal_wrapped(|ui| {
                                                ui.label(&overclock.guid);
                                                ui.label(&overclock.status_string());
                                            });
                                        });
                                    });
                                }
                            });
                        }
                    }

                    //driller
                    let name = weapon_ocs.driller.name.clone();
                    ui.collapsing(name, |ui| populate_collapsing_with_class_overclock_info(ui, &mut weapon_ocs.driller));
                    //engineer
                    let name = weapon_ocs.engineer.name.clone();
                    ui.collapsing(name, |ui| populate_collapsing_with_class_overclock_info(ui, &mut weapon_ocs.engineer));
                    //gunner
                    let name = weapon_ocs.gunner.name.clone();
                    ui.collapsing(name, |ui| populate_collapsing_with_class_overclock_info(ui, &mut weapon_ocs.gunner));
                    //scout
                    let name = weapon_ocs.scout.name.clone();
                    ui.collapsing(name, |ui| populate_collapsing_with_class_overclock_info(ui, &mut weapon_ocs.scout));
                });
            });
        });

        // call update_save function to use input in gui components to modify self.sav_data
        self.update_save();
    }
}

struct ClassUIComponents {
    id_source: String,
    class_label: String,
    xp_total_validator: ValText<usize>,
    level_validator: ValText<usize>,
    xp_progress_validator: ValText<usize>,
    promotion_options: Vec<String>,
    promotion_selected: usize,
}
impl ClassUIComponents {
    fn default(class_label:String) -> Self {
        Self { 
            id_source: format!("{} promotions", class_label.trim().to_ascii_lowercase()),
            class_label, 
            xp_total_validator: Default::default(), 
            level_validator: ValText::with_validator(|text| text.parse().ok().filter(|n| *n <= 25) ), 
            xp_progress_validator: Default::default(), 
            promotion_options: resources::level_info::PROMO_RANKS.iter().map(|&s|s.to_owned()).collect(), 
            promotion_selected: 0 
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
/// enum to keep track of the filter selected for the Overclock selection panel
enum OcFilter {
    All,
    Unforged,
    Forged,
    Unacquired,
}
impl OcFilter {
    /// returns a vector with all the variants of the enum
    fn vec_with_all() -> Vec<OcFilter> {
        return vec![OcFilter::All, OcFilter::Unforged, OcFilter::Forged, OcFilter::Unacquired];
    }
    fn string(&self) -> String {
        return String::from(match self {
            OcFilter::All => "All",
            OcFilter::Unforged => "Unforged",
            OcFilter::Forged => "Forged",
            OcFilter::Unacquired => "Unaquired",
        });
    }
}


#[cfg(test)]
mod tests {
    //unit tests
}
