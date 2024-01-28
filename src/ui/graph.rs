use crate::trace::{Error, GraphFile};
use egui::{vec2, Context, Response, Ui};
use egui_plot::{Legend, Plot, PlotImage, PlotPoint};
use image::DynamicImage;
use image::EncodableLayout;

#[derive(Default)]
pub struct Graph {
    current_texture: usize,
    textures: Vec<egui::TextureHandle>,
}

impl Graph {
    pub fn new(ctx: &Context, graphs: &Vec<GraphFile>) -> Result<Graph, Error> {
        let mut textures = Vec::new();

        for graph in graphs {
            let image = image::io::Reader::open(&graph.path)?.decode()?;

            let color_image = match &image {
                DynamicImage::ImageRgb8(image) => egui::ColorImage::from_rgb(
                    [image.width() as usize, image.height() as usize],
                    image.as_bytes(),
                ),
                other => {
                    let image = other.to_rgba8();
                    egui::ColorImage::from_rgba_unmultiplied(
                        [image.width() as usize, image.height() as usize],
                        image.as_bytes(),
                    )
                }
            };

            textures.push(ctx.load_texture("graph", color_image, Default::default()));
        }

        Ok(Self {
            current_texture: 0,
            textures,
        })
    }

    pub fn ui(&mut self, ui: &mut Ui) -> Response {
        let texture = &self.textures[self.current_texture];

        let aspect_ratio = texture.aspect_ratio();

        let image = PlotImage::new(texture, PlotPoint::new(0.0, 0.0), vec2(aspect_ratio, 1.0));

        let plot = Plot::new("graph view")
            .legend(Legend::default())
            .show_x(false)
            .show_y(false)
            .data_aspect(0.5)
            .show_grid(false);

        plot.show(ui, |plot_ui| plot_ui.image(image.name("graph")))
            .response
    }
}
