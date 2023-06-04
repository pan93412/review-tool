use eframe::wasm_bindgen::prelude::*;
use review_tool::types::deserialize::{deserialize, Format};
use wasm_bindgen_futures::JsFuture;

pub async fn run() -> Result<(), JsValue> {
    let runner = eframe::WebRunner::new();
    let web_options = eframe::WebOptions::default();

    // Reading manuscripts
    let manuscripts = {
        tracing::debug!("Reading manuscript.csv…");
        let response = JsFuture::from(
            web_sys::window()
                .expect("no window")
                .fetch_with_str("/manuscripts.csv"),
        )
        .await?
        .dyn_into::<web_sys::Response>()?;

        if response.status() != 200 {
            return Err("failed to fetch manuscripts".into());
        }

        tracing::debug!("Parsing manuscript.csv…");
        let manuscripts = JsFuture::from(response.text()?)
            .await?
            .as_string()
            .ok_or::<JsValue>("failed to convert response to string".into())?;

        let cursor = std::io::Cursor::new(manuscripts);
        let r = deserialize(Format::SitconGdsc, cursor)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        tracing::debug!("Done reading manuscript.csv");
        r
    }
    .into();

    runner
        .start("review-tool", web_options, {
            Box::new(|cc| {
                Box::new(review_tool::ui::ReviewToolApp::<
                    review_tool::types::rank::sitcon_gdsc::Group,
                >::new(cc, manuscripts).expect("run UI"))
            })
        })
        .await?;

    Ok(())
}
