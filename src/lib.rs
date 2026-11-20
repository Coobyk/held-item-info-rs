//! Held Item Info's own crate.
//!
//! The HUD mixin under `src/mixins/` is compiled into a copy of
//! `Game::native_render_gui_post`; the state and drawing live in the
//! `src/runtime.rs` module `[[native.module]]` injects as
//! `crate::game::umbx_held_item_info`. This crate only registers the mod's
//! settings page (Options > Mods > Held Item Info).

#[derive(Default)]
struct Mod;

#[mod_api::horizon_mod]
impl mod_api::Hooks for Mod {
    fn register(&self, r: &mut dyn mod_api::Registrar, _: &mod_api::RegistrationContext) {
        let _ = r.register_setting(
            mod_api::SettingSpec::toggle("enabled", "held_item_info.setting.enabled", true)
                .tooltip("held_item_info.setting.enabled.tooltip"),
        );
        let _ = r.register_setting(
            mod_api::SettingSpec::toggle(
                "background",
                "held_item_info.setting.background",
                true,
            )
            .tooltip("held_item_info.setting.background.tooltip"),
        );
    }
}
