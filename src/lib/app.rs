use std::{path::PathBuf, fs};

use rfd::FileDialog;

use super::{save_file_utils::{Save, self}, gui::widgets::val_text::ValText};

pub struct Application<> {
    //the path to the current file being edited
    sav_file_path: Option<PathBuf>,
    //the data in that file
    sav_data: Option<Save>,

    //labels and text validators for resources
    minerals_label_and_validator: Vec<(String,ValText<usize>)>,
    brewing_label_and_validator: Vec<(String,ValText<usize>)>,
    misc_label_and_validator: Vec<(String,ValText<usize>)>,
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
    fn update_save(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
        let Application { 
            sav_file_path, 
            sav_data,
            minerals_label_and_validator,
            brewing_label_and_validator,
            misc_label_and_validator,
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
            ui.heading("Minerals");
            ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {  
                ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);             
                for (label,validator) in minerals_label_and_validator.iter_mut() { //this consumes the collection, but that's okay, we can't use an iterator because we need the raw reference being stored, not a point to it
                    ui.horizontal(|ui| {
                        //labels
                        ui.label(label.to_string());
                        //text boxes
                        //make text red when invalid input is given
                        if !validator.is_valid() {
                            ui.visuals_mut().override_text_color = Some(egui::Color32::RED);
                        }
                        ui.text_edit_singleline(validator);
                    });
                }
            });

            ui.heading("Brewing");
            ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {  
                ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);             
                for (label,validator) in brewing_label_and_validator.iter_mut() { //this consumes the collection, but that's okay, we can't use an iterator because we need the raw reference being stored, not a point to it
                    ui.horizontal(|ui| {
                        //labels
                        ui.label(label.to_string());
                        //text boxes
                        //make text red when invalid input is given
                        if !validator.is_valid() {
                            ui.visuals_mut().override_text_color = Some(egui::Color32::RED);
                        }
                        ui.text_edit_singleline(validator);
                    });
                }
            });

            ui.heading("Miscellaneous");
            ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {  
                ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);             
                for (label,validator) in misc_label_and_validator.iter_mut() { //this consumes the collection, but that's okay, we can't use an iterator because we need the raw reference being stored, not a point to it
                    ui.horizontal(|ui| {
                        //labels
                        ui.label(label.to_string());
                        //text boxes
                        //make text red when invalid input is given
                        if !validator.is_valid() {
                            ui.visuals_mut().override_text_color = Some(egui::Color32::RED);
                        }
                        ui.text_edit_singleline(validator);
                    });
                }
            });

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
        
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("eframe template");
            ui.hyperlink("https://github.com/emilk/eframe_template");
            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/master/",
                "Source code."
            ));
            egui::warn_if_debug_build(ui);
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }

        // call update_save function to use input in gui components to modify self.sav_data
        self.update_save(ctx, _frame);
    }
}
