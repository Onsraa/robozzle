use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    LoadingTutorial,  // Chargement des niveaux tutoriel
    Tutorial,         // Phase tutoriel
    PlayerInfo,       // Saisie nom/prénom du candidat (après tutoriel)
    Loading,          // Chargement des niveaux principaux
    Menu,             // Sélection du problème
    Editing,          // Édition des instructions
    TimeUp,           // Temps écoulé - affichage des résultats
}