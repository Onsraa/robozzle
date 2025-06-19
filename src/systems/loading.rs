use crate::resources::level::LevelManager;
use crate::resources::loading::LoadingState;
use crate::states::game::GameState;
use bevy::prelude::*;
use std::fs;

// Composant marker pour indiquer que le chargement est en cours
#[derive(Component)]
pub struct LoadingIndicator;

// Système pour charger les niveaux tutoriel
pub fn load_tutorial_levels_on_enter_system(
    mut loading_state: ResMut<LoadingState>,
    mut level_manager: ResMut<LevelManager>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
) {
    // Reset l'état d'erreur
    loading_state.error_message = None;

    let tutorial_path = "src/levels/tutorials";
    info!(
        "Démarrage du chargement des tutoriels depuis: {}",
        tutorial_path
    );

    // Affiche temporairement un message de chargement
    let loading_entity = commands
        .spawn((
            Text::new("Chargement des tutoriels..."),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(50.0),
                top: Val::Percent(50.0),
                justify_self: JustifySelf::Center,
                align_self: AlignSelf::Center,
                ..default()
            },
            LoadingIndicator,
        ))
        .id();

    // Vérifie que le dossier existe
    if !fs::metadata(tutorial_path).is_ok() {
        let error_msg = format!(
            "Dossier '{}' introuvable. Créez le dossier et ajoutez vos fichiers de tutoriel 1.txt, 2.txt, etc.",
            tutorial_path
        );
        error!("{}", error_msg);
        loading_state.error_message = Some(error_msg);

        // Met à jour le message d'erreur
        commands.entity(loading_entity).despawn();
        commands.spawn((
            Text::new(format!(
                "Erreur: {}\nAppuyez sur Échap pour quitter",
                loading_state.error_message.as_ref().unwrap()
            )),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(50.0),
                top: Val::Percent(50.0),
                justify_self: JustifySelf::Center,
                align_self: AlignSelf::Center,
                ..default()
            },
            LoadingIndicator,
        ));
        return;
    }

    // Charge les niveaux tutoriel
    match LevelManager::load_tutorial_levels_from_directory(tutorial_path) {
        Ok(tutorial_levels) => {
            let num_levels = tutorial_levels.len();
            info!("Chargement réussi de {} tutoriel(s)", num_levels);

            if num_levels == 0 {
                let error_msg = format!(
                    "Aucun fichier .txt trouvé dans le dossier '{}'. Ajoutez vos fichiers de tutoriel 1.txt, 2.txt, etc.",
                    tutorial_path
                );
                warn!("{}", error_msg);
                loading_state.error_message = Some(error_msg);

                // Affiche l'erreur
                commands.entity(loading_entity).despawn();
                commands.spawn((
                    Text::new(format!(
                        "Erreur: {}\nAppuyez sur Échap pour quitter",
                        loading_state.error_message.as_ref().unwrap()
                    )),
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(50.0),
                        top: Val::Percent(50.0),
                        justify_self: JustifySelf::Center,
                        align_self: AlignSelf::Center,
                        ..default()
                    },
                    LoadingIndicator,
                ));
                return;
            } else {
                level_manager.set_tutorial_levels(tutorial_levels);
            }

            // Supprime l'indicateur de chargement
            commands.entity(loading_entity).despawn();

            // Transition vers l'état Tutorial
            next_state.set(GameState::Tutorial);
            info!(
                "Transition vers l'état Tutorial avec {} niveaux",
                level_manager.get_levels_count()
            );
        }
        Err(e) => {
            error!("Erreur chargement tutoriels: {}", e);
            loading_state.error_message = Some(e);

            // Affiche le message d'erreur
            commands.entity(loading_entity).despawn();
            commands.spawn((
                Text::new(format!(
                    "Erreur: {}\nAppuyez sur Échap pour quitter",
                    loading_state.error_message.as_ref().unwrap()
                )),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(50.0),
                    top: Val::Percent(50.0),
                    justify_self: JustifySelf::Center,
                    align_self: AlignSelf::Center,
                    ..default()
                },
                LoadingIndicator,
            ));
        }
    }
}

