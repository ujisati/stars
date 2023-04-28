use bevy::prelude::App;
use crossterm::event;

use crate::ui;

pub fn handle_key_event(key: event::KeyEvent, tui_state: &mut ui::TuiState, app: &mut App) {
    let log_key_event = |action: &str| {
        log::info!(
            "key pressed: {:?}, kind: {:?}, action: {:?}",
            key.code,
            key.kind,
            action
        )
    };
    match key.kind {
        event::KeyEventKind::Press => match key.code {
            event::KeyCode::Char('H') if tui_state.active_modal == ui::Modal::Help => {
                tui_state.active_modal = ui::Modal::Off;
                log_key_event("hide help");
                return;
            }
            event::KeyCode::Char('H') if tui_state.active_modal != ui::Modal::Help => {
                tui_state.active_modal = ui::Modal::Help;
                log_key_event("show help");
                return;
            }
            event::KeyCode::Left if tui_state.active_view == ui::View::Galaxy => {
                tui_state.galaxy_view.camera.origin.0 += 5.;
            }
            event::KeyCode::Right if tui_state.active_view == ui::View::Galaxy => {
                tui_state.galaxy_view.camera.origin.0 -= 5.;
            }
            event::KeyCode::Up if tui_state.active_view == ui::View::Galaxy => {
                tui_state.galaxy_view.camera.origin.1 -= 5.;
            }
            event::KeyCode::Down if tui_state.active_view == ui::View::Galaxy => {
                tui_state.galaxy_view.camera.origin.1 += 5.;
            }
            event::KeyCode::Char('i') if tui_state.active_view == ui::View::Galaxy => {
                tui_state.galaxy_view.camera.zoom_in();
                // find the point on the canvas that is centered
                // find the proportional point when scaled
                // center the scaled point on the frame by moving the canvas origin by the difference between the new point and the frame center
                // let old_scale = tui_state.galaxy_view.scale;
                // if old_scale != 4. {
                //     tui_state.galaxy_view.scale =
                //         (tui_state.galaxy_view.scale + 0.25).clamp(1., 4.);
                //     let old_center = (
                //         tui_state.frame_size.0 as f64 / 2.,
                //         tui_state.frame_size.1 as f64 / 2.,
                //     );
                //     let scaled_canvas_point = (
                //         (old_center.0 - tui_state.galaxy_view.origin.0)
                //             * tui_state.galaxy_view.scale,
                //         (old_center.1 - tui_state.galaxy_view.origin.1)
                //             * tui_state.galaxy_view.scale,
                //     );
                //     let origin = (
                //         -(scaled_canvas_point.0 - old_center.0),
                //         -(scaled_canvas_point.1 - old_center.1),
                //     );
                //     tui_state.galaxy_view.origin = origin;
                // }
            }
            event::KeyCode::Char('o') if tui_state.active_view == ui::View::Galaxy => {
                tui_state.galaxy_view.camera.zoom_out();
                // let old_scale = tui_state.galaxy_view.scale;
                // if old_scale != 1. {
                //     tui_state.galaxy_view.scale =
                //         (tui_state.galaxy_view.scale - 0.25).clamp(1., 4.);
                //     let old_center = (
                //         tui_state.frame_size.0 as f64 / 2.,
                //         tui_state.frame_size.1 as f64 / 2.,
                //     );
                //     let scaled_canvas_point = (
                //         (old_center.0 - tui_state.galaxy_view.origin.0)
                //             * tui_state.galaxy_view.scale,
                //         (old_center.1 - tui_state.galaxy_view.origin.1)
                //             * tui_state.galaxy_view.scale,
                //     );
                //     let origin = (
                //         -(scaled_canvas_point.0 - old_center.0),
                //         -(scaled_canvas_point.1 - old_center.1),
                //     );
                //     tui_state.galaxy_view.origin = (0., 0.);
                // }
            }
            _ => {}
        },
        _ => {}
    }
}
