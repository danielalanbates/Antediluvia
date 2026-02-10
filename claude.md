# Project State
- [x] Phase 3A: Make Combat Feel Real - COMPLETE
- [x] Phase 3B: World Comes Alive - COMPLETE
- [x] Phase 4A: Bevy 0.13 → 0.18 Upgrade - COMPLETE
- [x] Phase 4B: Graphics Quality Tier System - COMPLETE
- [x] Phase 4C: Post-Processing Pipeline & Graphics UI - COMPLETE
- [ ] Active Task: Phase 5 - Asset Pipeline, Terrain, Water, Advanced Rendering

## Completed Work

### Phase 4C - Post-Processing Pipeline & Graphics UI
1. **Rendering pipeline** (rendering.rs) now wires up Bevy 0.18 post-processing:
   - **SSAO** (ScreenSpaceAmbientOcclusion) - Medium+ tiers
   - **Bloom** with tier-scaled intensity (0.15→0.25) - Medium+ tiers
   - **TAA** (TemporalAntiAliasing) - Medium+ tiers (recommended with SSAO)
   - **VolumetricFog** with god rays (VolumetricLight on DirectionalLight) - High/Ultra tiers
   - **DirectionalLightShadowMap** resolution per tier (512→4096px)
   - **CascadeShadowConfig** cascades per tier (1→4)
   - Dynamic runtime switching: add/remove Bloom, SSAO when tier changes
2. **Graphics settings UI** (gui.rs, F4 key):
   - FPS counter with color-coded thresholds (green >55, yellow >30, red <30)
   - Quality preset buttons (Low/Medium/High/Ultra) with gold highlight on active
   - Live display of all settings: shadows, draw distance, fog, SSAO, bloom, RT, upscaling, vegetation, particles, LOD
3. **Solari ray tracing** support:
   - Optional `solari` feature flag in client Cargo.toml (`bevy/bevy_solari`)
   - Build with: `cargo run --features solari` for RT-capable GPUs
   - `bevy_solari` v0.18.0 auto-resolved in Cargo.lock

### Phase 4A - Bevy 0.18 Upgrade
Full migration from Bevy 0.13.2 to Bevy 0.18.0 across all crates:
1. **Workspace dependencies**: bevy 0.18, bevy_egui 0.39, bevy_renet 4.0, renet 2.0, rand 0.9, thiserror 2.0, reqwest 0.12, glam 0.29
2. **Bundle removal**: All `PbrBundle` → `(Mesh3d, MeshMaterial3d, Transform)` tuples, `Camera3dBundle` → `Camera3d`, `Camera2dBundle` → `Camera2d`, `DirectionalLightBundle` → `DirectionalLight`, `PointLightBundle` → `PointLight`
3. **Text system rewrite**: `TextBundle` → `(Text, TextFont, TextColor, Node)`, `Text2dBundle` → `(Text2d, TextFont, TextColor, Transform)`
4. **UI rewrite**: `NodeBundle` → `Node` component, `Style` fields merged into `Node`
5. **Color API**: `Color::rgb()` → `Color::srgb()`, `Color::rgba()` → `Color::srgba()`, emissive now uses `LinearRgba`
6. **Query API**: `get_single()` → `single()` returning `Result`, `get_single_mut()` → `single_mut()`
7. **Time API**: `delta_seconds()` → `delta_secs()`, `elapsed_seconds()` → `elapsed_secs()`
8. **Fog**: `FogSettings` → `DistanceFog`
9. **Window**: `window.cursor` → `window.cursor_options`
10. **Server networking**: `bevy_renet::renet::transport::*` → `bevy_renet::netcode::*`, added `send_packets()` call
11. **Auth**: `rand::thread_rng().fill_bytes()` → `rand::rng().fill()` (rand 0.9)
12. **Login text**: `txt.sections[0].value` → `**txt = value` (Text now derefs to String)

