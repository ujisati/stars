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
                tui_state.galaxy_view.origin_pan.0 += 10.;
            }
            event::KeyCode::Right if tui_state.active_view == ui::View::Galaxy => {
                tui_state.galaxy_view.origin_pan.0 -= 10.;
            }
            event::KeyCode::Up if tui_state.active_view == ui::View::Galaxy => {
                tui_state.galaxy_view.origin_pan.1 -= 10.;
            }
            event::KeyCode::Down if tui_state.active_view == ui::View::Galaxy => {
                tui_state.galaxy_view.origin_pan.1 += 10.;
            }
            event::KeyCode::Char('i') if tui_state.active_view == ui::View::Galaxy => {
                let old_scale = tui_state.galaxy_view.scale;
                tui_state.galaxy_view.scale = (tui_state.galaxy_view.scale + 0.25).clamp(1., 4.);
                let old_center = (
                    tui_state.frame_size.0 as f64 / 2.,
                    tui_state.frame_size.1 as f64 / 2.,
                );
                let new_center = (
                    (tui_state.frame_size.0 as f64 * tui_state.galaxy_view.scale) / 2.,
                    (tui_state.frame_size.1 as f64 * tui_state.galaxy_view.scale) / 2.,
                );
                if old_scale != 4. {
                    tui_state.galaxy_view.origin.0 = -(new_center.0 - old_center.0) + tui_state.galaxy_view.origin_pan.0;
                    tui_state.galaxy_view.origin.1 = -(new_center.1 - old_center.1) + tui_state.galaxy_view.origin_pan.1;
                }
            }
            event::KeyCode::Char('o') if tui_state.active_view == ui::View::Galaxy => {
                let old_scale = tui_state.galaxy_view.scale;
                tui_state.galaxy_view.scale = (tui_state.galaxy_view.scale - 0.25).clamp(1., 4.);
                let old_center = (
                    tui_state.frame_size.0 as f64 / 2.,
                    tui_state.frame_size.1 as f64 / 2.,
                );
                let new_center = (
                    (tui_state.frame_size.0 as f64 * tui_state.galaxy_view.scale) / 2.,
                    (tui_state.frame_size.1 as f64 * tui_state.galaxy_view.scale) / 2.,
                );
                if old_scale != 1. {
                    tui_state.galaxy_view.origin.0 = -(new_center.0 - old_center.0) + tui_state.galaxy_view.origin_pan.0;
                    tui_state.galaxy_view.origin.1 = -(new_center.1 - old_center.1) + tui_state.galaxy_view.origin_pan.1;
                } else {
                    tui_state.galaxy_view.origin.0 = 0. + tui_state.galaxy_view.origin_pan.0;
                    tui_state.galaxy_view.origin.1 = 0. + tui_state.galaxy_view.origin_pan.1;
                }
            }
            _ => {}
        },
        _ => {}
    }
}
