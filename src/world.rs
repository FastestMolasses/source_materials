#![allow(clippy::type_complexity)]

use crate::game::GamePlugin;
use crate::loading::LoadingPlugin;
use crate::application::ApplicationPlugin;
use bevy::prelude::*;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    /// During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    /// Here the menu is drawn and waiting for player interaction
    Menu,
    /// During this State the actual game logic is executed
    Playing,
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_plugins((
            ApplicationPlugin,
            LoadingPlugin,
            GamePlugin,
        ));
    }
}
