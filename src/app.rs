use eframe::epaint::text::TextWrapMode;
use egui::ThemePreference;
use egui_code_editor::{CodeEditor, ColorTheme, Syntax, DEFAULT_THEMES};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateApp {
    label: String,
    #[serde(skip)]
    editor_theme: ColorTheme,
    #[serde(skip)]
    editor_syntax: Syntax,
    #[serde(skip)]
    value: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            label: "Hello World!".to_owned(),
            editor_theme: ColorTheme::default(),
            editor_syntax: Syntax::default(),
            value: 2.7,
        }
    }
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // top bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                }

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

        // main content
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.horizontal(|ui| {
                    // editor theme selector
                    egui::ComboBox::from_label("Theme")
                        .wrap_mode(TextWrapMode::Wrap)
                        .selected_text(format!("{:?}", &self.editor_theme.name))
                        .show_ui(ui, |ui| {
                            let themes = DEFAULT_THEMES.iter().clone();
                            for theme in themes {
                                ui.selectable_value(&mut self.editor_theme, *theme, theme.name);
                            }
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
                });
                CodeEditor::default()
                    .id_source("cat editor :3")
                    .with_fontsize(12.0)
                    .with_theme(self.editor_theme)
                    .with_syntax(self.editor_syntax.clone())
                    .with_numlines(true)
                    .show(ui, &mut self.label);
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
            });
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
