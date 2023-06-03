use std::rc::Rc;

use review_tool::types::deserialize::{deserialize, Format};

fn main() {
    tracing_subscriber::fmt::init();

    // Reading manuscripts
    let manuscripts = {
        let file = std::fs::File::open("manuscripts.csv").expect("failed to open manuscripts");

        deserialize(Format::SitconGdsc, file).expect("failed to deserialize manuscripts")
    }
    .into_iter()
    .map(Rc::new)
    .collect();

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Review Tool",
        options,
        Box::new(|cc| {
            Box::new(review_tool::ui::ReviewToolApp::new(cc, manuscripts))
        }),
    )
    .expect("failed to start UI")
}
