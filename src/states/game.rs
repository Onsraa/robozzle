use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    PlayerInfo,  // Saisie nom/prénom du candidat
    Loading,     // Chargement des niveaux
    Menu,        // Sélection du problème
    Running,     // Exécution du programme
    Paused,      // Pause pendant l'exécution
    Editing,     // Édition des instructions
    TimeUp,      // Temps écoulé - affichage des résultats
}