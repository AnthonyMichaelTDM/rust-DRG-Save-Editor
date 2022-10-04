pub(crate) mod widgets;

///function to add a text edit box (with a validator) whose text turns red when invalid input is given
pub fn add_singleline_textedit_with_validator<T>(ui:&mut egui::Ui, validator: &mut widgets::val_text::ValText<T>, desired_width:Option<f32>) {
    ui.scope(|ui| {
        //make text red when invalid input is given
        if !validator.is_valid() {
            ui.visuals_mut().override_text_color = Some(egui::Color32::RED);
        }
        //determine whether or not to give the text box a set desired width
        match desired_width {
            Some(width) => ui.add(egui::TextEdit::singleline(validator).desired_width(width)),
            None => ui.text_edit_singleline(validator),
        };
    });
}