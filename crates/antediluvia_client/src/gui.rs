use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use crate::AppState;
use crate::combat::{PlayerCombat, Mob, MobTier};
use crate::player::PlayerCamera;
use crate::npc::{NPCEntity, NPCInteraction};
use crate::inventory::{Satchel, InventoryItem};
use crate::{WorldState, CraftingRes, Equipment, DayNightCycle};
use crate::combat::ChainNotification;
use crate::gathering::GatheringNode;
use crate::graphics_settings::{GraphicsSettings, QualityTier};
use antediluvia_core::combat::CombatAction;
use antediluvia_core::world::FloodStage;

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default())
           .init_resource::<GuiState>()
           .add_systems(Update, (
               dev_panel_system,
               hud_system,
               target_info_system,
               npc_dialogue_system,
               crafting_panel_system,
               map_overlay_system,
               equipment_panel_system,
               gathering_prompt_system,
               graphics_settings_panel_system,
           ));
    }
}

#[derive(Resource)]
pub struct GuiState {
    pub show_dev_panel: bool,
    pub show_hud: bool,
    pub show_crafting: bool,
    pub show_map: bool,
    pub show_equipment: bool,
    pub show_graphics: bool,
}

impl Default for GuiState {
    fn default() -> Self {
        Self {
            show_dev_panel: false,
            show_hud: true,
            show_crafting: false,
            show_map: false,
            show_equipment: false,
            show_graphics: false,
        }
    }
}

// ─── Dev Panel ──────────────────────────────────────────

fn dev_panel_system(
    mut contexts: EguiContexts,
    mut gui_state: ResMut<GuiState>,
    diagnostics: Res<bevy::diagnostic::DiagnosticsStore>,
    app_state: Res<State<AppState>>,
    keys: Res<ButtonInput<KeyCode>>,
    player_q: Query<(&PlayerCombat, &Transform), With<PlayerCamera>>,
    world_state: Option<Res<WorldState>>,
) {
    if keys.just_pressed(KeyCode::F3) {
        gui_state.show_dev_panel = !gui_state.show_dev_panel;
    }
    if !gui_state.show_dev_panel { return; }

    egui::Window::new("Developer Panel")
        .default_pos([10.0, 10.0])
        .show(contexts.ctx_mut().unwrap(), |ui| {
            ui.heading("Debug Info");
            if let Some(fps) = diagnostics.get(&bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(value) = fps.smoothed() {
                    ui.label(format!("FPS: {:.1}", value));
                }
            }

            ui.separator();
            ui.label(format!("State: {:?}", app_state.get()));

            if let Ok((combat, transform)) = player_q.single() {
                ui.separator();
                ui.heading("Player");
                ui.label(format!("Pos: ({:.0}, {:.0}, {:.0})", transform.translation.x, transform.translation.y, transform.translation.z));
                ui.label(format!("Level: {} | XP: {:.0}/{:.0}", combat.level, combat.experience, combat.xp_to_next_level));
                ui.label(format!("HP: {:.0}/{:.0}", combat.health, combat.max_health));
                ui.label(format!("Damage: {:.0}%", combat.damage_multiplier * 100.0));
                ui.label(format!("In Combat: {}", combat.is_in_combat));
            }

            if let Some(ws) = &world_state {
                ui.separator();
                ui.heading("World");
                ui.label(format!("Corruption: {:.1}%", ws.corruption));
                ui.label(format!("Flood Stage: {:?}", ws.flood_stage));
            }
        });
}

// ─── HUD ────────────────────────────────────────────────

