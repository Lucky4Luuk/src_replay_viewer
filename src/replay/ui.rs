use super::*;

pub fn replay_ui(replay: &mut Replay, ctx: &egui::Context, ui: &mut egui::Ui) {
    replay_ui_main(replay, ui);

    let screen_rect = ctx.screen_rect();
    let tl_window = ((screen_rect.max.x - screen_rect.min.x) / 5.0 * 4.0, screen_rect.max.y - screen_rect.min.y);
    egui::Window::new("replay_timeline")
        .resizable(false)
        .title_bar(false)
        .fixed_size(tl_window)
        .anchor(egui::Align2::CENTER_BOTTOM, (0f32, 0f32))
        .collapsible(false)
        .show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.scope(|ui| {
                    let mut style = ui.style_mut();
                    style.spacing.slider_width = tl_window.0 - 5.0;
                    ui.add(egui::Slider::new(&mut replay.time, 0.0..=replay.end).show_value(false));
                });
                let minute = (replay.time / 60.0) as usize;
                let second = (replay.time % 60.0) as usize;
                ui.label(format!("{}:{:0>2}", minute, second));
            });
        });
}

fn replay_ui_main(replay: &Replay, ui: &mut egui::Ui) {
    let cur_state = replay.get_state();
    ui.label(format!("{:#?}", cur_state));
}
