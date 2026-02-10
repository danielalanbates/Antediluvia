# ANTEDILUVIA DESIGN BIBLE - VOLUME I: THE FOUNDATION & ARCHITECTURE

**Version:** 1.0 Alpha
**Date:** January 2026
**Target Architecture:** Rust / Bevy ECS
**Target Hardware:** Native macOS (Metal) / Windows (Vulkan)
**Author:** Bezalel (Lead Architect Agent)

---

## 1. Executive Summary & Vision

**Antediluvia: The Tenth Generation** is not merely a game; it is a digital simulation of the pre-flood world designed to test the moral fiber, discipline, and cooperative capacity of its players. It rejects the "skinner box" psychology of modern MMORPGs (daily logins, cash shops, glowing trails) in favor of a "refiner's fire."

### 1.1 The Core Pillars

1.  **Integrity (The Anti-Cheat Ethos):** The game world is built on strict rules. Cheating, exploitation, and pay-to-win mechanisms are aggressively countered not just by policy, but by ML-driven anomaly detection. The "Line of Seth" represents fair play; the "Line of Cain" represents exploitation. This narrative conflict is baked into the server architecture.
2.  **Discipline (The Progression Ethos):** There are no level skips. There is no "fast travel" teleportation. If you wish to cross the continent of Pangea, you must walk. If you wish to become a Master Swordsman, you must train. The reward is the journey itself and the "harvest of peace" that comes from legitimate accomplishment.
3.  **Permanency (The World Ethos):** The world does not reset weekly. It is a persistent simulation headed toward a singular, cataclysmic event: The Flood. Every plank Noah nails to the Ark is permanent. Every tree cut down in the Gopher Wood remains a stump. The world remembers.
4.  **Social Reliance (The Community Ethos):** Inspired by the "forced grouping" of FFXI (Final Fantasy XI) and early EverQuest. A player cannot survive the wilderness alone. The Nephilim and the corrupted fauna are designed with `Pack_Tactics` and high HP pools that mathematically require a "Trinity" (Tank, Healer, DPS) to defeat. Loneliness is dangerous; community is survival.

---

## 2. Technical Architecture (The "Ark" Engine)

The technical stack is chosen for **maximum efficiency** and **memory safety**. We are building on the "Rock" of Rust, avoiding the "Sand" of garbage-collected languages that cause micro-stutters and frame drops.

### 2.1 The Tech Stack

*   **Language:** **Rust (Edition 2024)**.
    *   *Why:* Zero-cost abstractions, memory safety without garbage collection, and strict type systems prevent entire classes of bugs (null pointer exceptions, race conditions) at compile time.
*   **Game Engine:** **Bevy 0.15+**.
    *   *Why:* Bevy is a data-driven Entity Component System (ECS). Unlike Object-Oriented engines (Unity/Unreal) where data is scattered in memory, ECS packs data contiguously. This allows us to process 100,000 entities (raindrops, soldiers, trees) in parallel across all CPU cores. It is "Metal-Native" on macOS via WGPU.
*   **Graphics Backend:** **WGPU**.
    *   *Why:* It translates our code directly to **Metal** (Apple Silicon) and **Vulkan/DX12** (Windows) without us needing to write separate renderers.
*   **Networking:** **bevy_renet** or **lightyear**.
    *   *Why:* We require **Rollback Netcode**. In a high-stakes combat game where a single parry matters, we cannot wait for the server to confirm an action. The client predicts the outcome, and the server corrects it if wrong. This provides "offline-feeling" responsiveness.
*   **Database:**
    *   **Client-Side:** `Sled` (Embedded Rust DB). Used to store local map chunks, user preferences, and cached assets.
    *   **Server-Side:** `PostgreSQL` accessed via `sqlx`. Fully async, type-safe queries. This stores the "Book of Life" (Player Character Data) and the "World State" (Corruption Meter, Ark Progress).
*   **AI/MLOps:**
    *   **Inference:** `rust-bert` binding to a quantized **Llama-3-8B** model running locally or on a dedicated inference server.
    *   **Procedural Gen:** `noise-rs` for deterministic terrain generation.

### 2.2 The Workspace Structure

To ensure modularity and fast compile times, the project is organized as a Cargo Workspace.

```text
antediluvia/
├── Cargo.toml                  # Workspace Root
├── assets/                     # glTF models, textures, audio (Git LFS)
├── crates/
│   ├── antediluvia_core/       # THE LAW. Shared logic, math, RNG, NetProtocol.
│   │                           # Both Client and Server import this.
│   ├── antediluvia_server/     # THE JUDGE. Authoritative headless server.
│   │                           # Handles Physics verifications, AI State, DB.
│   ├── antediluvia_client/     # THE VIEWER. Graphics, Audio, Input.
│   │                           # Contains no game logic that isn't prediction.
│   ├── antediluvia_ai/         # THE ORACLE. LLM bindings, NPC brains.
│   └── antediluvia_procgen/    # THE CREATOR. World generation algorithms.
└── tools/
    └── launcher/               # Rust-based auto-updater and launcher.
```

### 2.3 The "Windsurf" Protocol (Coding Standards)

Since this codebase is primarily generated by AI (Windsurf/Cascade), strict protocols must be observed to prevent "Hallucinations" or "Drift."

1.  **The "No Unwrap" Rule:** `unwrap()` is forbidden in production code. It causes panics (crashes). All errors must be handled via `Result<T, E>` and propagated or logged.
2.  **Documentation First:** Every public struct, enum, and function must have a docstring (`///`) explaining *why* it exists in the context of the Biblical setting.
    *   *Bad:* `/// Calculates damage.`
    *   *Good:* `/// Calculates damage based on the 'Weight of Sin' mechanic. Bronze weapons deal +10% vs Nephilim.`
3.  **ECS Purity:** Do not store state in "Global Variables." Everything must be a **Component** (attached to an Entity) or a **Resource** (Global singleton managed by Bevy).
4.  **Tests:** Critical logic (Combat formulas, Crafting outcomes) must have unit tests in `antediluvia_core`.

---

## 3. Deployment & Distribution

### 3.1 The "Free Tier" Mandate
As per the Creator's directive:
*   **Free Tier:** Unlimited access to the base game. Uses **DeepSeek** (or Groq backup) for NPC conversations.
*   **Premium Tier:** Access to "High-Fidelity" AI models (Llama-3-70B via cloud) for richer, more prophetic NPC interactions. Cosmetic "Tunic of Support."
*   **No P2W:** Premium players gain NO stat advantages.

### 3.2 Cross-Platform Strategy
1.  **macOS:** Distributed as a signed `.app` bundle / `.dmg`. Optimized for M1/M2/M3/M4 chips.
2.  **Windows:** Distributed as an `.exe` installer.
3.  **Mobile (Future):** The `antediluvia_client` crate is designed to be compiled to iOS/Android targets, but the UI will need a "Touch Overlay" (Phase 2).

---

**END OF VOLUME I**
