use egui::{vec2, Context, Response, Ui};
use egui_plot::{Legend, Plot, PlotImage, PlotPoint};
use image::DynamicImage;
use image::EncodableLayout;
use std::io::Cursor;

#[derive(Default)]
pub struct Graph {
    texture: Option<egui::TextureHandle>,
}

impl Graph {
    pub fn new(ctx: &Context, bytes: Vec<u8>) -> Self {
        // TODO: HANDLE THESE UNWRAPS
        let image = image::io::Reader::new(Cursor::new(bytes))
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap();

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

        let texture = ctx.load_texture("graph", color_image, Default::default());

        Self {
            texture: Some(texture),
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) -> Response {
        let texture = self.texture.get_or_insert_with(|| {
            ui.ctx()
                .load_texture("graph", egui::ColorImage::example(), Default::default())
        });

        let aspect_ratio = texture.aspect_ratio();

        let image = PlotImage::new(texture, PlotPoint::new(0.0, 0.0), vec2(aspect_ratio, 1.0));

        let plot = Plot::new("graph view")
            .legend(Legend::default())
            .show_x(false)
            .show_y(false)
            .show_grid(false);

        plot.show(ui, |plot_ui| plot_ui.image(image.name("graph")))
            .response
    }
}
