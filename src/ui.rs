use self::{file::FileDialog, graph::Graph};

mod file;
mod graph;

#[derive(Default)]
pub struct TraceView {
    fd: FileDialog,
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
        if let Some(image) = self.fd.get() {
            self.graph = Some(Graph::new(ctx, image));
        }

        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    #[cfg(not(target_arch = "wasm32"))]
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }

                    if ui.button("Open file").clicked() {
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
