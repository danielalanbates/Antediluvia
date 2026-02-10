# ANTEDILUVIA DESIGN BIBLE - VOLUME VII: MLOps & AI INTEGRATION

**Version:** 1.0 Alpha
**Role:** The "Brain" of the World
**Tools:** Rust-Bert, Llama-3, Python Analysis

---

## 1. The "Logos" Engine (LLM NPCs)

We do not use pre-written dialogue trees. Every conversation is generated, but strictly bounded.

### 1.1 The Architecture
*   **Model:** Llama-3-8B (Quantized to 4-bit) running locally on the user's client (if hardware allows) or Llama-3-70B via Server API (for Premium/Complex interactions).
*   **Integration:** `rust-bert` or `llm` crate in Rust.

### 1.2 The System Prompt (The Guardrails)
To prevent the AI from breaking immersion (e.g., mentioning iPhones or modern theology), we use a strict persona injection.

```text
SYSTEM PROMPT:
You are an elder living in the year 1650 Anno Mundi (Year of the World).
You reside in the Land of Havilah.
You know only what is written in Genesis 1-5 and the Book of Enoch.
You DO NOT know about: Jesus, The Cross, Israel, Moses, or the Flood (it hasn't happened yet).
You speak in an archaic, King James-style syntax but accessible.
If asked about the "Sons of God," refer to the Sethites.
If asked about the "Giants," speak with fear and whisper.
Current World State: Corruption is at {corruption_level}%. The sky is {weather_state}.
```

### 1.3 RAG (Retrieval Augmented Generation)
*   **The Knowledge Base:** A vector database containing:
    *   Genesis Chapters 1-6.
    *   1 Enoch.
    *   The Book of Jubilees.
    *   Antiquities of the Jews (Josephus) - Antediluvian sections only.
*   **Process:** When a player asks "Who is Tubal-Cain?", the system queries the Vector DB, retrieves the relevant verses, and feeds them to the LLM to construct the answer.

---

## 2. Procedural World Generation (The "Genesis" Algo)

### 2.1 MLOps in Terrain
While the base terrain is Noise, we use a **GAN (Generative Adversarial Network)** trained on heightmaps of real geological canyons to "upscale" the detail.
*   **Training Data:** Grand Canyon, Amazon Basin, Kilimanjaro.
*   **Runtime:** The client generates the coarse noise. The GAN (running via `tch-rs` / Torch) refines the cliff faces to look eroded and realistic.

---

## 3. The "Watcher" Anti-Cheat System

We rely on Behavioral Analysis rather than kernel-level spying.

### 3.1 Anomaly Detection
*   **Model:** An Isolation Forest algorithm trained on "Normal Player Movement."
*   **Input Features:** Input frequency, turn speed, reaction time, loot speed.
*   **Detection:**
    *   **Botting:** Perfect, rhythmic inputs trigger a flag.
    *   **Speedhacks:** Movement vectors exceeding the physics cap trigger a rollback.
*   **Response:** The "Angel of Judgment." A game master (or AI agent) teleports to the player. If they fail a Turing Test (Conversation), they are banned.

---

## 4. Dynamic Difficulty (The Flood Gauge)

### 4.1 The Corruption Monitor
A Reinforcement Learning (RL) agent monitors the server balance.
*   **Goal:** Maintain a 40% Player Death Rate in the wilderness.
*   **Action Space:**
    *   Increase/Decrease Nephilim Spawn Rate.
    *   Change Weather severity.
    *   Modify predator aggression radius.
*   **Logic:** If players are winning too easily (Death Rate < 10%), the AI "hardens" the world (The Watchers get angry). If players are quitting from frustration, it offers "Grace" (more medicinal herbs spawn).

---

**END OF VOLUME VII**
