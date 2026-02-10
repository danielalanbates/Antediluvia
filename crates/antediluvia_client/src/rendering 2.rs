//! Rendering pipeline configuration based on quality tier.
//!
//! Configures fog, shadows, post-processing (SSAO, Bloom, TAA, volumetric fog,
//! god rays), color grading, tonemapping, and atmospheric fog coloring
//! according to the current GraphicsSettings quality tier, day/night cycle,
//! and world corruption level.

use bevy::prelude::*;
use bevy::pbr::ScreenSpaceAmbientOcclusion;
use bevy::post_process::bloom::Bloom;
use bevy::anti_alias::taa::TemporalAntiAliasing;
use bevy::light::{VolumetricFog, VolumetricLight, DirectionalLightShadowMap, GlobalAmbientLight};
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::render::view::{ColorGrading, ColorGradingGlobal, ColorGradingSection};
use crate::graphics_settings::{GraphicsSettings, QualityTier};
use crate::player::FollowCamera;
use crate::{DayNightCycle, WorldState};

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DirectionalLightShadowMap { size: 1024 })
           .add_systems(PostStartup, apply_rendering_settings)
           .add_systems(Update, (
               update_fog_from_settings,
               update_post_processing_from_settings,
               dynamic_color_grading_system,
               atmospheric_fog_system,
           ));
    }
}

/// Apply rendering settings after startup (once cameras and lights exist).
fn apply_rendering_settings(
    mut commands: Commands,
    settings: Res<GraphicsSettings>,
    mut fog_q: Query<&mut DistanceFog>,
    mut dir_light_q: Query<(Entity, &mut DirectionalLight)>,
    mut ambient: ResMut<GlobalAmbientLight>,
    mut shadow_map: ResMut<DirectionalLightShadowMap>,
    camera_q: Query<Entity, With<FollowCamera>>,
) {
    // Update fog distances based on quality tier
    for mut fog in fog_q.iter_mut() {
        fog.falloff = FogFalloff::Linear {
            start: settings.fog_start,
            end: settings.fog_end,
        };
    }

    // Update shadow map resolution
    shadow_map.size = settings.shadow_resolution as usize;

    // Update shadow settings on directional lights
    for (_entity, mut light) in dir_light_q.iter_mut() {
        light.shadows_enabled = settings.shadow_cascades > 0;
    }

    // Add VolumetricLight to directional lights for god rays (High+)
    if settings.quality_tier == QualityTier::High || settings.quality_tier == QualityTier::Ultra {
        for (entity, _light) in dir_light_q.iter() {
            commands.entity(entity).insert(VolumetricLight);
        }
    }

    // Ambient light adjustment per tier
    ambient.brightness = match settings.quality_tier {
        QualityTier::Low => 0.6,
        QualityTier::Medium => 0.5,
        QualityTier::High => 0.4,
        QualityTier::Ultra => 0.35,
    };

    // Add post-processing components to the 3D camera
    if let Ok(camera_entity) = camera_q.single() {
        let mut entity_commands = commands.entity(camera_entity);

        // Tonemapping (quality-dependent algorithm)
        entity_commands.insert(match settings.quality_tier {
            QualityTier::Low => Tonemapping::Reinhard,
            QualityTier::Medium => Tonemapping::AcesFitted,
            _ => Tonemapping::TonyMcMapface,
        });

        // Color grading (initial values, updated dynamically)
        entity_commands.insert(ColorGrading {
            global: ColorGradingGlobal {
                exposure: 0.0,
                temperature: 0.0,
                tint: 0.0,
                hue: 0.0,
                post_saturation: 1.0,
                ..default()
            },
            shadows: ColorGradingSection::default(),
            midtones: ColorGradingSection::default(),
            highlights: ColorGradingSection::default(),
        });

        // Bloom (Medium+)
        if settings.bloom_enabled {
            entity_commands.insert(Bloom {
                intensity: match settings.quality_tier {
                    QualityTier::Low => 0.0,
                    QualityTier::Medium => 0.15,
                    QualityTier::High => 0.2,
                    QualityTier::Ultra => 0.25,
                },
                ..default()
            });
        }

        // SSAO (Medium+)
        if settings.ssao_enabled {
            entity_commands.insert(ScreenSpaceAmbientOcclusion::default());
        }

        // TAA (Medium+) - recommended with SSAO
        if settings.ssao_enabled || settings.bloom_enabled {
            entity_commands.insert(TemporalAntiAliasing::default());
        }

        // Volumetric fog (High+)
        if settings.quality_tier == QualityTier::High || settings.quality_tier == QualityTier::Ultra {
            entity_commands.insert(VolumetricFog {
                ambient_intensity: 0.05,
                step_count: match settings.quality_tier {
                    QualityTier::High => 32,
                    QualityTier::Ultra => 64,
                    _ => 16,
                },
                ..default()
            });
        }
    }

    println!("Rendering pipeline configured for {} quality", settings.quality_tier);
    println!("  Shadow map: {}px | Bloom: {} | SSAO: {} | TAA: {}",
        settings.shadow_resolution,
        settings.bloom_enabled,
        settings.ssao_enabled,
        settings.ssao_enabled || settings.bloom_enabled);
    println!("  Color grading: enabled | Tonemapping: {}",
        match settings.quality_tier {
            QualityTier::Low => "Reinhard",
            QualityTier::Medium => "ACES Fitted",
            _ => "TonyMcMapface",
        });
    if settings.quality_tier == QualityTier::High || settings.quality_tier == QualityTier::Ultra {
        println!("  Volumetric fog: enabled | God rays: enabled");
    }
}

