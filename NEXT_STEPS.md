# Antediluvia: Next Steps for Phase 3

**Current Status:** Phase 2 Complete (Combat, Inventory, Physics, Graphics)
**Ready for:** Playtesting, Bug Fixing, Content Expansion

---

## Immediate Actions (This Week)

### 1. Verify Build & Test
```bash
cd Antediluvia_Dev
cargo test --all 2>&1 | grep -E "test result|error"
cargo run -p antediluvia_client --release
```

**What to test:**
- [ ] Game launches without crashes
- [ ] Login works or offline mode activates
- [ ] Terrain renders correctly
- [ ] Can move with WASD, look with mouse
- [ ] Mobs visible and attackable
- [ ] Inventory UI opens with I key
- [ ] Combat abilities work (1-4 keys)
- [ ] Ability cooldowns prevent spam

### 2. Fix Any Compilation Errors
If `cargo check` shows errors:
```bash
cargo check -p antediluvia_client 2>&1 | grep "^error"
```

**Common issues & fixes:**
- Missing imports → Add to module declarations
- Type mismatches → Check query parameters
- Unused variables → Use `#[allow(dead_code)]` or remove

### 3. Performance Baseline
Run game and measure:
- FPS (target: 60)
- Startup time (target: <5 seconds)
- Memory usage (Activity Monitor)
- Frame time consistency

---

## Week 2: Content & Features

### 1. Crafting System UI (800 LOC estimate)

**Current State:** Core crafting system exists, not exposed to client

**To Implement:**
- [ ] Create `client/crafting.rs`
- [ ] Add CraftingUI panel (similar to inventory)
- [ ] Display available recipes based on inventory
- [ ] Add craft button with cost validation
- [ ] Show crafted items in real-time
- [ ] Track crafting progress/time

**Example UI:**
```
CRAFTING
━━━━━━━━━━━━━━
Available Recipes:
  [Bronze Sword] - Requires: 2x Bronze
  [Shield] - Requires: 3x Wood
  [Rope] - Requires: 1x Plant Fiber

Select with mouse, click [Craft]
Progress: ████░░ (50%)
```

### 2. Mob AI & Variety (600 LOC estimate)

**Current:** Static test wolves
**Add:**
- [ ] Mob types: Wolf, Spider, Skeleton
- [ ] Mob behaviors: Idle, Patrol, Aggro, Flee
- [ ] Aggro ranges (50-200 units depending on mob)
- [ ] Pack tactics (2+ mobs attack together)
- [ ] Different damage, health, speeds
- [ ] Loot drops (items, gold, experience)

**Code structure:**
```rust
#[derive(Component)]
pub struct MobBrain {
    pub state: MobState,
    pub aggro_range: f32,
    pub pack_members: Vec<Entity>,
    pub behavior_timer: f32,
}

enum MobState {
    Idle,
    Patrol { target: Vec3 },
    Aggro { target: Entity },
    Fleeing { from: Vec3 },
}
```

### 3. Visual Effects (400 LOC estimate)

**Current:** Basic health bars in console
**Add:**
- [ ] Particle effects for damage
- [ ] Ability impact visuals
- [ ] Mob death effects
- [ ] Healing glow
- [ ] Skill chain flash effects
- [ ] Blood splat on hit

**Particle types:**
- Impact particles (on hit)
- Healing aura (green)
- Critical hit (red burst)
- Skill chain (lightning)

---

## Week 3-4: Advanced Systems

### 1. Leveling & Progression (500 LOC)
- [ ] Experience system
- [ ] Level up mechanics
- [ ] Stat increases (HP, damage, speed)
- [ ] Skill points & ability unlocking
- [ ] Character sheet UI

### 2. Loot & Equipment (400 LOC)
- [ ] Equip slots (head, chest, legs, feet, weapon)
- [ ] Item quality tiers (Common → Legendary)
- [ ] Equipment stats (damage, defense, speed)
- [ ] Item durability & repair

### 3. Flood Event Progression (300 LOC)
- [ ] Visual water level rising
- [ ] Darkness increasing
- [ ] Time running out timer
- [ ] Ark readiness tracker
- [ ] End game triggers

### 4. NPC Dialogue (600 LOC)
- [ ] Interact button (press E near NPC)
- [ ] Dialogue trees
- [ ] Quest system foundation
- [ ] Relationship tracking
- [ ] Multiple dialogue options

---

## High-Value Quick Wins

These are easy but impactful:

