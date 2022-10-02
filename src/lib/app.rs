use std::{path::PathBuf, fs};

use rfd::FileDialog;

use super::{save_file_utils::{Save, self}, widgets::val_text};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    //the path to the current file being edited
    sav_file_path: Option<PathBuf>,
    #[serde(skip)]
    sav_data: Option<Save>,

    // Example stuff:
    _label: String,
    

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    _value: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            sav_file_path: None,
            sav_data: None,
            // Example stuff:
            _label: "Hello World!".to_owned(),
            _value: 2.7,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app:TemplateApp;
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            app = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        else {
            app = Default::default()
        }

        //for testing purposes, start up with a default Save
        app.sav_data = Some(Default::default());

        return app;
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { sav_file_path,sav_data, _label, _value } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
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
                ui.horizontal(|ui| {
                    ui.label("Bismor");
                    match sav_data {
                        Some(save) => ui.text_edit_singleline(&mut save.resources.minerals.bismor),
                        _ => ui.text_edit_singleline(&mut val_text::number_box_validator()) //because the text validator is declared and initialized in this scope, it's reset every frame, effectively blocking input while a save file is not loaded, this is desired behavior and not a bug
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Croppa");
                    match sav_data {
                        Some(save) => ui.text_edit_singleline(&mut save.resources.minerals.croppa),
                        _ => ui.text_edit_singleline(&mut val_text::number_box_validator())
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Enor Pearl");
                    match sav_data {
                        Some(save) => ui.text_edit_singleline(&mut save.resources.minerals.enor_pearl),
                        _ => ui.text_edit_singleline(&mut val_text::number_box_validator())
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Jadiz");
                    match sav_data {
                        Some(save) => ui.text_edit_singleline(&mut save.resources.minerals.jadiz),
                        _ => ui.text_edit_singleline(&mut val_text::number_box_validator())
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Magnite");
                    match sav_data {
                        Some(save) => ui.text_edit_singleline(&mut save.resources.minerals.magnite),
                        _ => ui.text_edit_singleline(&mut val_text::number_box_validator())
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Umanite");
                    match sav_data {
                        Some(save) => ui.text_edit_singleline(&mut save.resources.minerals.umanite),
                        _ => ui.text_edit_singleline(&mut val_text::number_box_validator())
                    }
                });
            });

            // ui.horizontal(|ui| {
            //     ui.label("Write something: ");
            //     ui.text_edit_singleline(label);
            // });

            // ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
            // if ui.button("Increment").clicked() {
            //     *value += 1.0;
            // }

            // ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            //     ui.horizontal(|ui| {
            //         ui.spacing_mut().item_spacing.x = 0.0;
            //         ui.label("powered by ");
            //         ui.hyperlink_to("egui", "https://github.com/emilk/egui");
            //         ui.label(" and ");
            //         ui.hyperlink_to(
            //             "eframe",
            //             "https://github.com/emilk/egui/tree/master/crates/eframe",
            //         );
            //         ui.label(".");
            //     });
            // });
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
    }
}
