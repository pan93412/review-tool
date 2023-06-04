#[cfg(not(target_family = "wasm"))]
fn main() {
    use review_tool::types::{
        deserialize::{deserialize, Format},
        rank::sitcon_gdsc,
    };

    tracing_subscriber::fmt::init();

    // Reading manuscripts
    let manuscripts = {
        let file = std::fs::File::open("manuscripts.csv").expect("failed to open manuscripts");

        deserialize(Format::SitconGdsc, file).expect("failed to deserialize manuscripts")
    }
    .into();

    let options = eframe::NativeOptions {
        app_id: Some("review-tool".to_owned()),
        ..Default::default()
    };
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

#[cfg(target_family = "wasm")]
mod wasm;

#[cfg(target_family = "wasm")]
fn main() {
    tracing_wasm::set_as_global_default();

    wasm_bindgen_futures::spawn_local(async move { wasm::run().await.expect("start UI") });
}
