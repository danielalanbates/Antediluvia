use bevy::prelude::*;
use antediluvia_core::combat::{CombatAction, get_skill_chains};
use antediluvia_core::entity::Job;
use std::collections::HashMap;
use crate::mob_ai::MobBrain;
use crate::inventory::{Satchel, InventoryItem};

#[derive(Component, Debug, Clone)]
pub struct PlayerCombat {
    pub health: f32,
    pub max_health: f32,
    pub job: Job,
    pub active_cooldowns: HashMap<CombatAction, f32>,
    pub is_in_combat: bool,
    pub current_target: Option<Entity>,
    pub level: u32,
    pub experience: f32,
    pub xp_to_next_level: f32,
    pub damage_multiplier: f32,
    pub is_dead: bool,
    pub respawn_timer: f32,
    pub last_action: Option<CombatAction>,
    pub combo_window: f32,
}

impl PlayerCombat {
    pub fn new(job: Job) -> Self {
        Self {
            health: 100.0,
            max_health: 100.0,
            job,
            active_cooldowns: HashMap::new(),
            is_in_combat: false,
            current_target: None,
            level: 1,
            experience: 0.0,
            xp_to_next_level: 100.0,
            damage_multiplier: 1.0,
            is_dead: false,
            respawn_timer: 0.0,
            last_action: None,
            combo_window: 0.0,
        }
    }

    pub fn get_cooldown_remaining(&self, action: CombatAction) -> f32 {
        self.active_cooldowns.get(&action).copied().unwrap_or(0.0)
    }

    pub fn can_use_action(&self, action: CombatAction) -> bool {
        !self.is_dead && self.get_cooldown_remaining(action) <= 0.0
    }

    pub fn take_damage(&mut self, damage: f32) {
        if self.is_dead {
            return;
        }
        self.health = (self.health - damage).max(0.0);
        if self.health <= 0.0 {
            self.is_dead = true;
            self.respawn_timer = 5.0;
        }
    }

    pub fn heal(&mut self, amount: f32) {
        if self.is_dead {
            return;
        }
        self.health = (self.health + amount).min(self.max_health);
    }

    pub fn award_xp(&mut self, amount: f32) {
        self.experience += amount;
        while self.experience >= self.xp_to_next_level {
            self.experience -= self.xp_to_next_level;
            self.level += 1;
            self.max_health += 15.0;
            self.health = self.max_health;
            self.damage_multiplier += 0.05;
            self.xp_to_next_level = 100.0 * (1.0 + self.level as f32 * 0.5);
            println!(
                "LEVEL UP! You are now level {}. Max HP: {:.0}, Damage: {:.0}%",
                self.level,
                self.max_health,
                self.damage_multiplier * 100.0
            );
        }
    }

    pub fn respawn(&mut self) {
        self.health = self.max_health;
        self.is_dead = false;
        self.respawn_timer = 0.0;
        self.is_in_combat = false;
        self.current_target = None;
        self.last_action = None;
        self.combo_window = 0.0;
    }
}

#[derive(Component, Debug, Clone)]
pub struct Mob {
    pub health: f32,
    pub max_health: f32,
    pub name: String,
    pub level: u32,
    pub damage_per_hit: f32,
    pub xp_reward: f32,
    pub mob_tier: MobTier,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MobTier {
    Common,
    Elite,
    Boss,
}

impl Mob {
    pub fn wolf(level: u32) -> Self {
        Self {
            health: 50.0 + level as f32 * 10.0,
            max_health: 50.0 + level as f32 * 10.0,
            name: "Wolf".to_string(),
            level,
            damage_per_hit: 8.0 + level as f32 * 2.0,
            xp_reward: 25.0 + level as f32 * 10.0,
            mob_tier: MobTier::Common,
        }
    }

    pub fn lion(level: u32) -> Self {
        Self {
            health: 100.0 + level as f32 * 20.0,
            max_health: 100.0 + level as f32 * 20.0,
            name: "Lion".to_string(),
            level,
            damage_per_hit: 15.0 + level as f32 * 3.0,
            xp_reward: 50.0 + level as f32 * 15.0,
            mob_tier: MobTier::Common,
        }
    }

