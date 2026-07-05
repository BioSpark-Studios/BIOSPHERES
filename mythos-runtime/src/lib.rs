pub mod components;

use components::MythosNodeComponent;
use fyrox::{
    core::{reflect::prelude::*, visitor::prelude::*},
    plugin::{error::GameResult, Plugin, PluginRegistrationContext},
};

/// BioSpark narrative engine plugin.
/// Register with `editor.add_game_plugin(MythosPlugin::default())` in editor-standalone,
/// or `executor.add_plugin(MythosPlugin::default())` in a game executor.
#[derive(Visit, Reflect, Debug, Default, PartialEq)]
#[reflect(non_cloneable, type_uuid = "c7a4e831-2f9d-4b8c-a015-6d3e9f1c7b2a")]
pub struct MythosPlugin;

impl Plugin for MythosPlugin {
    fn register(&self, context: PluginRegistrationContext) -> GameResult {
        context
            .serialization_context
            .script_constructors
            .add::<MythosNodeComponent>("Mythos Node");
        Ok(())
    }
}
