use eframe::Renderer;
use application_catnip::CatnipApp;

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
        Box::new(|context| Ok(Box::new(CatnipApp::new(context)))),
    )
    .expect("TODO: panic message");
}