    pub fn chimera(level: u32) -> Self {
        Self {
            health: 200.0 + level as f32 * 30.0,
            max_health: 200.0 + level as f32 * 30.0,
            name: "Chimera".to_string(),
            level,
            damage_per_hit: 25.0 + level as f32 * 5.0,
            xp_reward: 100.0 + level as f32 * 25.0,
            mob_tier: MobTier::Elite,
        }
    }

    pub fn corrupted(level: u32) -> Self {
        Self {
            health: 150.0 + level as f32 * 25.0,
            max_health: 150.0 + level as f32 * 25.0,
            name: "Corrupted".to_string(),
            level,
            damage_per_hit: 20.0 + level as f32 * 4.0,
            xp_reward: 75.0 + level as f32 * 20.0,
            mob_tier: MobTier::Elite,
        }
    }

    pub fn nephilim(level: u32) -> Self {
        Self {
            health: 500.0 + level as f32 * 50.0,
            max_health: 500.0 + level as f32 * 50.0,
            name: "Nephilim".to_string(),
            level,
            damage_per_hit: 40.0 + level as f32 * 8.0,
            xp_reward: 250.0 + level as f32 * 50.0,
            mob_tier: MobTier::Boss,
        }
    }

    pub fn take_damage(&mut self, damage: f32) {
        self.health = (self.health - damage).max(0.0);
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0.0
    }
}

#[derive(Component)]
pub struct CombatIndicator;

#[derive(Component)]
pub struct DamageNumber {
    pub value: f32,
    pub timer: f32,
    pub is_heal: bool,
}

impl DamageNumber {
    const LIFETIME: f32 = 1.5;