fn hud_system(
    mut contexts: EguiContexts,
    gui_state: Res<GuiState>,
    app_state: Res<State<AppState>>,
    player_q: Query<&PlayerCombat, With<PlayerCamera>>,
    world_state: Option<Res<WorldState>>,
    chain_notif: Res<ChainNotification>,
    day_night: Option<Res<DayNightCycle>>,
) {
    if !gui_state.show_hud || *app_state.get() != AppState::InWorld { return; }

    let Ok(ctx) = contexts.ctx_mut() else { return; };
    let combat = match player_q.single() {
        Ok(c) => c,
        Err(_) => return,
    };

    // Death screen
    if combat.is_dead {
        egui::Area::new("death_screen".into())
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(egui::RichText::new("YOU HAVE FALLEN")
                        .size(48.0).color(egui::Color32::RED).strong());
                    ui.add_space(10.0);
                    ui.label(egui::RichText::new(format!("Respawning in {:.1}s...", combat.respawn_timer))
                        .size(24.0).color(egui::Color32::WHITE));
                });
            });
        return;
    }

    // Health bar - bottom center
    egui::Area::new("hud_health".into())
        .anchor(egui::Align2::CENTER_BOTTOM, [0.0, -60.0])
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                let hp_pct = combat.health / combat.max_health;
                let hp_color = if hp_pct > 0.5 {
                    egui::Color32::from_rgb(50, 200, 50)
                } else if hp_pct > 0.25 {
                    egui::Color32::from_rgb(230, 180, 30)
                } else {
                    egui::Color32::from_rgb(220, 40, 40)
                };
                let bar_width = 300.0;
                let bar_height = 20.0;
                let (rect, _) = ui.allocate_exact_size(egui::vec2(bar_width, bar_height), egui::Sense::hover());
                let painter = ui.painter();
                painter.rect_filled(rect, 4.0, egui::Color32::from_rgb(30, 30, 30));
                let filled = egui::Rect::from_min_size(rect.min, egui::vec2(bar_width * hp_pct, bar_height));
                painter.rect_filled(filled, 4.0, hp_color);
                painter.text(rect.center(), egui::Align2::CENTER_CENTER,
                    format!("HP: {:.0} / {:.0}", combat.health, combat.max_health),
                    egui::FontId::proportional(14.0), egui::Color32::WHITE);
            });
        });

    // Abilities - bottom center
    egui::Area::new("hud_abilities".into())
        .anchor(egui::Align2::CENTER_BOTTOM, [0.0, -20.0])
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                let abilities = [
                    (CombatAction::HunterThrust, "1: Thrust", 2.5f32),
                    (CombatAction::HunterSlash, "2: Slash", 2.0),
                    (CombatAction::LeviteHeal, "3: Heal", 4.0),
                    (CombatAction::ForgeSmash, "4: Smash", 3.5),
                ];
                for (action, label, max_cd) in &abilities {
                    let remaining = combat.get_cooldown_remaining(*action);
                    let ready = remaining <= 0.0;
                    let bg = if ready { egui::Color32::from_rgb(40, 80, 40) } else { egui::Color32::from_rgb(80, 40, 40) };
                    let size = egui::vec2(70.0, 30.0);
                    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
                    let painter = ui.painter();
                    painter.rect_filled(rect, 4.0, bg);
                    if !ready {
                        let cd_pct = remaining / max_cd;
                        let overlay = egui::Rect::from_min_size(rect.min, egui::vec2(size.x * cd_pct, size.y));
                        painter.rect_filled(overlay, 4.0, egui::Color32::from_black_alpha(120));
                    }
                    let text = if ready { label.to_string() } else { format!("{:.1}s", remaining) };
                    painter.text(rect.center(), egui::Align2::CENTER_CENTER, text,
                        egui::FontId::proportional(12.0), egui::Color32::WHITE);
                }
            });
        });

    // XP bar + Level - top center
    egui::Area::new("hud_xp".into())
        .anchor(egui::Align2::CENTER_TOP, [0.0, 10.0])
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(egui::RichText::new(format!("Level {} {:?}", combat.level, combat.job))
                    .size(16.0).color(egui::Color32::GOLD).strong());
                let xp_pct = combat.experience / combat.xp_to_next_level;
                let bar_width = 200.0;
                let bar_height = 8.0;
                let (rect, _) = ui.allocate_exact_size(egui::vec2(bar_width, bar_height), egui::Sense::hover());
                let painter = ui.painter();
                painter.rect_filled(rect, 2.0, egui::Color32::from_rgb(20, 20, 40));
                let filled = egui::Rect::from_min_size(rect.min, egui::vec2(bar_width * xp_pct, bar_height));
                painter.rect_filled(filled, 2.0, egui::Color32::from_rgb(100, 100, 255));
            });
        });

    // Corruption bar - top right
    if let Some(ws) = &world_state {
        egui::Area::new("hud_corruption".into())
            .anchor(egui::Align2::RIGHT_TOP, [-10.0, 10.0])
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    let stage_text = match ws.flood_stage {
                        FloodStage::Innocence => "Age of Innocence",
                        FloodStage::Violence => "Age of Violence",
                        FloodStage::Judgment => "Age of Judgment",
                        FloodStage::TheFlood => "THE FLOOD",
                    };
                    let stage_color = match ws.flood_stage {
                        FloodStage::Innocence => egui::Color32::from_rgb(100, 200, 100),
                        FloodStage::Violence => egui::Color32::from_rgb(200, 180, 50),
                        FloodStage::Judgment => egui::Color32::from_rgb(200, 100, 50),
                        FloodStage::TheFlood => egui::Color32::RED,
                    };
                    ui.label(egui::RichText::new(stage_text).size(13.0).color(stage_color).strong());

                    let c_pct = ws.corruption / 100.0;
                    let bar_width = 120.0;
                    let bar_height = 10.0;
                    let (rect, _) = ui.allocate_exact_size(egui::vec2(bar_width, bar_height), egui::Sense::hover());
                    let painter = ui.painter();
                    painter.rect_filled(rect, 3.0, egui::Color32::from_rgb(20, 20, 20));
                    let filled = egui::Rect::from_min_size(rect.min, egui::vec2(bar_width * c_pct, bar_height));
                    let r = (50.0 + c_pct * 200.0).min(255.0) as u8;
                    let gb = (50.0 - c_pct * 50.0).max(0.0) as u8;
                    painter.rect_filled(filled, 3.0, egui::Color32::from_rgb(r, gb, gb));
                    painter.text(rect.center(), egui::Align2::CENTER_CENTER,
                        format!("{:.0}%", ws.corruption),
                        egui::FontId::proportional(9.0), egui::Color32::WHITE);
                });
            });
    }

    // Chain notification
    if chain_notif.timer > 0.0 {
        egui::Area::new("chain_notif".into())
            .anchor(egui::Align2::CENTER_CENTER, [0.0, -100.0])
            .show(ctx, |ui| {
                ui.label(egui::RichText::new(format!("SKILL CHAIN: {}", chain_notif.chain_name))
                    .size(28.0).color(egui::Color32::from_rgb(255, 200, 50)).strong());
            });
    }

    // Time of day
    if let Some(cycle) = &day_night {
        egui::Area::new("hud_time".into())
            .anchor(egui::Align2::LEFT_TOP, [10.0, 50.0])
            .show(ctx, |ui| {
                let hour = cycle.time_of_day as u32;
                let minute = ((cycle.time_of_day - hour as f32) * 60.0) as u32;
                let period = if hour >= 12 { "PM" } else { "AM" };
                let display_hour = if hour == 0 { 12 } else if hour > 12 { hour - 12 } else { hour };
                let time_color = if hour >= 6 && hour < 18 {
                    egui::Color32::from_rgb(200, 200, 150)
                } else {
                    egui::Color32::from_rgb(120, 120, 180)
                };
                ui.label(egui::RichText::new(format!("{:2}:{:02} {}", display_hour, minute, period))
                    .size(14.0).color(time_color));
            });
    }
}

