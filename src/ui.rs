use crate::trace::{TraceFile, Tracer};
use serde_json::from_str;

use self::{file::FileDialog, graph::Graph};

mod file;
mod graph;

#[derive(Default)]
pub struct TraceView {
    fd: FileDialog,
    tracer: Option<Tracer>,
    graph: Option<Graph>,
}

impl TraceView {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        TraceView {
            ..Default::default()
        }
    }
}

impl eframe::App for TraceView {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(trace) = self.fd.get() {
            // HANDLE THIS ERROR
            let trace: TraceFile = from_str(&String::from_utf8_lossy(&trace)).unwrap();

            self.tracer = Some(Tracer::new(trace));
        }

        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    #[cfg(not(target_arch = "wasm32"))]
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }

                    if ui.button("Open trace").clicked() {
                        self.fd.open()
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(graph) = &mut self.graph {
                graph.ui(ui);
            }
        });
    }
}
