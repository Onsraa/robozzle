use crate::plugins::grid::GridDisplayPlugin;
use crate::plugins::loading::LevelLoadingPlugin;
use crate::plugins::menu::MenuPlugin;
use crate::plugins::player::PlayerInfoPlugin;
use crate::plugins::setup::SetupPlugin;
use crate::plugins::timer::TimerPlugin;
use crate::plugins::timeup::TimeUpPlugin;
use crate::systems::ui::EguiUIPlugin;
use bevy::prelude::*;

pub struct RobozzlePlugin;

impl Plugin for RobozzlePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            SetupPlugin,        // Plugin pour setup le jeu
            LevelLoadingPlugin, // Plugin de chargement des niveaux
            MenuPlugin,         // Plugin de menu
            PlayerInfoPlugin,   // Plugin pour la saisie des infos joueur
            GridDisplayPlugin,  // Plugin d'affichage de la grille
            EguiUIPlugin,       // Plugin d'édition d'instructions
            TimerPlugin,        // Plugin de gestion des timers
            TimeUpPlugin,       // Plugin pour l'écran de fin
        ));
    }
}