    pub fn new(value: f32, is_heal: bool) -> Self {
        Self {
            value,
            timer: Self::LIFETIME,
            is_heal,
        }
    }
}

#[derive(Resource, Default)]
pub struct ChainNotification {
    pub chain_name: String,
    pub timer: f32,
}

pub fn update_cooldowns(
    mut query: Query<&mut PlayerCombat>,
    mut chain_notif: ResMut<ChainNotification>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    for mut combat in query.iter_mut() {
        for cooldown in combat.active_cooldowns.values_mut() {
            *cooldown -= dt;
        }
        combat.active_cooldowns.retain(|_, v| *v > 0.0);
        combat.combo_window = (combat.combo_window - dt).max(0.0);
        if combat.combo_window <= 0.0 {
            combat.last_action = None;
        }
    }
    if chain_notif.timer > 0.0 {
        chain_notif.timer -= dt;
    }
}

pub fn player_respawn_system(
    mut player_q: Query<(&mut PlayerCombat, &mut Transform)>,
    time: Res<Time>,
) {
    let Ok((mut combat, mut transform)) = player_q.single_mut() else {
        return;
    };

    if combat.is_dead {
        combat.respawn_timer -= time.delta_secs();
        if combat.respawn_timer <= 0.0 {
            combat.respawn();
            transform.translation = Vec3::new(0.0, 5.0, 100.0);
            println!("You have respawned at the Eden Pillar.");
        }
    }
}

pub fn combat_input_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player_q: Query<(&mut PlayerCombat, &Transform)>,
    mut mob_q: Query<(&mut Mob, &Transform, Entity), Without<PlayerCombat>>,
    mut satchel_q: Query<&mut Satchel>,
    mut chain_notif: ResMut<ChainNotification>,
    equipment: Res<crate::Equipment>,
) {
    let Ok((mut player_combat, player_transform)) = player_q.single_mut() else {
        return;
    };

    if player_combat.is_dead {
        return;
    }

    let player_pos = player_transform.translation;

    let action = if keys.just_pressed(KeyCode::Digit1) {
        Some(CombatAction::HunterThrust)
    } else if keys.just_pressed(KeyCode::Digit2) {
        Some(CombatAction::HunterSlash)
    } else if keys.just_pressed(KeyCode::Digit3) {
        Some(CombatAction::LeviteHeal)
    } else if keys.just_pressed(KeyCode::Digit4) {
        Some(CombatAction::ForgeSmash)
    } else {
        None
    };

    if let Some(action) = action {
        if !player_combat.can_use_action(action) {
            return;
        }

        // Heal targets self
        if action == CombatAction::LeviteHeal {
            let heal_amount = 50.0;
            player_combat.heal(heal_amount);

            commands.spawn((
                Text2d::new(format!("+{:.0}", heal_amount)),
                TextFont {
                    font: asset_server.load("FiraSans-Bold.ttf"),
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::srgb(0.2, 1.0, 0.2)),
                Transform::from_translation(player_pos + Vec3::Y * 20.0),
                DamageNumber::new(heal_amount, true),
            ));

            player_combat.active_cooldowns.insert(action, action.cooldown());
            println!("Healed for {:.0}! HP: {:.0}/{:.0}", heal_amount, player_combat.health, player_combat.max_health);
            return;
        }

        // Check for skill chain
        let mut chain_bonus = 1.0;
        if player_combat.combo_window > 0.0 {
            if let Some(last) = player_combat.last_action {
                for chain in get_skill_chains() {
                    if chain.matches(last, action) {
                        chain_bonus = chain.damage_multiplier;
                        chain_notif.chain_name = chain.name.clone();
                        chain_notif.timer = 2.0;
                        println!("SKILL CHAIN: {} ({:.1}x damage!)", chain.name, chain.damage_multiplier);
                        break;
                    }
                }
            }
        }
        player_combat.last_action = Some(action);
        player_combat.combo_window = 3.0;

        // Find closest alive mob as target
        let mut closest_mob: Option<(Entity, f32)> = None;
        for (mob, mob_transform, entity) in mob_q.iter() {
            if !mob.is_alive() {
                continue;
            }
            let distance = player_pos.distance(mob_transform.translation);
            if distance < 200.0 {
                if closest_mob.is_none() || distance < closest_mob.unwrap().1 {
                    closest_mob = Some((entity, distance));
                }
            }
        }

        if let Some((mob_entity, _)) = closest_mob {
            let damage = action.damage() * player_combat.damage_multiplier * chain_bonus + equipment.weapon_damage_bonus();
            let mut mob_died = false;
            let mut mob_name = String::new();
            let mut mob_xp = 0.0;
            let mut mob_level = 0u32;

            if let Ok((mut mob, mob_transform, _)) = mob_q.get_mut(mob_entity) {
                mob.take_damage(damage);
                mob_name = mob.name.clone();
                mob_xp = mob.xp_reward;
                mob_level = mob.level;
                println!("{} takes {:.0} damage! ({:.0}/{:.0} HP)", mob.name, damage, mob.health, mob.max_health);

                if !mob.is_alive() {
                    mob_died = true;
                    println!("{} defeated! +{:.0} XP", mob.name, mob.xp_reward);
                }

                // Spawn damage number
                commands.spawn((
                    Text2d::new(format!("{:.0}", damage)),
                    TextFont {
                        font: asset_server.load("FiraSans-Bold.ttf"),
                        font_size: 28.0,
                        ..default()
                    },
                    TextColor(Color::srgb(1.0, 0.3, 0.1)),
                    Transform::from_translation(mob_transform.translation + Vec3::Y * 15.0),
                    DamageNumber::new(damage, false),
                ));
            }

            if mob_died {
                player_combat.award_xp(mob_xp);
                player_combat.is_in_combat = false;
                player_combat.current_target = None;

                // Loot drop
                let loot = get_loot_for_mob(&mob_name, mob_level);
                if !loot.is_empty() {
                    if let Ok(mut satchel) = satchel_q.single_mut() {
                        for item in &loot {
                            if satchel.add_item(item.clone()) {
                                println!("  Loot: {} x{}", item.name, item.quantity);
                            } else {
                                println!("  Satchel full! {} dropped on the ground.", item.name);
                            }
                        }
                    }
                }
            } else {
                player_combat.is_in_combat = true;
                player_combat.current_target = Some(mob_entity);
            }

            player_combat.active_cooldowns.insert(action, action.cooldown());
        }
    }
}