### Phase 4B - Graphics Quality Tier System
1. **graphics_settings.rs**: `GraphicsSettings` resource with `QualityTier` enum (Low/Medium/High/Ultra)
   - Per-tier configuration: shadows, draw distance, fog, SSAO, bloom, vegetation density, particles, LOD, ray tracing, upscaling
   - GPU auto-detection stub (defaults to Medium, macOS-aware)
   - Runtime tier switching via `set_tier()`

### Phase 3A (Combat)
1. Mob AI system - aggro, patrol, attack, death/despawn with dissolve effect
2. Live HUD - HP bar, ability cooldowns, XP bar, target info, death screen
3. Mob variety - wolves, lions, chimera, corrupted, nephilim boss
4. XP/Leveling with combat rewards and stat scaling
5. Loot drops per mob type into satchel
6. Sprint fix (3x speed), cursor grab, damage numbers, player death/respawn

### Phase 3B (World)
1. NPC Interaction (E key) - Noah, Methuselah, Jubal the Merchant with multi-line dialogue
2. Crafting UI (C key) - "Forge of Tubal-Cain" panel, 3 recipes, ingredient checking, crafting execution
3. World Corruption meter - HUD display, sky color changes with corruption, flood stage tracking
4. Map overlay (M key) - Top-down view with player/NPC/mob markers, color-coded by tier
5. Terrain variety - hills (5), river, bushes (10), more trees (15), more rocks (8)
6. NPCs moved close to player start, merchant added, starting crafting materials

## Current Architecture
- **Bevy 0.18.0** ECS with bevy_egui 0.39, bevy_renet 4.0
- **Client files**: main.rs, player.rs, combat.rs, mob_ai.rs, gui.rs, inventory.rs, login.rs, npc.rs, gathering.rs, physics.rs, map.rs, graphics_settings.rs, rendering.rs
- **Server files**: main.rs, net.rs, game.rs, auth.rs, db.rs
- **Resources**: GraphicsSettings, QualityTier, WorldState, CraftingRes, Equipment, DayNightCycle
- **Plugins**: GraphicsSettingsPlugin, RenderingPlugin, GuiPlugin
- **Post-processing**: SSAO, Bloom, TAA, VolumetricFog, VolumetricLight (tier-dependent)
- **Build**: Zero errors, zero warnings. Compiles from /tmp/antediluvia_build

## Key Bindings
- WASD: Move | Shift: Sprint | Mouse: Look
- 1-4: Abilities | 3: Heal self
- E: Talk to NPC | I: Inventory | C: Crafting | M: Map
- Tab: Equipment | F: Gather | F3: Debug panel | F4: Graphics settings
- Click: Capture mouse | ESC: Release

## Next Steps (Phase 5+)
1. Asset pipeline setup (Blender → glTF 2.0 → Bevy)
2. PBR material templates (albedo, normal, roughness, metallic, AO, emissive)
3. LOD system per model (VisibilityRange component)
4. Terrain heightmap system (replace sphere hills with real mesh)
5. Water shader (planar reflections Low/Med, SSR High, RT reflections Ultra)
6. GPU particles (compute shader fire, smoke, dust, magic effects)
7. Foliage system (instanced grass, tree billboards)
8. Character rendering (subsurface scattering, hair, cloth)
9. Atmosphere (procedural ScatteringMedium)

## Architecture Decisions
- Bevy 0.18.0 ECS - components + systems (no more bundles)
- Tiered rendering: Low (forward/ambient) → Medium (deferred/SSAO/Bloom/TAA) → High (VolumetricFog/GodRays/Solari) → Ultra (native RT, max quality)
- Client: offline-first, graceful degradation
- Core: pure logic, no graphics deps (except bevy::prelude for Vec3)
- Diegetic UI philosophy (but egui dev panel is OK)
- Build from /tmp for speed (Google Drive FS is extremely slow for cargo)
- Solari optional: `cargo run --features solari` for RT-capable GPUs
