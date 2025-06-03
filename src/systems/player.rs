use bevy::prelude::*;
use bevy_simple_text_input::{TextInput, TextInputValue, TextInputSubmitEvent, TextInputInactive};
use crate::resources::player::PlayerInfo;
use crate::states::game::GameState;

#[derive(Component)]
pub struct LastNameInput;

#[derive(Component)]
pub struct FirstNameInput;

#[derive(Component)]
pub struct PlayerInfoUI;

#[derive(Component)]
pub struct SubmitButton;

// Système pour créer l'interface
pub fn setup_player_info_ui(
    mut commands: Commands,
) {
    // Camera UI
    commands.spawn((Camera2d, PlayerInfoUI));

    // Container principal
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
        PlayerInfoUI,
    )).with_children(|parent| {
        // Panneau central
        parent.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(40.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            BorderColor(Color::srgb(0.4, 0.4, 0.4)),
        )).with_children(|parent| {
            // Titre
            parent.spawn((
                Text::new("Informations du candidat"),
                TextFont {
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(30.0)),
                    ..default()
                },
            ));

            // Champ Nom
            parent.spawn(Node {
                flex_direction: FlexDirection::Column,
                margin: UiRect::bottom(Val::Px(20.0)),
                ..default()
            }).with_children(|parent| {
                parent.spawn((
                    Text::new("Nom :"),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.8, 0.8, 0.8)),
                    Node {
                        margin: UiRect::bottom(Val::Px(5.0)),
                        ..default()
                    },
                ));

                // Spawn avec moins de components dans le tuple
                parent.spawn((
                    TextInput,
                    TextInputValue("".to_string()),
                    TextInputInactive(true), // Commence inactif avec le bon paramètre
                    Node {
                        width: Val::Px(300.0),
                        padding: UiRect::all(Val::Px(10.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                    BorderColor(Color::srgb(0.5, 0.5, 0.5)),
                ))
                    .insert(LastNameInput); // Insérer séparément
            });

            // Champ Prénom
            parent.spawn(Node {
                flex_direction: FlexDirection::Column,
                margin: UiRect::bottom(Val::Px(30.0)),
                ..default()
            }).with_children(|parent| {
                parent.spawn((
                    Text::new("Prenom :"),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.8, 0.8, 0.8)),
                    Node {
                        margin: UiRect::bottom(Val::Px(5.0)),
                        ..default()
                    },
                ));

                // Spawn avec moins de components dans le tuple
                parent.spawn((
                    TextInput,
                    TextInputValue("".to_string()),
                    TextInputInactive(true), // Commence inactif avec le bon paramètre
                    Node {
                        width: Val::Px(300.0),
                        padding: UiRect::all(Val::Px(10.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                    BorderColor(Color::srgb(0.5, 0.5, 0.5)),
                ))
                    .insert(FirstNameInput); // Insérer séparément
            });

            // Bouton de validation
            parent.spawn((
                Button,
                Node {
                    width: Val::Px(220.0),
                    height: Val::Px(45.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
                SubmitButton,
            )).with_children(|parent| {
                parent.spawn((
                    Text::new("Commencer les exercices"),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.6, 0.6, 0.6)),
                ));
            });

            // Message d'erreur (invisible au début)
            parent.spawn((
                Text::new("Veuillez remplir tous les champs"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.3, 0.3)),
                Node {
                    margin: UiRect::top(Val::Px(15.0)),
                    ..default()
                },
                Visibility::Hidden,
            )).insert(ErrorMessage);
        });
    });
}

// Système pour gérer le focus des inputs
pub fn handle_input_focus(
    mut commands: Commands,
    interaction_query: Query<
    (Entity, &Interaction, Option<&LastNameInput>, Option<&FirstNameInput>),
    (Changed<Interaction>, With<TextInput>)
    >,
    mut all_inputs: Query<Entity, With<TextInput>>,
) {
    for (entity, interaction, is_last_name, is_first_name) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            // Désactiver tous les autres inputs
            for input_entity in all_inputs.iter_mut() {
                if input_entity != entity {
                    commands.entity(input_entity).insert(TextInputInactive(true));
                }
            }
            // Activer l'input cliqué
            commands.entity(entity).remove::<TextInputInactive>();
        }
    }
}

// Système pour gérer la validation
pub fn handle_player_info_validation(
    mut player_info: ResMut<PlayerInfo>,
    mut next_state: ResMut<NextState<GameState>>,
    button_query: Query<&Interaction, (Changed<Interaction>, With<SubmitButton>)>,
    last_name_query: Query<&TextInputValue, With<LastNameInput>>,
    first_name_query: Query<&TextInputValue, With<FirstNameInput>>,
    mut button_colors: Query<&mut BackgroundColor, With<SubmitButton>>,
    mut error_visibility: Query<&mut Visibility, With<ErrorMessage>>,
) {
    let Ok(last_name_value) = last_name_query.single() else { return };
    let Ok(first_name_value) = first_name_query.single() else { return };

    let last_name = last_name_value.0.trim();
    let first_name = first_name_value.0.trim();
    let can_submit = !last_name.is_empty() && !first_name.is_empty();

    // Mettre à jour l'apparence du bouton
    if let Ok(mut bg_color) = button_colors.single_mut() {
        if can_submit {
            bg_color.0 = Color::srgb(0.3, 0.7, 0.3);
        } else {
            bg_color.0 = Color::srgb(0.3, 0.3, 0.3);
        }
    }

    // Gérer le clic sur le bouton
    for interaction in button_query.iter() {
        if *interaction == Interaction::Pressed {
            if can_submit {
                player_info.last_name = last_name.to_string();
                player_info.first_name = first_name.to_string();
                info!("Informations du candidat: {} {}", 
                      player_info.last_name, player_info.first_name);
                next_state.set(GameState::Loading);
            } else {
                // Afficher le message d'erreur
                if let Ok(mut visibility) = error_visibility.get_single_mut() {
                    *visibility = Visibility::Visible;
                }
            }
        }
    }
}

// Système pour nettoyer l'UI quand on quitte l'état
pub fn cleanup_player_info_ui(
    mut commands: Commands,
    ui_query: Query<Entity, With<PlayerInfoUI>>,
) {
    for entity in ui_query.iter() {
        commands.entity(entity).despawn();
    }
}

#[derive(Component)]
pub struct ErrorMessage;