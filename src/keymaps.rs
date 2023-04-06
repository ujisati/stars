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
            event::KeyCode::Left => {
                if tui_state.active_view == ui::View::Galaxy {
                    // TODO: the selected is good, but scrolling with arrows is WRONG HERE. USE LEFT RIGHT UP DOWN ON SORTED ASTRO_OBJS
                    tui_state.galaxy_view.selected_astro_obj = Some(
                        tui_state.galaxy_view.astro_objs[(tui_state.galaxy_view.selected_idx - 1)
                            % (tui_state.galaxy_view.astro_objs.len() - 1)],
                    );
                    log_key_event("previous astro obj");
                    return;
                }
            }
            event::KeyCode::Right => {
                if tui_state.active_view == ui::View::Galaxy {
                    tui_state.galaxy_view.selected_astro_obj = Some(
                        tui_state.galaxy_view.astro_objs[(tui_state.galaxy_view.selected_idx + 1)
                            % (tui_state.galaxy_view.astro_objs.len() - 1)],
                    );
                    log_key_event("next astro obj");
                    return;
                }
            }
            _ => {}
        },
        _ => {}
    }
}
