// The held item's info under its name. `native_hud` is gone;
// `native_render_gui_post` is its new home, and it hands us the same batch
// and corner stack (after the minimap). The name's layer draws it first:
// the info hangs under it, toward the hotbar.
fn held_item_info_hud(game: &Game<'_>, batch: &mut HudBatch, corners: &mut Corners) {
    let _ = corners;
    crate::game::umbx_held_item_info::draw(game, batch);
}
