# Antediluvia: The Tenth Generation

A high-efficiency, biblically-grounded survival MMORPG set in the antediluvian age (Genesis 4-6).

## Project Status

**Phase 2: Gameplay Systems & Stability** ✅ COMPLETE
- ✅ Fixed all crash-causing `.unwrap()` calls
- ✅ Removed blocking HTTP calls (non-blocking fallback)
- ✅ Integrated combat system (playable battles)
- ✅ Built inventory UI (weight tracking, item display)
- ✅ Added physics system (collision detection)
- ✅ Enhanced graphics (multi-layer terrain, better lighting)
- ✅ Secured database credentials (.env config)
- ✅ Deuplicated networking code

**Currently Playable:**
- Move with WASD, look with mouse
- Press 1-4 to use combat abilities (50-60 damage each)
- Press I to view inventory
- Press M for map
- Mobs spawn and take damage
- Health tracking and cooldown system

**Phase 1: Foundation** ✅ COMPLETE (Prior)
- Rust/Bevy workspace initialized
- Procedural terrain generation (Pangea Ultima)
- Core entity system (Players, NPCs, Jobs)
- Diegetic UI systems (Map, Inventory, Camera)
- NPC spawning (Noah, Methuselah)

## Building & Running

### Prerequisites
- Rust 1.92.0+ (installed via rustup)
- macOS or Windows

### Build
```bash
source $HOME/.cargo/env
cd /path/to/Antediluvia
cargo build -p antediluvia_client --release
```

### Run
```bash
cargo run -p antediluvia_client --release
```

## Controls

### Movement
- **W/A/S/D**: Move forward/left/backward/right
- **Mouse**: Look around
- **Shift**: Sprint

### Combat
- **1**: Hunter Thrust (50 damage, 2.5s cooldown)
- **2**: Hunter Slash (40 damage, 2.0s cooldown)
- **3**: Levite Heal (50 HP, 4.0s cooldown)
- **4**: Forge Smash (60 damage, 3.5s cooldown)

### UI
- **I**: Toggle Inventory
- **M**: Toggle Map
- **F3**: Debug HUD (Toggle)

## Architecture

### Workspace Structure
```
crates/
├── antediluvia_core/       # Shared logic, world gen, entities
├── antediluvia_client/     # Graphics, input, UI (Bevy)
├── antediluvia_server/     # Authoritative server (future)
└── antediluvia_ai/         # LLM integration (future)
```

### Core Systems
- **World Generation**: Procedural Pangea using Simplex Noise
- **Entity System**: ECS-based (Bevy)
- **Diegetic Interface**: No HUD elements
- **Job System**: Classless progression with hidden unlocks

## Next Steps (Phase 3)

1. **Crafting System UI** - Hook up core crafting logic to visual interface
2. **Mob AI & Variety** - Pack tactics, different mob types, loot drops
3. **Visual Effects** - Particle systems for abilities, damage numbers
4. **Leveling System** - Experience, levels, stat progression
5. **Loot & Equipment** - Equipment slots, item quality tiers, stats
6. **Flood Event Progression** - Water rising, darkness, end-game trigger
7. **NPC Dialogue** - Interact system, conversation trees, quests

See `NEXT_STEPS.md` for detailed roadmap and time estimates.

## Documentation

- **`REBUILD_SUMMARY.md`** - Complete Phase 2 improvements overview
- **`PHASE_2_IMPROVEMENTS.md`** - Detailed technical changes and fixes
- **`NEXT_STEPS.md`** - Phase 3 roadmap with time estimates
- **`Design_Bible/`** - Complete game design (10 volumes, 20,000+ words)
- **`BUILD_STATUS.md`** - Compilation metrics and technical details

## License

Proprietary. All rights reserved.
