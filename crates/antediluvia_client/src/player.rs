use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::window::{CursorGrabMode, CursorOptions};
use crate::combat::PlayerCombat;

/// Marker component for the player entity (body mesh + combat).
/// Used by all systems to identify the player.
#[derive(Component)]
pub struct PlayerCamera;

/// Component for the camera that follows the player in third-person.
#[derive(Component)]
pub struct FollowCamera {
    pub offset: Vec3,
    pub pitch: f32,
}

impl Default for FollowCamera {
    fn default() -> Self {
        Self {
            offset: Vec3::new(0.0, 8.0, 14.0),
            pitch: -0.2,
        }
    }
}

const BASE_SPEED: f32 = 50.0;
const SPRINT_MULTIPLIER: f32 = 3.0;

pub fn player_movement_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, Option<&PlayerCombat>), With<PlayerCamera>>,
) {
    let Ok((mut transform, combat)) = query.single_mut() else {
        return;
    };

    if let Some(combat) = combat {
        if combat.is_dead { return; }
    }

    let is_sprinting = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);
    let speed_mult = if is_sprinting { SPRINT_MULTIPLIER } else { 1.0 };
    let speed = BASE_SPEED * speed_mult * time.delta_secs();

    let forward = transform.forward();
    let forward_flat = Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();
    let right_flat = Vec3::new(-forward.z, 0.0, forward.x).normalize_or_zero();

    let mut movement = Vec3::ZERO;
    if keyboard.pressed(KeyCode::KeyW) { movement += forward_flat; }
    if keyboard.pressed(KeyCode::KeyS) { movement -= forward_flat; }
    if keyboard.pressed(KeyCode::KeyA) { movement -= right_flat; }
    if keyboard.pressed(KeyCode::KeyD) { movement += right_flat; }

    if movement != Vec3::ZERO {
        transform.translation += movement.normalize() * speed;
    }

    // Keep player on ground
    transform.translation.y = 5.0;
}

pub fn player_look_system(
    mut mouse_motion: MessageReader<MouseMotion>,
    mut body_q: Query<&mut Transform, With<PlayerCamera>>,
    mut cam_q: Query<&mut FollowCamera>,
) {
    let Ok(mut body) = body_q.single_mut() else {
        return;
    };
    let Ok(mut follow) = cam_q.single_mut() else {
        return;
    };
    let sensitivity = 0.003;

    for motion in mouse_motion.read() {
        let (yaw, _, _) = body.rotation.to_euler(EulerRot::YXZ);
        let new_yaw = yaw - motion.delta.x * sensitivity;
        body.rotation = Quat::from_rotation_y(new_yaw);

        follow.pitch -= motion.delta.y * sensitivity;
        follow.pitch = follow.pitch.clamp(-1.0, 0.5);
    }
}

pub fn camera_follow_system(
    body_q: Query<&Transform, With<PlayerCamera>>,
    mut cam_q: Query<(&mut Transform, &FollowCamera), Without<PlayerCamera>>,
) {
    let Ok(body) = body_q.single() else {
        return;
    };
    let Ok((mut cam_t, follow)) = cam_q.single_mut() else {
        return;
    };

    let pitch_rot = Quat::from_rotation_x(follow.pitch);
    let offset = body.rotation * pitch_rot * follow.offset;
    cam_t.translation = body.translation + offset;
    cam_t.look_at(body.translation + Vec3::Y * 4.0, Vec3::Y);
}

pub fn cursor_grab_system(
    mut cursor_q: Query<&mut CursorOptions>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    let Ok(mut cursor) = cursor_q.single_mut() else {
        return;
    };

    if mouse.just_pressed(MouseButton::Left) {
        cursor.grab_mode = CursorGrabMode::Locked;
        cursor.visible = false;
    }

    if keys.just_pressed(KeyCode::Escape) {
        cursor.grab_mode = CursorGrabMode::None;
        cursor.visible = true;
    }
}
