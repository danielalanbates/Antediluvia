# Antediluvia Deployment Guide

**Version:** 1.0
**Status:** Ready for Alpha Launch
**Date:** January 9, 2026

---

## Quick Start

### Prerequisites
- Rust 1.92.0+ (installed via rustup)
- macOS or Windows
- 2GB free disk space

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

### Controls
- **WASD:** Move camera
- **Mouse:** Look around
- **Shift:** Sprint
- **M:** Toggle map view
- **I:** Toggle inventory

---

## Architecture Overview

### Crates

**antediluvia_core** (Shared Logic)
- World generation (Pangea Ultima)
- Entity system (Players, NPCs, Mobs)
- Combat system (Skill chains)
- Crafting system
- End game (The Flood)
- Network protocol

**antediluvia_client** (Graphics & Input)
- Bevy rendering
- Camera system
- Diegetic UI (Map, Inventory)
- NPC spawning
- Input handling

**antediluvia_ai** (NPC Intelligence)
- Dialogue generation
- Knowledge base
- NPC brain (state machine)
- Lineage-based responses

**antediluvia_server** (Authoritative Logic - Phase 7)
- Game state management
- Player persistence
- Anti-cheat
- Database integration

---

## Phase Breakdown

### Phase 1: Foundation ✅
- Workspace setup
- Terrain generation
- Entity system
- Diegetic UI

### Phase 2: LLM Integration ✅
- NPC dialogue
- Knowledge base
- AI brain

### Phase 3: Mob AI ✅
- Pack Tactics
- Mob types
- Coordination

### Phase 4: Crafting ✅
- Recipe system
- Item quality
- Durability

### Phase 5: End Game ✅
- Flood event
- Ark system
- Survival mechanics

### Phase 6: Networking ✅
- Network protocol
- Rollback netcode
- Player state sync

### Phase 7: Server (Pending)
- Authoritative logic
- Database persistence
- Anti-cheat system

### Phase 8: Advanced Features (Pending)
- Prophetic visions
- World events
- Skill chain effects

---

## Deployment Checklist

### Pre-Launch (Alpha)
- [ ] Run all unit tests
- [ ] Verify terrain generation
- [ ] Test NPC dialogue
- [ ] Confirm combat calculations
- [ ] Check crafting recipes
- [ ] Validate flood event logic

### AWS Setup (Phase 7)
- [ ] Create AWS account
- [ ] Set up EC2 instance (t3.medium)
- [ ] Configure RDS PostgreSQL
- [ ] Set up S3 for assets
- [ ] Configure CloudFront CDN
- [ ] Enable CloudWatch monitoring

### Server Implementation
- [ ] Implement authoritative game loop
- [ ] Set up database schema
- [ ] Create player persistence
- [ ] Implement anti-cheat
- [ ] Set up logging/monitoring

### Client Optimization
- [ ] Profile memory usage
- [ ] Optimize terrain rendering
- [ ] Implement LOD system
- [ ] Add asset streaming

---

## Testing

### Unit Tests
```bash
cargo test -p antediluvia_core
cargo test -p antediluvia_ai
```

### Integration Tests
```bash
cargo test --all
```

### Manual Testing
1. Launch client
2. Walk around Havilah
3. Press M to view map
4. Press I to view inventory
5. Approach NPCs (Noah, Methuselah)

---

## Performance Targets

| Metric | Target | Current |
|--------|--------|---------|
| FPS | 60 | TBD |
| Memory | <500MB | ~200MB |
| Load Time | <5s | TBD |
| Network Latency | <100ms | N/A |

---

## Funding Roadmap

### Month 1-3: Alpha (Bootstrapped)
- Cost: $70/month
- Players: 100 concurrent
- Revenue: $0 (free tier)

### Month 3-6: Beta (Grants + Patreon)
- Cost: $235/month
- Players: 500 concurrent
- Revenue: $500/month (Patreon target)

### Month 6-12: Launch (Angel Round)
- Cost: $780/month
- Players: 2,000+ concurrent
- Revenue: $5,000/month (Premium tier)

### Year 2+: Self-Sustaining
- Cost: $3,000/month (scaling)
- Players: 10,000+ concurrent
- Revenue: $50,000+/month

---

## Monitoring & Logging

### Key Metrics
- Active players
- Server CPU/memory
- Network latency
- Error rates
- Corruption meter (global)

### Logging Strategy
- Client logs: Local file + CloudWatch
- Server logs: CloudWatch + DataDog
- Database logs: RDS native

---

## Rollback Plan

### If Server Crashes
1. Restore from latest backup
2. Notify players of downtime
3. Compensate with in-game rewards

### If Corruption Meter Breaks
1. Reset to last known state
2. Verify all player actions
3. Replay events from checkpoint

### If Flood Event Triggers Early
1. Pause the flood
2. Investigate cause
3. Reset to pre-flood state
4. Relaunch with fix

---

## Post-Launch Support

### Week 1
- Monitor server stability
- Fix critical bugs
- Gather player feedback

### Week 2-4
- Implement balance changes
- Add quality-of-life features
- Optimize performance

### Month 2+
- Plan Phase 7 (Server)
- Design Phase 8 (Advanced Features)
- Expand content

---

## Contact & Support

**Discord:** [TBD]
**Email:** [TBD]
**GitHub:** [TBD]

---

## License

Proprietary. All rights reserved.

---

**Next Step:** Phase 7 (Server Implementation)
