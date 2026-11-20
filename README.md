# Held Item Info (Horizon native mod)

A port of [A5b84's `held-item-info`](https://github.com/A5b84/held-item-info)
Fabric mod to a Horizon native mod (`held_item_info`).

While the held item's name shows above the hotbar after switching items, the
mod draws the stack's other tooltip lines — enchantments, potion effects,
container contents, lore, `Unbreakable`, sign text, map id, written book
info, spawner entity, and (with F3+H advanced tooltips) durability, item id
and component count — centred under the name, fading with it.

## Settings (Options > Mods > Held Item Info)

- **Show held item info** (`enabled`, default on): turn the overlay off
  without uninstalling.
- **Text background** (`background`, default on): draw vanilla's text
  backdrop behind each info line, like the item name's; off draws the text
  alone.

## How it works

- `src/runtime.rs` is injected as `crate::game::umbx_held_item_info`
  (`[[native.module]]`). Its `draw` reads the game's `tool_highlight` state
  (the same timer/alpha as the vanilla held name) and the selected hotbar
  stack, builds the vanilla tooltip lines through the game's own
  `item_resource_tooltip` (skipping the name line), and draws each line
  centred under the name with `textWithBackdrop`-style boxes.
- `src/mixins/hud.rs` calls it from `native_render_gui_post`, the HUD
  layer's mod hook (after the minimap, hidden under F1).

Not ported from the Fabric mod: the per-line-type toggles (enchantments,
potion effects, container contents, etc. — Horizon's settings page gets the
two toggles above instead), the bundle contents image (text lines only),
and the 1.8.0 "move up over the health bar / action bar" nudge.

Build: `horizon-launcher native-build --trust-native-code <this dir>` on a
machine with the Horizon native mod SDK.