### 1. More Mobs at Startup (30 min)
```rust
// In spawn_test_mob, add more variety
spawn_mob(commands, Vec3::new(100, 5, 100), "Wolf", 1);
spawn_mob(commands, Vec3::new(-100, 5, 100), "Spider", 2);
spawn_mob(commands, Vec3::new(50, 5, -100), "Wolf", 1);
```

### 2. Color-Coded Damage Numbers (1 hour)
- Green for healing
- Red for damage
- Yellow for critical hits
- Floating text above target

### 3. Sound Effects (2 hours)
```bash
# Add bevy_kira_audio plugin
cargo add bevy_kira_audio

# Add assets/sounds/
# - hit.wav
# - heal.wav
# - level_up.wav
```

### 4. Ability Tooltips (30 min)
Hover over ability numbers to see:
- Damage/healing amount
- Cooldown time
- Range/radius
- Mana cost (if adding mana system)

### 5. Simple Minimap (1 hour)
Overlay showing:
- Player position (center)
- Mob positions (red dots)
- NPC positions (blue dots)
- World bounds (gray)

---

## Testing Checklist for Phase 2

### Stability
- [ ] No crashes on startup
- [ ] No crashes on combat
- [ ] No crashes on inventory open
- [ ] Offline mode works if server down
- [ ] Graceful error messages

### Gameplay
- [ ] Can attack mobs (press 1-4)
- [ ] Cooldowns prevent spam
- [ ] Mobs take damage and die
- [ ] Health bars work correctly
- [ ] Inventory tracks items accurately

### Graphics
- [ ] Terrain visible and colored
- [ ] Lighting looks good
- [ ] Eden Pillar visible
- [ ] Mobs render correctly
- [ ] UI panels display properly

### Performance
- [ ] 60 FPS target achieved
- [ ] No lag spikes
- [ ] Smooth camera movement
- [ ] Inventory toggle instant

---

## Documentation TODOs

- [ ] Update README.md with new features
- [ ] Add combat system docs to Design_Bible
- [ ] Create physics system explanation
- [ ] Document new hotkeys (1-4 for abilities, I for inventory)
- [ ] Update BUILD_STATUS.md

---

## Deployment Considerations

### Before Public Alpha:
- [ ] Complete Phase 3 features
- [ ] Full playtesting (4+ hours)
- [ ] Crash bug hunt
- [ ] Performance profiling
- [ ] Security audit (database, auth)

### Before Beta:
- [ ] Server implementation
- [ ] Multiplayer testing
- [ ] Balance tuning
- [ ] Content lock (finalize abilities, mobs, items)

---

## Estimated Timelines

| Task | Estimate | Priority |
|------|----------|----------|
| Bug fixes & build verification | 2 hours | CRITICAL |
| Crafting UI | 3 hours | HIGH |
| Mob AI | 4 hours | HIGH |
| Visual effects | 3 hours | MEDIUM |
| Leveling system | 4 hours | HIGH |
| Loot & equipment | 3 hours | MEDIUM |
| Flood visuals | 2 hours | MEDIUM |
| NPC dialogue | 4 hours | MEDIUM |

**Total Phase 3:** ~25 hours (1 week intensive, or 2-3 weeks part-time)

---

## Quick Reference: Key Bindings

```
MOVEMENT
  W/A/S/D      - Move forward/left/backward/right
  Shift        - Sprint
  Mouse        - Look around

COMBAT
  1            - Hunter Thrust (50 dmg, 2.5s cooldown)
  2            - Hunter Slash (40 dmg, 2.0s cooldown)
  3            - Levite Heal (50 HP, 4.0s cooldown)
  4            - Forge Smash (60 dmg, 3.5s cooldown)

UI
  I            - Toggle Inventory
  M            - Toggle Map
  F3           - Toggle Debug HUD
  ESC          - Menu (future)

DEBUG
  CTRL+D       - Toggle debug info (future)
  CTRL+S       - Force save to database (future)
```

---

## Recommendation

**Start with:**
1. Run build verification (30 min)
2. Playtest for 30 minutes
3. Fix any crashes (1-2 hours)
4. Add 2-3 more mob types (1 hour)
5. Add simple particle effects (1 hour)

This gets you to a **stable, playable, visually interesting vertical slice** that's ready for feedback.

**Then:**
1. Crafting UI
2. Mob AI
3. Effects
4. Progression

---

**Next Session:** Verify build works, then decide: Bug fixes first, or new features first?
