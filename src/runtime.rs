// Held Item Info's game side: the held stack's tooltip lines under its name
// while the name shows, fading with it — a Rust port of A5b84's Fabric mod
// `held-item-info`, reimplemented on the game's own tooltip builder.
//
// Injected as `crate::game::umbx_held_item_info` (see horizon-mod.toml); the
// mixin in `src/mixins/hud.rs` calls `draw` from `native_render_gui_post`,
// the HUD layer's mod hook (after the minimap, not drawn under F1). Settings
// are Horizon's per-mod files, read every frame so the page applies live.
use super::Game;

/// The mod's id and settings namespace.
pub(super) const ID: &str = "held_item_info";

/// The `enabled` switch: hide the info without uninstalling the mod.
fn enabled(game: &Game<'_>) -> bool {
    game.setting(ID, "enabled")
        .map_or(true, |v| v.trim() != "false")
}
/// The `background` switch (the Fabric mod's "tooltip background" option):
/// each line gets vanilla's `textWithBackdrop` box, off draws the text alone.
fn background(game: &Game<'_>) -> bool {
    game.setting(ID, "background")
        .map_or(true, |v| v.trim() != "false")
}

/// A packed batch colour (`rgba` packs `a | b | g | r` big-endian) with
/// `alpha` (0..=255) multiplied into its own, as the held name's fade scales
/// everything the info draws.
fn faded(color: u32, alpha: u32) -> u32 {
    (color & 0x00ff_ffff) | ((((color >> 24) & 255) * alpha / 255) << 24)
}

/// `RenderGuiEvent.Post`: while `Hud.toolHighlightTimer` has the selected
/// stack's name up, draw the stack's other tooltip lines under it, each
/// centred as the name is.
pub(super) fn draw(game: &Game<'_>, batch: &mut horizon_client::hud::HudBatch) {
    if !enabled(game) {
        return;
    }
    let Some(last) = &game.tool_highlight.last else {
        return;
    };
    if game.tool_highlight.timer <= 0 {
        return;
    }
    let Some(Some(stack)) = game.inventory.get(game.selected) else {
        return;
    };
    // The stack left the hand mid-fade; the tick restarts the timer next
    // tick, so only this frame would show stale info.
    if *stack.item != *last.item {
        return;
    }
    // `Hud.extractSelectedItemName`: the name's alpha from the highlight
    // timer; the info fades with it.
    let alpha = ((game.tool_highlight.timer as f32 * 256.0 / 10.0) as i32).clamp(0, 255) as u32;
    if alpha == 0 {
        return;
    }
    // `ItemStack.getTooltipLines` with the HUD's flags: the player's
    // `advancedItemTooltips` (F3+H) adds the durability, id and component
    // lines; `creative` is the creative tab's own slots, not this one.
    let advanced = game.settings.file.bool("advancedItemTooltips");
    let creative = false;
    let Some(elements) = game.item_resource_tooltip(stack, advanced, creative) else {
        return;
    };
    // The tooltip's first element is the stack's name when it shows one; the
    // HUD's held-name draw owns it.
    let name_shown = crate::item_tooltip::tooltip_display(stack).name_visible(creative);
    let mut lines = Vec::new();
    for (i, element) in elements.iter().enumerate() {
        let mod_api::TooltipElement::Text(text, color) = element else {
            continue;
        };
        if i == 0 && name_shown {
            continue;
        }
        if let Ok(text) = crate::api_text::to_engine(text) {
            lines.push((text.resolve_legacy(), *color));
        }
    }
    if lines.is_empty() {
        return;
    }
    let atlas = &game.gui.atlas;
    let font = &atlas.font;
    // `ToolHighlight.draw`: the name sits 59 pixels above the bottom edge,
    // 14 lower in a mode that cannot hurt the player (creative). The info
    // hangs under it toward the hotbar, one 11-pixel step a line.
    let w = batch.width as i32;
    let mut y = batch.height as i32 - 59;
    if game.creative {
        y += 14;
    }
    let backdrop = background(game)
        .then(|| game.settings.text_background(0.0))
        .filter(|b| *b != 0);
    let fade = horizon_client::hud_layout::argb(alpha, 255, 255, 255);
    let mut ly = y + 12;
    for (text, color) in &lines {
        let tw = font.width_text(text).ceil() as i32;
        let tx = (w - tw) / 2;
        if let Some(background) = backdrop {
            // `GuiGraphicsExtractor.textWithBackdrop`: two pixels around the
            // text's advance and the font's nine-pixel line height, the
            // backdrop's colour multiplied by the name's.
            let color = horizon_client::hud_layout::abgr(
                horizon_client::hud_layout::multiply(background, fade),
            );
            batch.fill(atlas, (tx - 2) as f32, (ly - 2) as f32, (tw + 4) as f32, 13.0, color);
        }
        batch.text_styled(atlas, text, tx as f32, ly as f32, faded(*color, alpha), true);
        ly += 11;
    }
}
