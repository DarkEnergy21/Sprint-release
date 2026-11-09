use std::sync::atomic::{AtomicBool, Ordering};

const SPRINT_MAPPING: &str = "key.sprint";
static SPRINT_KEY_WAS_DOWN: AtomicBool = AtomicBool::new(false);

pub fn on_client_tick(game: &mut crate::game::Game<'_>) {
    let sprint_binding = game.settings.keys.get(SPRINT_MAPPING);
    let sprint_key_is_down = horizon_client::keybinds::key_code(sprint_binding)
        .is_some_and(|key| game.keys.raw(&key))
        || horizon_client::keybinds::mouse_button(sprint_binding)
            .is_some_and(|button| game.physical_buttons.contains(&button));

    let sprint_key_was_down = SPRINT_KEY_WAS_DOWN.swap(sprint_key_is_down, Ordering::Relaxed);
    if sprint_key_was_down && !sprint_key_is_down {
        if let Some(player) = game.player.as_mut() {
            let _ = player.set_sprinting(false);
        }
    }
}