// ─── Target Info ────────────────────────────────────────

fn target_info_system(
    mut contexts: EguiContexts,
    gui_state: Res<GuiState>,
    app_state: Res<State<AppState>>,
    player_q: Query<(&PlayerCombat, &Transform), With<PlayerCamera>>,
    mob_q: Query<(&Mob, &Transform), Without<PlayerCamera>>,
) {
    if !gui_state.show_hud || *app_state.get() != AppState::InWorld { return; }

    let (combat, player_transform) = match player_q.single() {
        Ok(c) => c,
        Err(_) => return,
    };
    if combat.is_dead { return; }

    let player_pos = player_transform.translation;
    let mut nearest: Option<(&Mob, f32)> = None;
    for (mob, mob_transform) in mob_q.iter() {
        if !mob.is_alive() { continue; }
        let dist = player_pos.distance(mob_transform.translation);
        if dist < 200.0 {
            if nearest.is_none() || dist < nearest.unwrap().1 {
                nearest = Some((mob, dist));
            }
        }
    }

    if let Some((mob, distance)) = nearest {
        let Ok(ctx) = contexts.ctx_mut() else { return; };
        egui::Area::new("target_info".into())
            .anchor(egui::Align2::CENTER_TOP, [0.0, 50.0])
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    let tier_color = match mob.mob_tier {
                        MobTier::Common => egui::Color32::WHITE,
                        MobTier::Elite => egui::Color32::from_rgb(180, 130, 255),
                        MobTier::Boss => egui::Color32::from_rgb(255, 80, 80),
                    };
                    ui.label(egui::RichText::new(format!("{} (Lv{})", mob.name, mob.level))
                        .size(16.0).color(tier_color).strong());

                    let hp_pct = mob.health / mob.max_health;
                    let bar_width = 180.0;
                    let bar_height = 12.0;
                    let (rect, _) = ui.allocate_exact_size(egui::vec2(bar_width, bar_height), egui::Sense::hover());
                    let painter = ui.painter();
                    painter.rect_filled(rect, 3.0, egui::Color32::from_rgb(40, 10, 10));
                    let filled = egui::Rect::from_min_size(rect.min, egui::vec2(bar_width * hp_pct, bar_height));
                    painter.rect_filled(filled, 3.0, egui::Color32::from_rgb(200, 30, 30));
                    painter.text(rect.center(), egui::Align2::CENTER_CENTER,
                        format!("{:.0}/{:.0}", mob.health, mob.max_health),
                        egui::FontId::proportional(10.0), egui::Color32::WHITE);

                    ui.label(egui::RichText::new(format!("{:.0}m", distance))
                        .size(11.0).color(egui::Color32::GRAY));
                });
            });
    }
}

