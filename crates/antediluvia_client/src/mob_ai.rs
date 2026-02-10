use bevy::prelude::*;
use crate::combat::{Mob, PlayerCombat};
use crate::player::PlayerCamera;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MobState {
    Idle,
    Patrol,
    Aggro,
    Attacking,
    Dead,
}

#[derive(Component, Debug)]
pub struct MobBrain {
    pub state: MobState,
    pub aggro_range: f32,
    pub attack_range: f32,
    pub attack_cooldown: f32,
    pub attack_timer: f32,
    pub move_speed: f32,
    pub patrol_target: Option<Vec3>,
    pub patrol_timer: f32,
    pub death_timer: f32,
    pub home_position: Vec3,
    pub leash_range: f32,
}

impl MobBrain {
    pub fn new(aggro_range: f32, move_speed: f32, home: Vec3) -> Self {
        Self {
            state: MobState::Idle,
            aggro_range,
            attack_range: 15.0,
            attack_cooldown: 2.0,
            attack_timer: 0.0,
            move_speed,
            patrol_target: None,
            patrol_timer: 0.0,
            death_timer: 3.0,
            home_position: home,
            leash_range: aggro_range * 3.0,
        }
    }
}

#[derive(Component)]
pub struct DeathEffect {
    pub timer: f32,
    pub original_scale: Vec3,
}

pub fn mob_ai_system(
    mut mob_q: Query<(&mut MobBrain, &mut Mob, &mut Transform, Entity), Without<PlayerCamera>>,
    player_q: Query<&Transform, With<PlayerCamera>>,
    time: Res<Time>,
) {
    let Ok(player_transform) = player_q.single() else {
        return;
    };
    let player_pos = player_transform.translation;
    let dt = time.delta_secs();

    for (mut brain, mob, mut transform, _entity) in mob_q.iter_mut() {
        if !mob.is_alive() {
            brain.state = MobState::Dead;
            continue;
        }

        let mob_pos = transform.translation;
        let distance_to_player = mob_pos.distance(player_pos);
        let distance_to_home = mob_pos.distance(brain.home_position);

        match brain.state {
            MobState::Idle => {
                brain.patrol_timer -= dt;
                if brain.patrol_timer <= 0.0 {
                    let offset = Vec3::new(
                        (rand_simple(mob_pos.x + time.elapsed_secs()) - 0.5) * 60.0,
                        0.0,
                        (rand_simple(mob_pos.z + time.elapsed_secs()) - 0.5) * 60.0,
                    );
                    brain.patrol_target = Some(brain.home_position + offset);
                    brain.patrol_timer = 3.0 + rand_simple(mob_pos.x) * 4.0;
                    brain.state = MobState::Patrol;
                }

                if distance_to_player < brain.aggro_range {
                    brain.state = MobState::Aggro;
                }
            }

            MobState::Patrol => {
                if let Some(target) = brain.patrol_target {
                    let dir = (target - mob_pos).normalize_or_zero();
                    transform.translation += dir * brain.move_speed * 0.5 * dt;
                    transform.translation.y = 5.0;

                    if mob_pos.distance(target) < 5.0 {
                        brain.patrol_target = None;
                        brain.state = MobState::Idle;
                        brain.patrol_timer = 2.0 + rand_simple(mob_pos.z) * 3.0;
                    }
                } else {
                    brain.state = MobState::Idle;
                }

                if distance_to_player < brain.aggro_range {
                    brain.state = MobState::Aggro;
                }
            }

            MobState::Aggro => {
                if distance_to_home > brain.leash_range {
                    brain.state = MobState::Patrol;
                    brain.patrol_target = Some(brain.home_position);
                    return;
                }

                if distance_to_player > brain.aggro_range * 1.5 {
                    brain.state = MobState::Idle;
                    brain.patrol_timer = 1.0;
                    return;
                }

                let dir = (player_pos - mob_pos).normalize_or_zero();
                transform.translation += dir * brain.move_speed * dt;
                transform.translation.y = 5.0;

                let look_target = Vec3::new(player_pos.x, transform.translation.y, player_pos.z);
                transform.look_at(look_target, Vec3::Y);

                if distance_to_player < brain.attack_range {
                    brain.state = MobState::Attacking;
                }
            }

            MobState::Attacking => {
                if distance_to_player > brain.attack_range * 1.5 {
                    brain.state = MobState::Aggro;
                    return;
                }

                brain.attack_timer -= dt;

                let dir = (player_pos - mob_pos).normalize_or_zero();
                transform.translation += dir * brain.move_speed * 0.3 * dt;
                transform.translation.y = 5.0;
            }

            MobState::Dead => {}
        }
    }
}

pub fn mob_attack_system(
    mut mob_q: Query<(&mut MobBrain, &Mob, &Transform), Without<PlayerCamera>>,
    mut player_q: Query<(&mut PlayerCombat, &Transform), With<PlayerCamera>>,
    time: Res<Time>,
) {
    let Ok((mut player_combat, player_transform)) = player_q.single_mut() else {
        return;
    };
    let player_pos = player_transform.translation;
    let dt = time.delta_secs();

    for (mut brain, mob, mob_transform) in mob_q.iter_mut() {
        if brain.state != MobState::Attacking {
            continue;
        }
        if !mob.is_alive() {
            continue;
        }

        brain.attack_timer -= dt;
        if brain.attack_timer <= 0.0 {
            let distance = mob_transform.translation.distance(player_pos);
            if distance < brain.attack_range * 1.5 {
                player_combat.take_damage(mob.damage_per_hit);
                println!(
                    "{} attacks you for {:.0} damage! HP: {:.0}/{:.0}",
                    mob.name, mob.damage_per_hit, player_combat.health, player_combat.max_health
                );
            }
            brain.attack_timer = brain.attack_cooldown;
        }
    }
}

pub fn mob_death_system(
    mut commands: Commands,
    mut mob_q: Query<(Entity, &Mob, &MobBrain, &mut Transform), Without<DeathEffect>>,
) {
    for (entity, mob, brain, mut transform) in mob_q.iter_mut() {
        if !mob.is_alive() && brain.state == MobState::Dead {
            commands.entity(entity).insert(DeathEffect {
                timer: 30.0,
                original_scale: transform.scale,
            });
            transform.translation.y -= 0.5;
        }
    }
}

pub fn death_effect_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut DeathEffect, &mut Transform, &mut Mob, &mut MobBrain)>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    for (entity, mut effect, mut transform, mut mob, mut brain) in query.iter_mut() {
        effect.timer -= dt;

        if effect.timer > 28.0 {
            // First 2 seconds: shrink animation
            let progress = 1.0 - ((effect.timer - 28.0) / 2.0).clamp(0.0, 1.0);
            transform.scale = effect.original_scale * (1.0 - progress);
        } else if effect.timer > 0.0 {
            // Waiting to respawn - stay invisible
            transform.scale = Vec3::ZERO;
        } else {
            // Respawn at home position
            mob.health = mob.max_health;
            brain.state = MobState::Idle;
            brain.patrol_timer = 3.0;
            brain.attack_timer = 0.0;
            transform.translation = brain.home_position;
            transform.scale = effect.original_scale;
            commands.entity(entity).remove::<DeathEffect>();
            println!("A {} has respawned!", mob.name);
        }
    }
}

fn rand_simple(seed: f32) -> f32 {
    let x = (seed * 12.9898 + 78.233).sin() * 43758.5453;
    x - x.floor()
}
