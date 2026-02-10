# ANTEDILUVIA DESIGN BIBLE - VOLUME X: THE DEVELOPER'S HANDBOOK

**Version:** 1.0 Alpha
**Target:** AI Agents (Windsurf/Cascade) & Human Auditors

---

## 1. The Code of Conduct (Development)

We are building a cathedral, not a shack. Every line of code must be intentional.

### 1.1 Rust Best Practices
*   **Clippy is Law:** No code is committed until `cargo clippy` returns zero warnings.
*   **Formatting:** `cargo fmt` must be run on save.
*   **Error Handling:** Use `thiserror` for library errors and `anyhow` for application errors. Never use `expect()` without a paragraph explaining why the panic is impossible.

### 1.2 Bevy ECS Patterns
*   **Systems:** Keep systems small. One job per system (`move_player`, `check_collision`, `render_map`).
*   **Queries:** Use `Changed<T>` filters to avoid processing static entities.
*   **States:** Use Bevy States (`GameState::Loading`, `GameState::Playing`, `GameState::TheFlood`) to manage logic flow.

---

## 2. Workflows

### 2.1 The "Cascade" Workflow
When asking the AI to write code, use this format:
1.  **Cite the Volume:** "Refer to Volume II (Geography) of the Design Bible."
2.  **Define the Goal:** "Implement the Simplex Noise generator for the Havilah zone."
3.  **Set Constraints:** "Ensure it uses the Seed 'Genesis 6:14' and outputs a heightmap `Vec<f32>`."

### 2.2 The Asset Pipeline
*   **Models:** All 3D models must be `.glb` (glTF binary).
*   **Textures:** `.ktx2` for GPU efficiency.
*   **Hot Reloading:** Enable `bevy/file_watcher` feature during dev to tweak assets without restarting.

---

## 3. Future Roadmap (Post-Alpha)

*   **Phase 2:** Mobile Client Optimization (Touch Controls).
*   **Phase 3:** VR Support (The ultimate "Diegetic" experience).
*   **Phase 4:** The "Babel" Expansion (Post-flood mechanics).

---

## 4. Final Exhortation

*"Except the Lord build the house, they labour in vain that build it."* - Psalm 127:1

Build with integrity. Code with discipline. Create a world that matters.

**END OF VOLUME X**
