mod file;

#[derive(Default)]
pub struct TraceView {}

impl TraceView {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        TraceView {
            ..Default::default()
        }
    }
}

impl eframe::App for TraceView {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {}
}