// ─── NPC Dialogue ───────────────────────────────────────

fn npc_dialogue_system(
    mut contexts: EguiContexts,
    interaction: Res<NPCInteraction>,
    player_q: Query<&Transform, With<PlayerCamera>>,
    npc_q: Query<(&NPCEntity, &Transform)>,
) {
    let Ok(ctx) = contexts.ctx_mut() else { return; };

    if !interaction.is_open {
        // Show prompt when near NPC
        let player_pos = match player_q.single() {
            Ok(t) => t.translation,
            Err(_) => return,
        };
        let mut nearest_npc: Option<(&NPCEntity, f32)> = None;
        for (npc, npc_t) in npc_q.iter() {
            let dist = player_pos.distance(npc_t.translation);
            if dist < 30.0 {
                if nearest_npc.is_none() || dist < nearest_npc.unwrap().1 {
                    nearest_npc = Some((npc, dist));
                }
            }
        }
        if let Some((npc, _)) = nearest_npc {
            egui::Area::new("npc_prompt".into())
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 80.0])
                .show(ctx, |ui| {
                    ui.label(egui::RichText::new(format!("Press E to speak with {}", npc.npc.name))
                        .size(18.0).color(egui::Color32::YELLOW).strong());
                });
        }
        return;
    }

    // Active dialogue
    if interaction.current_line < interaction.dialogue_lines.len() {
        egui::Area::new("npc_dialogue".into())
            .anchor(egui::Align2::CENTER_BOTTOM, [0.0, -120.0])
            .show(ctx, |ui| {
                egui::Frame::none()
                    .fill(egui::Color32::from_black_alpha(210))
                    .inner_margin(16.0)
                    .rounding(8.0)
                    .show(ui, |ui| {
                        ui.set_max_width(500.0);
                        ui.label(egui::RichText::new(&interaction.npc_name)
                            .strong().size(18.0).color(egui::Color32::GOLD));
                        ui.add_space(6.0);
                        ui.label(egui::RichText::new(&interaction.dialogue_lines[interaction.current_line])
                            .size(15.0).color(egui::Color32::WHITE));
                        ui.add_space(8.0);
                        ui.label(egui::RichText::new(
                            format!("[{}/{}] SPACE: next | E: close",
                                interaction.current_line + 1, interaction.dialogue_lines.len()))
                            .size(11.0).color(egui::Color32::GRAY));
                    });
            });
    }
}

// ─── Crafting Panel ─────────────────────────────────────

