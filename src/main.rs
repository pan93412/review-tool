use review_tool::types::{
    deserialize::{deserialize, Format},
    rank::sitcon_gdsc,
};

fn main() {
    tracing_subscriber::fmt::init();

    // Reading manuscripts
    let manuscripts = {
        let file = std::fs::File::open("manuscripts.csv").expect("failed to open manuscripts");

        deserialize(Format::SitconGdsc, file).expect("failed to deserialize manuscripts")
    }
    .into();

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Review Tool",
        options,
        Box::new(|cc| {
            Box::new(
                review_tool::ui::ReviewToolApp::<sitcon_gdsc::Group>::new(cc, manuscripts).unwrap(),
            )
        }),
    )
    .expect("failed to start UI");
}
