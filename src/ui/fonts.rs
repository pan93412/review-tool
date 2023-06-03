use egui::{FontData, FontDefinitions};

pub fn create_font_def() -> FontDefinitions {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "HarmonyOS Sans TC".to_owned(),
        FontData::from_static(include_bytes!("fonts/HarmonyOS_Sans_TC_Regular.ttf")),
    );
    fonts.font_data.insert(
        "HarmonyOS Sans SC".to_owned(),
        FontData::from_static(include_bytes!("fonts/HarmonyOS_Sans_SC_Regular.ttf")),
    );

    let proportional = fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .expect("failed to get font family");

    proportional.push("HarmonyOS Sans TC".to_owned());
    proportional.push("HarmonyOS Sans SC".to_owned());

    fonts
}
