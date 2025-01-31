use crate::world::GameState;

use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use iyes_progress::{ProgressCounter, ProgressPlugin};

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ProgressPlugin::new(GameState::Loading).continue_to(GameState::Playing))
            .add_loading_state(
                LoadingState::new(GameState::Loading).load_collection::<WorldProps>(),
            )
            .add_systems(
                Update,
                print_progress
                    .run_if(in_state(GameState::Loading))
                    .after(LoadingStateSet(GameState::Loading)),
            );
    }
}

#[derive(AssetCollection, Resource)]
pub struct WorldProps {
    #[asset(path = "models/wood_crate.glb#Scene0")]
    pub wood_crate: Handle<Scene>,
}

fn print_progress(
    progress: Option<Res<ProgressCounter>>,
    diagnostics: Res<DiagnosticsStore>,
    mut last_done: Local<u32>,
) {
    if let Some(progress) = progress.map(|counter| counter.progress()) {
        if progress.done > *last_done {
            *last_done = progress.done;
            info!(
                "[Frame {}] Changed progress: {:?}",
                diagnostics
                    .get(&FrameTimeDiagnosticsPlugin::FRAME_COUNT)
                    .map(|diagnostic| diagnostic.value().unwrap_or(0.))
                    .unwrap_or(0.),
                progress
            );
        }
    }
}