fn get_loot_for_mob(mob_name: &str, level: u32) -> Vec<InventoryItem> {
    let mut loot = Vec::new();

    match mob_name {
        "Wolf" => {
            loot.push(InventoryItem {
                name: "Wolf Pelt".to_string(),
                quantity: 1,
                weight: 3.0,
            });
            if level >= 2 {
                loot.push(InventoryItem {
                    name: "Wolf Fang".to_string(),
                    quantity: 1,
                    weight: 0.5,
                });
            }
        }
        "Lion" => {
            loot.push(InventoryItem {
                name: "Lion Mane".to_string(),
                quantity: 1,
                weight: 4.0,
            });
            loot.push(InventoryItem {
                name: "Raw Meat".to_string(),
                quantity: 2,
                weight: 1.0,
            });
        }
        "Chimera" => {
            loot.push(InventoryItem {
                name: "Chimera Scale".to_string(),
                quantity: 1,
                weight: 5.0,
            });
            loot.push(InventoryItem {
                name: "Bronze Ingot".to_string(),
                quantity: 1,
                weight: 4.0,
            });
        }
        "Corrupted" => {
            loot.push(InventoryItem {
                name: "Dark Essence".to_string(),
                quantity: 1,
                weight: 1.0,
            });
            loot.push(InventoryItem {
                name: "Iron Ingot".to_string(),
                quantity: 1,
                weight: 5.0,
            });
        }
        "Nephilim" => {
            loot.push(InventoryItem {
                name: "Giant's Bone".to_string(),
                quantity: 1,
                weight: 10.0,
            });
            loot.push(InventoryItem {
                name: "Ancient Relic".to_string(),
                quantity: 1,
                weight: 2.0,
            });
            loot.push(InventoryItem {
                name: "Leather Grip".to_string(),
                quantity: 2,
                weight: 1.0,
            });
        }
        _ => {}
    }

    loot
}

