use crate::resources::level::LevelManager;
use crate::resources::loading::LoadingState;
use crate::states::game::GameState;
use bevy::prelude::*;
use std::fs;

// Composant marker pour indiquer que le chargement est en cours
#[derive(Component)]
pub struct LoadingIndicator;

// Système principal qui charge les niveaux quand on entre dans l'état Loading
pub fn load_levels_on_enter_system(
    mut loading_state: ResMut<LoadingState>,
    mut level_manager: ResMut<LevelManager>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
) {
    // Reset l'état d'erreur
    loading_state.error_message = None;

    info!("Démarrage du chargement des niveaux depuis: {}", loading_state.levels_path);

    // Affiche temporairement un message de chargement
    let loading_entity = commands.spawn((
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
    )).id();

    // Vérifie que le dossier existe
    if !fs::metadata(&loading_state.levels_path).is_ok() {
        let error_msg = format!("Dossier '{}' introuvable. Créez le dossier et ajoutez vos fichiers 1.txt, 2.txt, etc.", loading_state.levels_path);
        error!("{}", error_msg);
        loading_state.error_message = Some(error_msg);

        // Met à jour le message d'erreur
        commands.entity(loading_entity).despawn_recursive();
        commands.spawn((
            Text::new(format!("Erreur: {}\nAppuyez sur Échap pour revenir",
                              loading_state.error_message.as_ref().unwrap())),
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

    // Charge les niveaux depuis le dossier
    match LevelManager::load_levels_from_directory(&loading_state.levels_path) {
        Ok(loaded_manager) => {
            let num_levels = loaded_manager.get_levels_count();
            info!("Chargement réussi de {} niveau(x)", num_levels);

            if num_levels == 0 {
                let error_msg = format!("Aucun fichier .txt trouvé dans le dossier '{}'. Ajoutez vos fichiers 1.txt, 2.txt, etc.", loading_state.levels_path);
                warn!("{}", error_msg);
                loading_state.error_message = Some(error_msg);

                // Affiche l'erreur
                commands.entity(loading_entity).despawn_recursive();
                commands.spawn((
                    Text::new(format!("Erreur: {}\nAppuyez sur Échap pour revenir",
                                      loading_state.error_message.as_ref().unwrap())),
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
                *level_manager = loaded_manager;
            }

            // Supprime l'indicateur de chargement
            commands.entity(loading_entity).despawn_recursive();

            // Transition vers l'état Menu
            next_state.set(GameState::Menu);
            info!("Transition vers l'état Menu avec {} niveaux", level_manager.get_levels_count());
        }
        Err(e) => {
            error!("Erreur chargement niveaux: {}", e);
            loading_state.error_message = Some(e);

            // Affiche le message d'erreur
            commands.entity(loading_entity).despawn_recursive();
            commands.spawn((
                Text::new(format!("Erreur: {}\nAppuyez sur Échap pour revenir",
                                  loading_state.error_message.as_ref().unwrap())),
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

// Système pour gérer les erreurs de chargement (permet de revenir en arrière)
pub fn handle_loading_error_system(
    loading_state: Res<LoadingState>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if loading_state.error_message.is_some() && keyboard.just_pressed(KeyCode::Escape) {
        // Retour à l'état PlayerInfo en cas d'erreur
        next_state.set(GameState::PlayerInfo);
    }
}