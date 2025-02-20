use eframe::emath::Align;
use eframe::epaint::text::TextWrapMode;
use egui::ThemePreference;
use egui_code_editor::{CodeEditor, ColorTheme, Syntax, DEFAULT_THEMES};
use egui_modal::Modal;
use std::fs;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct CatnipApp {
    label: String,
    #[serde(skip)]
    editor_theme: ColorTheme,
    #[serde(skip)]
    editor_syntax: Syntax,
    editor_text: String,
    #[serde(skip)]
    value: f32,
}

impl Default for CatnipApp {
    fn default() -> Self {
        Self {
            label: "println!(\"meow\");".to_owned(),
            editor_theme: ColorTheme::default(),
            editor_syntax: Syntax::default(),
            editor_text: "".to_owned(),
            value: 2.7,
        }
    }
}

impl CatnipApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

impl eframe::App for CatnipApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let modal = Modal::new(ctx, "open folder modal :3");
        modal.show(|ui| {
            modal.title(ui, "Hello world!");
            modal.frame(ui, |ui| {
                modal.body(ui, "This is a modal.");
            });
            modal.buttons(ui, |ui| {
                if modal.button(ui, "meow").clicked() {
                };
                if modal.button(ui, "close").clicked() {
                };
            });
        });

        // top bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    ui.menu_button("New …", |_| {});
                    ui.menu_button("Open …", |ui| {
                        if ui.button("File").clicked()
                            && let Some(path) = rfd::FileDialog::new().pick_file()
                        {
                            self.editor_text =
                                fs::read_to_string(path.display().to_string()).unwrap();
                            ui.close_menu();
                        }
                        if ui.button("Folder").clicked() && !modal.is_open() {
                            modal.open();
                            ui.close_menu();
                        }
                    });
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
        });

        // main content
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.image(egui::include_image!("leaf.png"))
                .on_hover_text_at_pointer("test meow");
            ui.with_layout(egui::Layout::left_to_right(Align::Max), |ui| {
                CodeEditor::default()
                    .id_source("cat editor :3")
                    .with_fontsize(12.0)
                    .with_theme(self.editor_theme)
                    .with_syntax(self.editor_syntax.clone())
                    .with_numlines(true)
                    .vscroll(true)
                    .stick_to_bottom(true)
                    .show(ui, &mut self.editor_text);
            });

            // header example:
            // ui.heading("eframe template");

            /* config examples with serde:
            ui.horizontal(|ui| {
              ui.label("Write something: ");
              ui.te xt_edit_singleline(&mut self.label);
            });

            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            */
        });

        // bottom bar
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label("made by ");
                ui.hyperlink_to("lunarydess", "https://github.com/lunarydess");
                ui.label(" with ♡");

                ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                    // editor theme selector
                    egui::ComboBox::from_label("Theme")
                        .wrap_mode(TextWrapMode::Wrap)
                        .selected_text(format!("{:?}", &self.editor_theme.name))
                        .show_ui(ui, |ui| {
                            DEFAULT_THEMES.iter().clone().for_each(|theme| {
                                ui.selectable_value(&mut self.editor_theme, *theme, theme.name);
                            });
                        });

                    // syntax selector
                    egui::ComboBox::from_label("Syntax")
                        .wrap_mode(TextWrapMode::Wrap)
                        .selected_text(format!("{:?}", &self.editor_syntax.language))
                        .show_ui(ui, |ui| {
                            [
                                Syntax::asm(),
                                Syntax::shell(),
                                Syntax::sql(),
                                Syntax::lua(),
                                Syntax::rust(),
                                Syntax::python(),
                                Syntax::java(),
                            ]
                            .iter()
                            .for_each(|syntax| {
                                ui.selectable_value(
                                    &mut self.editor_syntax,
                                    syntax.clone(),
                                    syntax.language,
                                );
                            });
                        });

                    let mut theme = ui.ctx().options(|opt| opt.theme_preference);
                    egui::ComboBox::from_label("")
                        .wrap_mode(TextWrapMode::Wrap)
                        .selected_text(format!("{:?}", theme))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut theme, ThemePreference::System, "💻 System");
                            ui.selectable_value(&mut theme, ThemePreference::Light, "☀ Light");
                            ui.selectable_value(&mut theme, ThemePreference::Dark, "🌙 Dark");
                        });
                    ui.ctx().set_theme(theme);
                });
            });
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
