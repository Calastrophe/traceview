#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::{Style, Visuals};

mod trace;
mod ui;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    env_logger::init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };

    eframe::run_native(
        "traceview",
        native_options,
        Box::new(|cc| {
            let style = Style {
                visuals: Visuals::light(),
                override_font_id: Some(egui::FontId {
                    size: 20.0,
                    family: egui::FontFamily::default(),
                }),
                ..Style::default()
            };
            cc.egui_ctx.set_style(style);

            Box::new(ui::TraceView::new(cc))
        }),
    )
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "traceview",
                web_options,
                Box::new(|cc| Box::new(ui::TraceView::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}