// Système principal qui charge les niveaux normaux quand on entre dans l'état Loading
pub fn load_levels_on_enter_system(
    mut loading_state: ResMut<LoadingState>,
    mut level_manager: ResMut<LevelManager>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
) {
    // Reset l'état d'erreur
    loading_state.error_message = None;

    info!(
        "Demarrage du chargement des niveaux depuis: {}",
        loading_state.levels_path
    );

    // Affiche temporairement un message de chargement
    let loading_entity = commands
        .spawn((
            Text::new("Chargement des niveaux..."),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(50.0),
                top: Val::Percent(50.0),
                justify_self: JustifySelf::Center,
                align_self: AlignSelf::Center,
                ..default()
            },
            LoadingIndicator,
        ))
        .id();

    // Vérifie que le dossier existe
    if !fs::metadata(&loading_state.levels_path).is_ok() {
        let error_msg = format!(
            "Dossier '{}' introuvable. Creez le dossier et ajoutez vos fichiers 1.txt, 2.txt, etc.",
            loading_state.levels_path
        );
        error!("{}", error_msg);
        loading_state.error_message = Some(error_msg);

        // Met à jour le message d'erreur
        commands.entity(loading_entity).despawn();
        commands.spawn((
            Text::new(format!(
                "Erreur: {}\nAppuyez sur Echap pour revenir",
                loading_state.error_message.as_ref().unwrap()
            )),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(50.0),
                top: Val::Percent(50.0),
                justify_self: JustifySelf::Center,
                align_self: AlignSelf::Center,
                ..default()
            },
            LoadingIndicator,
        ));
        return;
    }

    // Charge les niveaux normaux
    match LevelManager::load_normal_levels_from_directory(&loading_state.levels_path) {
        Ok(normal_levels) => {
            let num_levels = normal_levels.len();
            info!("Chargement réussi de {} niveau(x)", num_levels);

            if num_levels == 0 {
                let error_msg = format!(
                    "Aucun fichier .txt trouvé dans le dossier '{}'. Ajoutez vos fichiers 1.txt, 2.txt, etc.",
                    loading_state.levels_path
                );
                warn!("{}", error_msg);
                loading_state.error_message = Some(error_msg);

                // Affiche l'erreur
                commands.entity(loading_entity).despawn();
                commands.spawn((
                    Text::new(format!(
                        "Erreur: {}\nAppuyez sur Échap pour revenir",
                        loading_state.error_message.as_ref().unwrap()
                    )),
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(50.0),
                        top: Val::Percent(50.0),
                        justify_self: JustifySelf::Center,
                        align_self: AlignSelf::Center,
                        ..default()
                    },
                    LoadingIndicator,
                ));
                return;
            } else {
                level_manager.set_normal_levels(normal_levels);
                level_manager.switch_level_type(crate::resources::level::LevelType::Normal);
            }

            // Supprime l'indicateur de chargement
            commands.entity(loading_entity).despawn();

            // Transition vers l'état Menu
            next_state.set(GameState::Menu);
            info!(
                "Transition vers l'etat Menu avec {} niveaux",
                level_manager.get_levels_count()
            );
        }
        Err(e) => {
            error!("Erreur chargement niveaux: {}", e);
            loading_state.error_message = Some(e);

            // Affiche le message d'erreur
            commands.entity(loading_entity).despawn();
            commands.spawn((
                Text::new(format!(
                    "Erreur: {}\nAppuyez sur Echap pour revenir",
                    loading_state.error_message.as_ref().unwrap()
                )),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(50.0),
                    top: Val::Percent(50.0),
                    justify_self: JustifySelf::Center,
                    align_self: AlignSelf::Center,
                    ..default()
                },
                LoadingIndicator,
            ));
        }
    }
}

// Système pour gérer les erreurs de chargement (permet de revenir en arrière ou quitter)
pub fn handle_loading_error_system(
    loading_state: Res<LoadingState>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>,
) {
    if loading_state.error_message.is_some() && keyboard.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}