fn crafting_panel_system(
    mut contexts: EguiContexts,
    mut gui_state: ResMut<GuiState>,
    keys: Res<ButtonInput<KeyCode>>,
    crafting: Option<Res<CraftingRes>>,
    player_q: Query<&PlayerCombat, With<PlayerCamera>>,
    mut satchel_q: Query<&mut Satchel>,
) {
    if keys.just_pressed(KeyCode::KeyC) {
        gui_state.show_crafting = !gui_state.show_crafting;
    }
    if !gui_state.show_crafting { return; }

    let crafting = match crafting {
        Some(c) => c,
        None => return,
    };

    let skill_level = player_q.single().map(|c| c.level * 5).unwrap_or(5);

    // Snapshot satchel for display
    let satchel_items: Vec<(String, u32)> = if let Ok(satchel) = satchel_q.single() {
        satchel.items.iter().map(|i| (i.name.clone(), i.quantity)).collect()
    } else {
        vec![]
    };

    // Collect owned recipe data
    struct RecipeInfo {
        name: String,
        corruption_cost: f32,
        skill_required: String,
        skill_level_required: u32,
        ingredients: Vec<(String, u32)>,
    }

    let recipe_data: Vec<RecipeInfo> = crafting.0
        .get_all_recipes()
        .into_iter()
        .map(|r| RecipeInfo {
            name: r.name.clone(),
            corruption_cost: r.corruption_cost,
            skill_required: r.recipe.skill_required.clone(),
            skill_level_required: r.recipe.skill_level_required,
            ingredients: r.recipe.ingredients.iter().map(|(k, v)| (k.clone(), *v)).collect(),
        })
        .collect();

    let mut craft_name: Option<String> = None;
    let Ok(ctx) = contexts.ctx_mut() else { return; };

    egui::Window::new("Forge of Tubal-Cain")
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .resizable(false)
        .collapsible(false)
        .min_width(320.0)
        .show(ctx, |ui| {
            ui.heading("Crafting");
            ui.separator();

            for recipe in &recipe_data {
                ui.group(|ui| {
                    ui.label(egui::RichText::new(&recipe.name).strong().size(16.0));

                    if recipe.corruption_cost > 0.0 {
                        ui.label(egui::RichText::new(format!("Corruption +{:.1}", recipe.corruption_cost))
                            .color(egui::Color32::from_rgb(200, 80, 80)));
                    }

                    let has_skill = skill_level >= recipe.skill_level_required;
                    let skill_color = if has_skill {
                        egui::Color32::from_rgb(80, 200, 80)
                    } else {
                        egui::Color32::from_rgb(200, 80, 80)
                    };
                    ui.label(egui::RichText::new(
                        format!("{} Lv{} (you: {})", recipe.skill_required, recipe.skill_level_required, skill_level))
                        .color(skill_color));

                    let mut can_craft = has_skill;
                    for (ingredient, needed) in &recipe.ingredients {
                        let have = satchel_items.iter()
                            .find(|(n, _)| n == ingredient)
                            .map(|(_, q)| *q)
                            .unwrap_or(0);
                        let enough = have >= *needed;
                        if !enough { can_craft = false; }
                        let color = if enough {
                            egui::Color32::from_rgb(80, 200, 80)
                        } else {
                            egui::Color32::from_rgb(200, 80, 80)
                        };
                        ui.label(egui::RichText::new(format!("  {} {}/{}", ingredient, have, needed)).color(color));
                    }

                    ui.add_space(2.0);
                    if can_craft {
                        if ui.button("Craft").clicked() {
                            craft_name = Some(recipe.name.clone());
                        }
                    } else {
                        ui.add_enabled(false, egui::Button::new("Missing requirements"));
                    }
                });
                ui.add_space(4.0);
            }

            ui.separator();
            ui.label(egui::RichText::new("Press C to close").size(11.0).color(egui::Color32::GRAY));
        });

    // Execute crafting
    if let Some(item_name) = craft_name {
        if let Some(recipe_item) = crafting.0.get_recipe(&item_name) {
            if let Ok(mut satchel) = satchel_q.single_mut() {
                for (ingredient, qty) in &recipe_item.recipe.ingredients {
                    satchel.remove_item(ingredient, *qty);
                }
                let weight = match item_name.as_str() {
                    "Bronze Sword" => 8.0,
                    "Iron Sword" => 10.0,
                    "Linen Tunic" => 3.0,
                    _ => 5.0,
                };
                satchel.add_item(InventoryItem {
                    name: item_name.clone(),
                    quantity: 1,
                    weight,
                });
                println!("Crafted: {}!", item_name);
            }
        }
    }
}

// ─── Map Overlay ────────────────────────────────────────

