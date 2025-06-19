use bevy::prelude::*;
use bevy_simple_text_input::{
    TextInput, TextInputValue, TextInputSubmitEvent,
    TextInputTextFont, TextInputTextColor
};
use crate::resources::player::PlayerInfo;
use crate::states::game::GameState;

#[derive(Component)]
pub struct NameInput;

#[derive(Component)]
pub struct PlayerInfoUI;

#[derive(Component)]
pub struct SubmitButton;

#[derive(Component)]
pub struct ErrorMessage;

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

            // Champ unique Nom + Prénom
            parent.spawn(Node {
                flex_direction: FlexDirection::Column,
                margin: UiRect::bottom(Val::Px(30.0)),
                ..default()
            }).with_children(|parent| {
                parent.spawn((
                    Text::new("Nom et Prenom :"),
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

                // Input unique - actif par défaut
                parent.spawn((
                    Node {
                        width: Val::Px(400.0),
                        padding: UiRect::all(Val::Px(10.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                    BorderColor(Color::srgb(0.7, 0.7, 0.7)),
                    TextInput,
                    TextInputValue("".to_string()),
                    TextInputTextFont(TextFont {
                        font_size: 18.0,
                        ..default()
                    }),
                    TextInputTextColor(TextColor(Color::WHITE)),
                    NameInput,
                ));

                // Instructions
                parent.spawn((
                    Text::new("(Exemple: Dupont Jean)"),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.6, 0.6, 0.6)),
                    Node {
                        margin: UiRect::top(Val::Px(5.0)),
                        ..default()
                    },
                ));
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
                    Text::new("Commencer"),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.6, 0.6, 0.6)),
                ));
            });

            // Message d'erreur (invisible au début)
            parent.spawn((
                Text::new("Veuillez entrer votre nom et prenom"),
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

// Système pour parser le nom complet et valider
fn parse_full_name(full_name: &str) -> Option<(String, String)> {
    let trimmed = full_name.trim();
    if trimmed.is_empty() {
        return None;
    }

    // Séparer par espaces
    let parts: Vec<&str> = trimmed.split_whitespace().collect();

    if parts.len() < 2 {
        // S'il n'y a qu'un mot, on considère que c'est invalide
        return None;
    }

    // Le premier mot est le nom, le reste est le prénom
    let last_name = parts[0].to_string();
    let first_name = parts[1..].join(" ");

    Some((last_name, first_name))
}

// Système pour gérer la validation avec Enter
pub fn handle_submit_events(
    mut submit_events: EventReader<TextInputSubmitEvent>,
    mut player_info: ResMut<PlayerInfo>,
    mut next_state: ResMut<NextState<GameState>>,
    name_query: Query<&TextInputValue, With<NameInput>>,
    mut error_visibility: Query<&mut Visibility, With<ErrorMessage>>,
) {
    for _event in submit_events.read() {
        let Ok(name_value) = name_query.single() else { continue };

        if let Some((last_name, first_name)) = parse_full_name(&name_value.0) {
            player_info.last_name = last_name;
            player_info.first_name = first_name;
            info!("Validation par Enter - Informations du candidat: {} {}", 
                  player_info.last_name, player_info.first_name);
            next_state.set(GameState::Loading);
        } else {
            // Afficher le message d'erreur
            if let Ok(mut visibility) = error_visibility.single_mut() {
                *visibility = Visibility::Visible;
            }
        }
    }
}

// Système pour gérer la validation par bouton
pub fn handle_player_info_validation(
    mut player_info: ResMut<PlayerInfo>,
    mut next_state: ResMut<NextState<GameState>>,
    button_query: Query<&Interaction, (Changed<Interaction>, With<SubmitButton>)>,
    name_query: Query<&TextInputValue, With<NameInput>>,
    mut button_colors: Query<&mut BackgroundColor, With<SubmitButton>>,
    mut error_visibility: Query<&mut Visibility, With<ErrorMessage>>,
) {
    let Ok(name_value) = name_query.single() else { return };

    let can_submit = parse_full_name(&name_value.0).is_some();

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
            if let Some((last_name, first_name)) = parse_full_name(&name_value.0) {
                player_info.last_name = last_name;
                player_info.first_name = first_name;
                info!("Informations du candidat: {} {}", 
                      player_info.last_name, player_info.first_name);
                next_state.set(GameState::Loading);
            } else {
                // Afficher le message d'erreur
                if let Ok(mut visibility) = error_visibility.single_mut() {
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