/// Dynamically update fog if settings change at runtime.
fn update_fog_from_settings(
    settings: Res<GraphicsSettings>,
    mut fog_q: Query<&mut DistanceFog>,
    mut shadow_map: ResMut<DirectionalLightShadowMap>,
    mut ambient: ResMut<GlobalAmbientLight>,
    mut dir_light_q: Query<&mut DirectionalLight>,
) {
    if !settings.is_changed() {
        return;
    }

    for mut fog in fog_q.iter_mut() {
        fog.falloff = FogFalloff::Linear {
            start: settings.fog_start,
            end: settings.fog_end,
        };
    }

    shadow_map.size = settings.shadow_resolution as usize;

    for mut light in dir_light_q.iter_mut() {
        light.shadows_enabled = settings.shadow_cascades > 0;
    }

    ambient.brightness = match settings.quality_tier {
        QualityTier::Low => 0.6,
        QualityTier::Medium => 0.5,
        QualityTier::High => 0.4,
        QualityTier::Ultra => 0.35,
    };
}

/// Update post-processing components when settings change.
fn update_post_processing_from_settings(
    mut commands: Commands,
    settings: Res<GraphicsSettings>,
    camera_q: Query<Entity, With<FollowCamera>>,
    bloom_q: Query<Entity, (With<FollowCamera>, With<Bloom>)>,
    ssao_q: Query<Entity, (With<FollowCamera>, With<ScreenSpaceAmbientOcclusion>)>,
) {
    if !settings.is_changed() {
        return;
    }

    let Ok(camera_entity) = camera_q.single() else {
        return;
    };

    // Bloom
    if settings.bloom_enabled {
        if bloom_q.is_empty() {
            commands.entity(camera_entity).insert(Bloom {
                intensity: match settings.quality_tier {
                    QualityTier::Low => 0.0,
                    QualityTier::Medium => 0.15,
                    QualityTier::High => 0.2,
                    QualityTier::Ultra => 0.25,
                },
                ..default()
            });
        }
    } else if !bloom_q.is_empty() {
        commands.entity(camera_entity).remove::<Bloom>();
    }

    // SSAO
    if settings.ssao_enabled {
        if ssao_q.is_empty() {
            commands.entity(camera_entity).insert(ScreenSpaceAmbientOcclusion::default());
        }
    } else if !ssao_q.is_empty() {
        commands.entity(camera_entity).remove::<ScreenSpaceAmbientOcclusion>();
    }
}