fn map_overlay_system(
    mut contexts: EguiContexts,
    mut gui_state: ResMut<GuiState>,
    keys: Res<ButtonInput<KeyCode>>,
    player_q: Query<&Transform, With<PlayerCamera>>,
    mob_q: Query<(&Mob, &Transform), Without<PlayerCamera>>,
    npc_q: Query<(&NPCEntity, &Transform)>,
) {
    if keys.just_pressed(KeyCode::KeyM) {
        gui_state.show_map = !gui_state.show_map;
    }
    if !gui_state.show_map { return; }

    let player_pos = match player_q.single() {
        Ok(t) => t.translation,
        Err(_) => return,
    };

    let Ok(ctx) = contexts.ctx_mut() else { return; };

    egui::Window::new("Map of Havilah")
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .resizable(false)
        .collapsible(false)
        .show(ctx, |ui| {
            let map_size = 300.0;
            let world_range = 300.0;

            let (rect, _) = ui.allocate_exact_size(egui::vec2(map_size, map_size), egui::Sense::hover());
            let painter = ui.painter_at(rect);

            // Background
            painter.rect_filled(rect, 4.0, egui::Color32::from_rgb(25, 45, 25));

            // Grid
            for i in 0..7 {
                let offset = i as f32 * map_size / 6.0;
                painter.line_segment(
                    [rect.min + egui::vec2(offset, 0.0), rect.min + egui::vec2(offset, map_size)],
                    egui::Stroke::new(0.5, egui::Color32::from_white_alpha(20)));
                painter.line_segment(
                    [rect.min + egui::vec2(0.0, offset), rect.min + egui::vec2(map_size, offset)],
                    egui::Stroke::new(0.5, egui::Color32::from_white_alpha(20)));
            }

            let to_map = |wx: f32, wz: f32| -> egui::Pos2 {
                let mx = ((wx + world_range) / (2.0 * world_range) * map_size).clamp(0.0, map_size);
                let mz = ((wz + world_range) / (2.0 * world_range) * map_size).clamp(0.0, map_size);
                rect.min + egui::vec2(mx, mz)
            };

            // Eden Pillar (center)
            painter.circle_filled(to_map(0.0, 0.0), 6.0, egui::Color32::GOLD);

            // Mobs (red dots)
            for (mob, t) in mob_q.iter() {
                if mob.is_alive() {
                    let pos = to_map(t.translation.x, t.translation.z);
                    if rect.contains(pos) {
                        let color = match mob.mob_tier {
                            MobTier::Common => egui::Color32::from_rgb(200, 60, 60),
                            MobTier::Elite => egui::Color32::from_rgb(180, 60, 200),
                            MobTier::Boss => egui::Color32::from_rgb(255, 30, 30),
                        };
                        painter.circle_filled(pos, 3.0, color);
                    }
                }
            }

            // NPCs (blue dots)
            for (_, t) in npc_q.iter() {
                let pos = to_map(t.translation.x, t.translation.z);
                if rect.contains(pos) {
                    painter.circle_filled(pos, 4.0, egui::Color32::from_rgb(80, 130, 255));
                }
            }

            // Player (green, on top)
            let pp = to_map(player_pos.x, player_pos.z);
            painter.circle_filled(pp, 5.0, egui::Color32::from_rgb(50, 255, 50));

            // Legend
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.colored_label(egui::Color32::from_rgb(50, 255, 50), "You");
                ui.colored_label(egui::Color32::from_rgb(80, 130, 255), "NPC");
                ui.colored_label(egui::Color32::from_rgb(200, 60, 60), "Mob");
                ui.colored_label(egui::Color32::GOLD, "Eden");
            });

            ui.label(egui::RichText::new(format!("Pos: ({:.0}, {:.0})", player_pos.x, player_pos.z))
                .size(11.0).color(egui::Color32::GRAY));
            ui.label(egui::RichText::new("Press M to close").size(11.0).color(egui::Color32::GRAY));
        });
}

// ─── Equipment Panel ────────────────────────────────────

