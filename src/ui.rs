use self::{file::FileDialog, graph::Graph};
use crate::trace::{TraceFile, Tracer};
use serde_json::from_str;

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
            let trace: TraceFile = from_str(&String::from_utf8_lossy(&trace)).unwrap();

            let tracer = Tracer::new(trace).expect("failed to parse");
            let graph = Graph::new(ctx, &tracer.graphs).expect("failed to graph");

            self.tracer = Some(tracer);
            self.graph = Some(graph);
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

                if let Some(tracer) = &mut self.tracer {
                    if ui.button("PREVIOUS").clicked() {
                        tracer.step_backward()
                    }

                    if ui.button("NEXT").clicked() {
                        tracer.step_forward()
                    }

                    ui.label(format!(
                        "LAST INSTRUCTION: {0}",
                        tracer.instructions[tracer.step].insn
                    ));
                }
            });
        });

        egui::TopBottomPanel::bottom("registers").show(ctx, |ui| {
            if let Some(tracer) = &mut self.tracer {
                for register in tracer.registers.iter() {
                    ui.label(register.to_string());
                }
            }
        });

        egui::SidePanel::right("functions").show(ctx, |ui| {
            if let Some(tracer) = &mut self.tracer {
                for func in &tracer.graphs {
                    if ui.button(&func.name).clicked() {
                        // Yeah, rather than rework the structure, just do this.
                        // Its bad.
                        if let Some(graph) = &mut self.graph {
                            graph.set_texture(&func.name);
                        }
                    }
                }
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(graph) = &mut self.graph {
                graph.ui(ui);
            }
        });
    }
}
