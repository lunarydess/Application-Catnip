use application_catnip::CatnipApp;
use eframe::Renderer;

fn main() {
    env_logger::init();
    eframe::run_native(
        "Catnip Editor",
        eframe::NativeOptions {
            centered: true,
            renderer: Renderer::Glow,
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([400.0, 300.0])
                .with_min_inner_size([300.0, 220.0]),
            ..Default::default()
        },
        Box::new(|context| {
            egui_extras::install_image_loaders(&context.egui_ctx);
            Ok(Box::<CatnipApp>::default())
        }),
    )
    .expect("TODO: panic message");
}
