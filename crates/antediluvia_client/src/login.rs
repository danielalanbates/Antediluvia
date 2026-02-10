//! Simple login UI placeholder.
//! Press Enter to enter the world. Hooks into AppState::Login -> InWorld.

use bevy::prelude::*;
use super::AppState;
use serde::{Deserialize, Serialize};

#[derive(Component)]
pub struct LoginRoot;

#[derive(Resource, Default)]
pub struct LoginStatus {
    message: String,
    connecting: bool,
}

#[derive(Serialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginResponse {
    token: String,
    player_id: u64,
}

pub fn spawn_login_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(LoginStatus {
        message: "Press Enter to Login".to_string(),
        connecting: false,
    });

    let font = asset_server.load("FiraSans-Bold.ttf");

    // Spawn a 2D camera for the UI
    commands.spawn((Camera2d, LoginRoot));

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
            LoginRoot,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Antediluvia Login\n\nPress Enter to enter the world.\n(This is a placeholder UI.)"),
                TextFont {
                    font,
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(Justify::Center),
                LoginStatusText,
            ));
        });
}

#[derive(Component)]
pub struct LoginStatusText;

pub fn despawn_login_ui(mut commands: Commands, root: Query<Entity, With<LoginRoot>>) {
    for entity in &root {
        commands.entity(entity).despawn();
    }
}

pub fn login_input_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut status: ResMut<LoginStatus>,
    mut text_q: Query<&mut Text, With<LoginStatusText>>,
    mut commands: Commands,
) {
    if status.connecting {
        return;
    }

    if keys.just_pressed(KeyCode::Enter) {
        status.connecting = true;
        status.message = "Connecting...".to_string();
        if let Ok(mut txt) = text_q.single_mut() {
            **txt = status.message.clone();
        }

        // Setup network connection - try online, fallback to offline
        if let Ok((client, transport)) = super::setup_network_connection(12345, "") {
            commands.insert_resource(client);
            commands.insert_resource(transport);
            info!("Connected to server");
            next_state.set(AppState::InWorld);
        } else if let Ok((client, transport)) = super::setup_offline_connection(999999) {
            commands.insert_resource(client);
            commands.insert_resource(transport);
            info!("Running in offline mode");
            next_state.set(AppState::InWorld);
        } else {
            status.connecting = false;
            status.message = "Connection failed. Try again.".to_string();
            if let Ok(mut txt) = text_q.single_mut() {
                **txt = status.message.clone();
            }
        }
    }
}
