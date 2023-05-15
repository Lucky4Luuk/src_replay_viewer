use super::*;

pub fn replay_ui(replay: &mut Replay, ctx: &egui::Context, ui: &mut egui::Ui) {
    replay_ui_main(replay, ctx, ui);

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
            ui.vertical_centered(|ui| {
                ui.columns(7, |columns| {
                    columns[1].button("|<");
                    columns[2].button("<<");
                    columns[3].button("=>");
                    columns[4].button(">>");
                    columns[5].button(">|");
                });
            });
        });
}

// TODO: Zoom in/out
// TODO: Draw car bbox based on size (no data yet)
// TODO: Tween zoom and drag
// TODO: Interpolation based on velocity and rotational velocity
// TODO: Event list? Show important events like car edits, players joining/leaving, etc
// TODO: Draw track (probably just an overhead image?)
fn replay_ui_main(replay: &mut Replay, ctx: &egui::Context, ui: &mut egui::Ui) {
    let cur_state = replay.get_state();
    // ui.label(format!("{:#?}", cur_state));
    let screen_rect = ctx.screen_rect();
    let size = egui::Vec2::new(screen_rect.max.x - screen_rect.min.x, screen_rect.max.y - screen_rect.min.y);
    let (resp, painter) = ui.allocate_painter(size, egui::Sense::drag());

    let drag_delta = resp.drag_delta();
    replay.cam_pos -= drag_delta; // Inverted so you drag the world around

    let rect = resp.rect;
    // let c = rect.center();
    let c = egui::Pos2::new(rect.min.x, rect.min.y);
    painter.text(c, egui::Align2::LEFT_TOP, format!("drag_delta: {:?}", drag_delta), egui::FontId::monospace(16f32), egui::Color32::WHITE);
    let c = egui::Pos2::new(rect.min.x, rect.min.y + 18f32);
    painter.text(c, egui::Align2::LEFT_TOP, format!("cam_pos: {:?}", replay.cam_pos), egui::FontId::monospace(16f32), egui::Color32::WHITE);

    // Draw the replay state
    for ((pid, vid), car) in &cur_state.cars {
        if car.size == (0f32, 0f32) {
            // We don't know the size, so draw a dot for now
            let c = egui::Vec2::new(car.pos.0, car.pos.1) - replay.cam_pos;
            painter.circle_filled(egui::Pos2::new(c.x, c.y), 4f32, egui::Color32::WHITE);
        } else {
            // We know the size, so draw an actual box
            todo!("My test data has no size :(");
        }
    }
}