fn equipment_panel_system(
    mut contexts: EguiContexts,
    mut gui_state: ResMut<GuiState>,
    keys: Res<ButtonInput<KeyCode>>,
    mut equipment: ResMut<Equipment>,
    mut satchel_q: Query<&mut Satchel>,
    mut player_q: Query<&mut PlayerCombat, With<PlayerCamera>>,
) {
    if keys.just_pressed(KeyCode::Tab) {
        gui_state.show_equipment = !gui_state.show_equipment;
    }
    if !gui_state.show_equipment { return; }

    // Snapshot equippable items from satchel
    let equippable: Vec<(String, bool)> = if let Ok(satchel) = satchel_q.single() {
        satchel.items.iter()
            .filter(|i| is_equippable(&i.name))
            .map(|i| (i.name.clone(), is_weapon(&i.name)))
            .collect()
    } else {
        vec![]
    };

    let mut equip_action: Option<(String, bool)> = None;
    let mut unequip_action: Option<bool> = None;

    let Ok(ctx) = contexts.ctx_mut() else { return; };

    egui::Window::new("Equipment")
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .resizable(false)
        .collapsible(false)
        .min_width(280.0)
        .show(ctx, |ui| {
            ui.heading("Equipped Gear");
            ui.separator();

            // Weapon slot
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Weapon:").strong());
                if let Some(w) = &equipment.weapon {
                    ui.label(egui::RichText::new(w.as_str()).color(egui::Color32::from_rgb(200, 200, 100)));
                    if ui.button("Unequip").clicked() {
                        unequip_action = Some(true);
                    }
                } else {
                    ui.label(egui::RichText::new("(empty)").color(egui::Color32::GRAY));
                }
            });

            // Armor slot
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Armor: ").strong());
                if let Some(a) = &equipment.armor {
                    ui.label(egui::RichText::new(a.as_str()).color(egui::Color32::from_rgb(100, 200, 200)));
                    if ui.button("Unequip").clicked() {
                        unequip_action = Some(false);
                    }
                } else {
                    ui.label(egui::RichText::new("(empty)").color(egui::Color32::GRAY));
                }
            });

            // Stats
            ui.separator();
            ui.label(format!("Weapon bonus: +{:.0} damage", equipment.weapon_damage_bonus()));
            ui.label(format!("Armor bonus: +{:.0} HP", equipment.armor_hp_bonus()));

            // Available items
            if !equippable.is_empty() {
                ui.separator();
                ui.heading("Available to Equip");
                for (name, is_w) in &equippable {
                    ui.horizontal(|ui| {
                        let slot = if *is_w { "Weapon" } else { "Armor" };
                        ui.label(name.as_str());
                        if ui.button(format!("Equip ({})", slot)).clicked() {
                            equip_action = Some((name.clone(), *is_w));
                        }
                    });
                }
            }

            ui.separator();
            ui.label(egui::RichText::new("Press Tab to close").size(11.0).color(egui::Color32::GRAY));
        });

    // Execute unequip
    if let Some(is_w) = unequip_action {
        let old_armor_bonus = equipment.armor_hp_bonus();
        let item_name = if is_w {
            equipment.weapon.take()
        } else {
            equipment.armor.take()
        };
        if let Some(name) = item_name {
            let weight = equip_weight(&name);
            if let Ok(mut satchel) = satchel_q.single_mut() {
                satchel.add_item(InventoryItem { name: name.clone(), quantity: 1, weight });
            }
            if !is_w {
                let new_armor_bonus = equipment.armor_hp_bonus();
                if let Ok(mut combat) = player_q.single_mut() {
                    combat.max_health += new_armor_bonus - old_armor_bonus;
                    combat.health = combat.health.min(combat.max_health);
                }
            }
            println!("Unequipped: {}", name);
        }
    }

    // Execute equip
    if let Some((name, is_w)) = equip_action {
        let old_armor_bonus = equipment.armor_hp_bonus();
        if let Ok(mut satchel) = satchel_q.single_mut() {
            satchel.remove_item(&name, 1);
            // Put old item back in satchel
            let old = if is_w {
                equipment.weapon.replace(name.clone())
            } else {
                equipment.armor.replace(name.clone())
            };
            if let Some(old_name) = old {
                let weight = equip_weight(&old_name);
                satchel.add_item(InventoryItem { name: old_name, quantity: 1, weight });
            }
        }
        if !is_w {
            let new_armor_bonus = equipment.armor_hp_bonus();
            if let Ok(mut combat) = player_q.single_mut() {
                combat.max_health += new_armor_bonus - old_armor_bonus;
                combat.health += (new_armor_bonus - old_armor_bonus).max(0.0);
            }
        }
        println!("Equipped: {}", name);
    }
}

fn is_equippable(name: &str) -> bool {
    matches!(name, "Bronze Sword" | "Iron Sword" | "Linen Tunic")
}

fn is_weapon(name: &str) -> bool {
    matches!(name, "Bronze Sword" | "Iron Sword")
}

fn equip_weight(name: &str) -> f32 {
    match name {
        "Bronze Sword" => 8.0,
        "Iron Sword" => 10.0,
        "Linen Tunic" => 3.0,
        _ => 5.0,
    }
}

// ─── Gathering Prompt ───────────────────────────────────

fn gathering_prompt_system(
    mut contexts: EguiContexts,
    player_q: Query<&Transform, With<PlayerCamera>>,
    node_q: Query<(&GatheringNode, &Transform, &Name), Without<PlayerCamera>>,
) {
    let player_pos = match player_q.single() {
        Ok(t) => t.translation,
        Err(_) => return,
    };

    let mut nearest: Option<(&GatheringNode, &Name, f32)> = None;
    for (node, t, name) in node_q.iter() {
        if node.remaining == 0 { continue; }
        let dist = player_pos.distance(t.translation);
        if dist < 20.0 {
            if nearest.is_none() || dist < nearest.unwrap().2 {
                nearest = Some((node, name, dist));
            }
        }
    }

    if let Some((node, name, _)) = nearest {
        let Ok(ctx) = contexts.ctx_mut() else { return; };
        egui::Area::new("gather_prompt".into())
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 110.0])
            .show(ctx, |ui| {
                ui.label(egui::RichText::new(
                    format!("Press F to gather {} ({}/{})",
                        name, node.remaining, node.max_remaining))
                    .size(16.0).color(egui::Color32::from_rgb(180, 220, 130)).strong());
            });
    }
}

