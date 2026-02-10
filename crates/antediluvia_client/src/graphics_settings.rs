//! Graphics quality tier system with GPU detection and auto-tier selection.
//!
//! Supports four quality tiers:
//! - Low: Forward rendering, ambient light, minimal shadows (integrated GPUs)
//! - Medium: Deferred rendering, light probes, SSAO, bloom (GTX 1060 class)
//! - High: Solari ray tracing, ReSTIR DI/GI, DLSS/FSR upscaling (RTX 2060 class)
//! - Ultra: Solari native resolution, max samples (RTX 4070+ class)

use bevy::prelude::*;

pub struct GraphicsSettingsPlugin;

impl Plugin for GraphicsSettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GraphicsSettings>()
           .add_systems(Startup, detect_and_apply_quality);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QualityTier {
    Low,
    Medium,
    High,
    Ultra,
}

impl Default for QualityTier {
    fn default() -> Self {
        QualityTier::Medium
    }
}

impl std::fmt::Display for QualityTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QualityTier::Low => write!(f, "Low"),
            QualityTier::Medium => write!(f, "Medium"),
            QualityTier::High => write!(f, "High"),
            QualityTier::Ultra => write!(f, "Ultra"),
        }
    }
}

#[derive(Resource)]
pub struct GraphicsSettings {
    pub quality_tier: QualityTier,
    pub shadow_cascades: u32,
    pub shadow_resolution: u32,
    pub draw_distance: f32,
    pub fog_start: f32,
    pub fog_end: f32,
    pub ssao_enabled: bool,
    pub bloom_enabled: bool,
    pub vegetation_density: f32,
    pub particle_quality: f32,
    pub lod_bias: f32,
    pub texture_quality: f32,
    pub ray_tracing_enabled: bool,
    pub upscaling_enabled: bool,
    pub vsync: bool,
    pub user_override: bool,
}

impl Default for GraphicsSettings {
    fn default() -> Self {
        Self::from_tier(QualityTier::Medium)
    }
}

impl GraphicsSettings {
    pub fn from_tier(tier: QualityTier) -> Self {
        match tier {
            QualityTier::Low => Self {
                quality_tier: tier,
                shadow_cascades: 1,
                shadow_resolution: 512,
                draw_distance: 200.0,
                fog_start: 120.0,
                fog_end: 200.0,
                ssao_enabled: false,
                bloom_enabled: false,
                vegetation_density: 0.25,
                particle_quality: 0.25,
                lod_bias: 2.0,
                texture_quality: 0.25,
                ray_tracing_enabled: false,
                upscaling_enabled: false,
                vsync: true,
                user_override: false,
            },
            QualityTier::Medium => Self {
                quality_tier: tier,
                shadow_cascades: 3,
                shadow_resolution: 1024,
                draw_distance: 500.0,
                fog_start: 250.0,
                fog_end: 500.0,
                ssao_enabled: true,
                bloom_enabled: true,
                vegetation_density: 0.5,
                particle_quality: 0.5,
                lod_bias: 1.0,
                texture_quality: 0.5,
                ray_tracing_enabled: false,
                upscaling_enabled: false,
                vsync: true,
                user_override: false,
            },
            QualityTier::High => Self {
                quality_tier: tier,
                shadow_cascades: 4,
                shadow_resolution: 2048,
                draw_distance: 1000.0,
                fog_start: 600.0,
                fog_end: 1000.0,
                ssao_enabled: true,
                bloom_enabled: true,
                vegetation_density: 1.0,
                particle_quality: 1.0,
                lod_bias: 0.5,
                texture_quality: 1.0,
                ray_tracing_enabled: true,
                upscaling_enabled: true,
                vsync: true,
                user_override: false,
            },
            QualityTier::Ultra => Self {
                quality_tier: tier,
                shadow_cascades: 4,
                shadow_resolution: 4096,
                draw_distance: 2000.0,
                fog_start: 1200.0,
                fog_end: 2000.0,
                ssao_enabled: true,
                bloom_enabled: true,
                vegetation_density: 1.0,
                particle_quality: 1.0,
                lod_bias: 0.0,
                texture_quality: 1.0,
                ray_tracing_enabled: true,
                upscaling_enabled: false,
                vsync: false,
                user_override: false,
            },
        }
    }

    pub fn set_tier(&mut self, tier: QualityTier) {
        let new = Self::from_tier(tier);
        self.quality_tier = new.quality_tier;
        self.shadow_cascades = new.shadow_cascades;
        self.shadow_resolution = new.shadow_resolution;
        self.draw_distance = new.draw_distance;
        self.fog_start = new.fog_start;
        self.fog_end = new.fog_end;
        self.ssao_enabled = new.ssao_enabled;
        self.bloom_enabled = new.bloom_enabled;
        self.vegetation_density = new.vegetation_density;
        self.particle_quality = new.particle_quality;
        self.lod_bias = new.lod_bias;
        self.texture_quality = new.texture_quality;
        self.ray_tracing_enabled = new.ray_tracing_enabled;
        self.upscaling_enabled = new.upscaling_enabled;
        self.user_override = true;
    }
}

/// Auto-detect GPU capabilities and select appropriate quality tier.
fn detect_gpu_tier() -> QualityTier {
    // Bevy 0.18 doesn't expose GPU info at startup easily.
    // For now, default to Medium which works on most hardware.
    // In the future, we can query wgpu adapter info after renderer init.
    //
    // Detection strategy (future implementation):
    // 1. Query wgpu::AdapterInfo for device name and backend
    // 2. Check for ray tracing extension support
    // 3. Estimate VRAM from adapter limits
    // 4. Map known GPU families to tiers:
    //    - Intel HD/UHD/Iris → Low
    //    - GTX 1050-1070, RX 570-590 → Medium
    //    - RTX 2060-3060, RX 6600-6700 → High
    //    - RTX 3070+, RTX 4060+, RX 7800+ → Ultra
    //    - Apple M1/M2 → Medium (no ray tracing)
    //    - Apple M3 Pro/Max → High (hardware ray tracing)

    #[cfg(target_os = "macos")]
    {
        // Apple Silicon: default to Medium (M1/M2) or High (M3+)
        // Can't easily distinguish at compile time, so default Medium
        return QualityTier::Medium;
    }

    #[cfg(not(target_os = "macos"))]
    {
        QualityTier::Medium
    }
}

fn detect_and_apply_quality(
    mut settings: ResMut<GraphicsSettings>,
) {
    if settings.user_override {
        println!("Graphics: Using user-selected {} quality", settings.quality_tier);
        return;
    }

    let tier = detect_gpu_tier();
    settings.set_tier(tier);
    settings.user_override = false; // Reset since this was auto-detected

    println!("Graphics: Auto-detected {} quality tier", tier);
    println!("  Shadows: {} cascades @ {}px", settings.shadow_cascades, settings.shadow_resolution);
    println!("  Draw distance: {:.0}m", settings.draw_distance);
    println!("  SSAO: {} | Bloom: {}", settings.ssao_enabled, settings.bloom_enabled);
    println!("  Ray tracing: {} | Upscaling: {}", settings.ray_tracing_enabled, settings.upscaling_enabled);
    println!("  Vegetation: {:.0}% | Particles: {:.0}%", settings.vegetation_density * 100.0, settings.particle_quality * 100.0);
}
