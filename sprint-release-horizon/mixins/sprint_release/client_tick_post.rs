// Hook helper: native_client_tick_post
fn sprint_release_client_tick_post(game: &mut crate::game::Game<'_>) {
    crate::game::sprint_release::on_client_tick(game);
}