// ─── Graphics Settings Panel ────────────────────────────

fn graphics_settings_panel_system(
    mut contexts: EguiContexts,
    mut gui_state: ResMut<GuiState>,
    keys: Res<ButtonInput<KeyCode>>,
    mut settings: ResMut<GraphicsSettings>,
    diagnostics: Res<bevy::diagnostic::DiagnosticsStore>,
) {
    if keys.just_pressed(KeyCode::F4) {
        gui_state.show_graphics = !gui_state.show_graphics;
    }
    if !gui_state.show_graphics { return; }

    let Ok(ctx) = contexts.ctx_mut() else { return; };

    egui::Window::new("Graphics Settings")
        .anchor(egui::Align2::LEFT_CENTER, [10.0, 0.0])
        .resizable(false)
        .collapsible(false)
        .min_width(280.0)
        .show(ctx, |ui| {
            // FPS display
            if let Some(fps) = diagnostics.get(&bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(value) = fps.smoothed() {
                    let fps_color = if value > 55.0 {
                        egui::Color32::from_rgb(50, 200, 50)
                    } else if value > 30.0 {
                        egui::Color32::from_rgb(230, 180, 30)
                    } else {
                        egui::Color32::from_rgb(220, 40, 40)
                    };
                    ui.label(egui::RichText::new(format!("FPS: {:.0}", value))
                        .size(18.0).color(fps_color).strong());
                }
            }

            ui.separator();
            ui.heading("Quality Preset");

            let current = settings.quality_tier;
            ui.horizontal(|ui| {
                for tier in &[QualityTier::Low, QualityTier::Medium, QualityTier::High, QualityTier::Ultra] {
                    let label = format!("{}", tier);
                    let selected = *tier == current;
                    let btn = if selected {
                        ui.add(egui::Button::new(
                            egui::RichText::new(&label).strong().color(egui::Color32::BLACK))
                            .fill(egui::Color32::GOLD))
                    } else {
                        ui.button(&label)
                    };
                    if btn.clicked() && !selected {
                        settings.set_tier(*tier);
                        println!("Graphics quality changed to: {}", tier);
                    }
                }
            });

            ui.separator();
            ui.heading("Current Settings");

            ui.label(format!("Shadows: {} cascades @ {}px", settings.shadow_cascades, settings.shadow_resolution));
            ui.label(format!("Draw distance: {:.0}m", settings.draw_distance));
            ui.label(format!("Fog: {:.0}m - {:.0}m", settings.fog_start, settings.fog_end));

            let bool_label = |v: bool| if v { "ON" } else { "OFF" };
            let bool_color = |v: bool| if v {
                egui::Color32::from_rgb(80, 200, 80)
            } else {
                egui::Color32::from_rgb(150, 60, 60)
            };

            ui.horizontal(|ui| {
                ui.label("SSAO: ");
                ui.label(egui::RichText::new(bool_label(settings.ssao_enabled)).color(bool_color(settings.ssao_enabled)));
                ui.label(" | Bloom: ");
                ui.label(egui::RichText::new(bool_label(settings.bloom_enabled)).color(bool_color(settings.bloom_enabled)));
            });
            ui.horizontal(|ui| {
                ui.label("Ray Tracing: ");
                ui.label(egui::RichText::new(bool_label(settings.ray_tracing_enabled)).color(bool_color(settings.ray_tracing_enabled)));
                ui.label(" | Upscaling: ");
                ui.label(egui::RichText::new(bool_label(settings.upscaling_enabled)).color(bool_color(settings.upscaling_enabled)));
            });

            ui.label(format!("Vegetation: {:.0}% | Particles: {:.0}%",
                settings.vegetation_density * 100.0, settings.particle_quality * 100.0));
            ui.label(format!("LOD bias: {:.1} | Texture quality: {:.0}%",
                settings.lod_bias, settings.texture_quality * 100.0));

            ui.separator();
            ui.label(egui::RichText::new("Press F4 to close").size(11.0).color(egui::Color32::GRAY));
        });
}
