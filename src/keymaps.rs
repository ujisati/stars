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
                tui_state.galaxy_view.origin.0 += 1.; 
            }
            event::KeyCode::Right if tui_state.active_view == ui::View::Galaxy => {
                tui_state.galaxy_view.origin.0 -= 1.;
            }
            event::KeyCode::Up if tui_state.active_view == ui::View::Galaxy => {
                tui_state.galaxy_view.origin.1 -= 1.;
            }
            event::KeyCode::Down if tui_state.active_view == ui::View::Galaxy => {
                tui_state.galaxy_view.origin.1 += 1.;
            }
            event::KeyCode::Char('i') if tui_state.active_view == ui::View::Galaxy => {
                tui_state.galaxy_view.scale = (tui_state.galaxy_view.scale + 0.25).clamp(1., 4.);
                tui_state.galaxy_view.origin.0 -= 1.;
                tui_state.galaxy_view.origin.1 -= 1.;
            }
            event::KeyCode::Char('o') if tui_state.active_view == ui::View::Galaxy => {
                tui_state.galaxy_view.scale = (tui_state.galaxy_view.scale - 0.25).clamp(1., 4.);
                tui_state.galaxy_view.origin.0 += 0.25;
                tui_state.galaxy_view.origin.1 += 0.25;
            }
            _ => {}
        },
        _ => {}
    }
}