pub fn spawn_mobs(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    struct MobSpawn {
        pos: Vec3,
        mob: Mob,
        color: Color,
        radius: f32,
        aggro: f32,
        speed: f32,
    }

    let spawns = vec![
        // Wolf pack near player start
        MobSpawn { pos: Vec3::new(50.0, 5.0, 50.0), mob: Mob::wolf(1), color: Color::srgb(0.6, 0.4, 0.3), radius: 3.0, aggro: 50.0, speed: 25.0 },
        MobSpawn { pos: Vec3::new(80.0, 5.0, 60.0), mob: Mob::wolf(1), color: Color::srgb(0.55, 0.35, 0.25), radius: 3.0, aggro: 50.0, speed: 25.0 },
        MobSpawn { pos: Vec3::new(65.0, 5.0, 80.0), mob: Mob::wolf(2), color: Color::srgb(0.5, 0.3, 0.2), radius: 3.5, aggro: 55.0, speed: 27.0 },
        // Wolves south
        MobSpawn { pos: Vec3::new(-60.0, 5.0, -80.0), mob: Mob::wolf(2), color: Color::srgb(0.5, 0.35, 0.25), radius: 3.0, aggro: 50.0, speed: 25.0 },
        MobSpawn { pos: Vec3::new(-40.0, 5.0, -100.0), mob: Mob::wolf(1), color: Color::srgb(0.6, 0.4, 0.3), radius: 3.0, aggro: 50.0, speed: 25.0 },
        // Lion pride east
        MobSpawn { pos: Vec3::new(160.0, 5.0, 20.0), mob: Mob::lion(2), color: Color::srgb(0.85, 0.7, 0.3), radius: 5.0, aggro: 75.0, speed: 22.0 },
        MobSpawn { pos: Vec3::new(180.0, 5.0, -10.0), mob: Mob::lion(3), color: Color::srgb(0.8, 0.65, 0.25), radius: 5.5, aggro: 75.0, speed: 22.0 },
        // Corrupted west
        MobSpawn { pos: Vec3::new(-150.0, 5.0, 50.0), mob: Mob::corrupted(3), color: Color::srgb(0.4, 0.0, 0.5), radius: 5.0, aggro: 80.0, speed: 20.0 },
        MobSpawn { pos: Vec3::new(-170.0, 5.0, 80.0), mob: Mob::corrupted(4), color: Color::srgb(0.35, 0.0, 0.45), radius: 5.5, aggro: 80.0, speed: 20.0 },
        // Chimera far southeast
        MobSpawn { pos: Vec3::new(120.0, 5.0, -150.0), mob: Mob::chimera(4), color: Color::srgb(0.9, 0.1, 0.5), radius: 6.0, aggro: 100.0, speed: 18.0 },
        // Nephilim boss far north
        MobSpawn { pos: Vec3::new(0.0, 8.0, -250.0), mob: Mob::nephilim(5), color: Color::srgb(0.3, 0.0, 0.0), radius: 10.0, aggro: 200.0, speed: 15.0 },
    ];

    for s in spawns {
        let brain = MobBrain::new(s.aggro, s.speed, s.pos);

        let (body_mesh, body_scale, head_offset, head_size) = match s.mob.mob_tier {
            MobTier::Common => (
                meshes.add(Capsule3d::new(s.radius * 0.5, s.radius * 0.4)),
                Vec3::new(1.2, 1.0, 1.0),
                Vec3::new(0.0, s.radius * 0.2, s.radius * 0.5),
                s.radius * 0.35,
            ),
            MobTier::Elite => (
                meshes.add(Capsule3d::new(s.radius * 0.45, s.radius * 0.8)),
                Vec3::ONE,
                Vec3::new(0.0, s.radius * 0.8, 0.0),
                s.radius * 0.3,
            ),
            MobTier::Boss => (
                meshes.add(Capsule3d::new(s.radius * 0.4, s.radius * 1.2)),
                Vec3::ONE,
                Vec3::new(0.0, s.radius * 1.3, 0.0),
                s.radius * 0.35,
            ),
        };

        let mob_mat = materials.add(StandardMaterial {
            base_color: s.color,
            metallic: 0.1,
            perceptual_roughness: 0.8,
            ..default()
        });

        commands.spawn((
            Mesh3d(body_mesh),
            MeshMaterial3d(mob_mat.clone()),
            Transform::from_translation(s.pos).with_scale(body_scale),
            s.mob,
            brain,
            Name::new("Mob"),
        )).with_children(|parent| {
            parent.spawn((
                Mesh3d(meshes.add(Sphere::new(head_size))),
                MeshMaterial3d(mob_mat),
                Transform::from_translation(head_offset),
            ));
        });
    }

    println!("Spawned 11 mobs across the world. Hunt them down!");
}

pub fn update_mob_health_display(
    mob_q: Query<(&Mob, &Transform), Changed<Mob>>,
) {
    for (mob, _transform) in mob_q.iter() {
        if !mob.is_alive() {
            return;
        }
        let health_percent = mob.health / mob.max_health;
        let bar_len = 10;
        let filled = (health_percent * bar_len as f32).ceil() as usize;
        let bar: String = "█".repeat(filled) + &"░".repeat(bar_len - filled);
        println!("{} Lv{}: [{}] {:.0}%", mob.name, mob.level, bar, health_percent * 100.0);
    }
}

pub fn update_damage_numbers(
    mut commands: Commands,
    mut query: Query<(Entity, &mut DamageNumber, &mut Transform)>,
    time: Res<Time>,
) {
    for (entity, mut damage_num, mut transform) in query.iter_mut() {
        damage_num.timer -= time.delta_secs();
        transform.translation.y += 30.0 * time.delta_secs();

        if damage_num.timer <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