/// Dynamically adjust color grading based on time of day and corruption.
///
/// - Exposure: higher at noon, lower at night
/// - Temperature: warm at sunrise/sunset, cool at night
/// - Saturation: drops with corruption (world becomes desaturated)
/// - Shadows lift with corruption (murkier darks)
/// - Highlights gain increases with corruption (harsher contrast)
fn dynamic_color_grading_system(
    cycle: Res<DayNightCycle>,
    world_state: Res<WorldState>,
    settings: Res<GraphicsSettings>,
    mut grading_q: Query<&mut ColorGrading, With<FollowCamera>>,
) {
    let Ok(mut grading) = grading_q.single_mut() else { return };

    let hour = cycle.time_of_day;
    let corruption = world_state.corruption / 100.0; // 0..1

    // --- Exposure ---
    // Bright at noon (~0.3 EV boost), dark at night (~-0.4 EV)
    let exposure = if hour >= 6.0 && hour < 18.0 {
        let sun_progress = (hour - 6.0) / 12.0;
        let sun_height = (sun_progress * std::f32::consts::PI).sin();
        -0.1 + sun_height * 0.4
    } else {
        -0.4
    };
    grading.global.exposure = exposure;

    // --- Temperature (warm/cool shift) ---
    // Warm at sunrise (6-8) and sunset (17-19), neutral midday, cool at night
    let temperature = if hour >= 6.0 && hour < 8.0 {
        let t = (hour - 6.0) / 2.0; // 0..1 during sunrise
        0.06 * (1.0 - t) // Warm, fading
    } else if hour >= 17.0 && hour < 19.0 {
        let t = (hour - 17.0) / 2.0; // 0..1 during sunset
        0.08 * (1.0 - t) // Warm golden hour
    } else if hour >= 19.0 || hour < 6.0 {
        -0.03 // Slightly cool at night
    } else {
        0.0 // Neutral midday
    };
    grading.global.temperature = temperature;

    // --- Saturation ---
    // Corruption desaturates the world (apocalyptic feel)
    let saturation = 1.0 - corruption * 0.4;
    grading.global.post_saturation = saturation;

    // --- Quality-tier dependent section grading (Medium+) ---
    if settings.quality_tier == QualityTier::Low {
        return;
    }

    // Shadows: corruption lifts shadows (murky, hazy darks)
    grading.shadows.lift = corruption * 0.04;
    grading.shadows.saturation = 1.0 - corruption * 0.3;

    // Midtones: slight contrast boost at golden hour
    let golden_boost = if (hour >= 6.5 && hour < 8.0) || (hour >= 17.0 && hour < 18.5) {
        0.05
    } else {
        0.0
    };
    grading.midtones.contrast = 1.0 + golden_boost;

    // Highlights: corruption makes bright areas harsher
    grading.highlights.gain = 1.0 + corruption * 0.1;
    grading.highlights.saturation = 1.0 - corruption * 0.2;
}

/// Dynamically color the distance fog based on time of day and corruption.
///
/// Creates an atmospheric effect where fog color matches the sky:
/// - Dawn/dusk: warm orange-pink haze
/// - Day: light blue-white atmospheric haze
/// - Night: dark blue-grey mist
/// - Corruption: reddish-brown tint overlay
fn atmospheric_fog_system(
    cycle: Res<DayNightCycle>,
    world_state: Res<WorldState>,
    mut fog_q: Query<&mut DistanceFog>,
) {
    let hour = cycle.time_of_day;
    let corruption = world_state.corruption / 100.0;

    // Base fog color from time of day
    let (r, g, b) = if hour >= 6.0 && hour < 7.5 {
        // Sunrise: warm peach/gold
        let t = (hour - 6.0) / 1.5;
        (
            0.75 - t * 0.2,  // starts warm, fades
            0.50 + t * 0.15,
            0.40 + t * 0.35,
        )
    } else if hour >= 7.5 && hour < 16.5 {
        // Day: light atmospheric blue-white
        (0.55, 0.65, 0.75)
    } else if hour >= 16.5 && hour < 18.5 {
        // Sunset: warm orange to dusky purple
        let t = (hour - 16.5) / 2.0;
        (
            0.55 + t * 0.25,  // gets warmer
            0.65 - t * 0.30,  // greens fade
            0.75 - t * 0.35,  // blues fade
        )
    } else {
        // Night: dark blue-grey
        (0.12, 0.14, 0.22)
    };

    // Corruption tints fog toward reddish-brown
    let r = (r + corruption * 0.2).min(0.9);
    let g = (g - corruption * 0.15).max(0.05);
    let b = (b - corruption * 0.2).max(0.05);

    for mut fog in fog_q.iter_mut() {
        fog.color = Color::srgba(r, g, b, 1.0);
    }
}
