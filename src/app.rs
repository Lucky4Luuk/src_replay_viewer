use crate::replay::*;

pub struct App {
    replay: Option<Replay>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            replay: None,
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        Default::default()
    }

    pub fn open_replay(&mut self, path: String) -> anyhow::Result<()> {
        let content = std::fs::read_to_string(path)?;
        let parsed = Replay::from_str(&content)?;
        self.replay = Some(parsed);
        Ok(())
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(replay) = &mut self.replay {
                replay_ui(replay, ctx, ui);
                // TODO: Only force refreshing every frame if a replay is being played back?
                ctx.request_repaint(); // Constantly trigger a repaint to force an update every frame
            } else {
                if let Some(path) = show_file_pick_ui(ctx, ui) {
                    self.open_replay(path).unwrap();
                }
            }
        });
    }
}

#[cfg(target_arch = "wasm32")]
fn show_file_picker(ui: &mut egui::Ui) -> Option<String> {
    None
}

#[cfg(not(target_arch = "wasm32"))]
fn show_file_picker(ui: &mut egui::Ui) -> Option<String> {
    rfd::FileDialog::new().pick_file().map(|path| path.display().to_string())
}

fn show_file_pick_ui(ctx: &egui::Context, ui: &mut egui::Ui) -> Option<String> {
    let mut result = None;
    egui::Window::new("Open a replay...")
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, (0f32,0f32))
        .collapsible(false)
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label("Drag and drop a replay file here or click on \"Open replay\" to open the replay...");
                ui.spacing();
                if ui.button("Open replay").clicked() {
                    if let Some(path) = show_file_picker(ui) {
                        result = Some(path);
                    }
                }
            });
        });
    result
}
