# ANTEDILUVIA DESIGN BIBLE - VOLUME II: THE WORLD OF ANTEDILUVIA

**Version:** 1.0 Alpha
**Focus:** Geography, Biomes, Procedural Generation
**Seed:** "Genesis 6:14"

---

## 1. The Pangea Ultima Protocol

The world of Antediluvia is not a fantasy map drawn by an artist. It is a **procedural reconstruction** of the theoretical supercontinent "Pangea Ultima" (or Proxima), modified to fit the biblical narrative of a singular landmass before the "fountains of the deep" broke it apart.

### 1.1 Scale & Scope
*   **The Scale:** 1:70.
    *   This ratio allows for a world that feels massive but is playable.
    *   1 Kilometer in real life = ~14.2 meters in-game space.
    *   However, we treat the "Unit" mapping as **1.0 Unit = 1.0 Meter** in engine, and simply scale the *distance* between cities.
*   **Total Playable Area:** ~30,200 km².
    *   Comparatively: This is roughly the size of **Belgium** or **Maryland**.
    *   Traversing from West to East on foot (walking speed) takes approximately **30 real-time hours**. This enforces the "Journey" aspect.

### 1.2 The Deterministic Generation
To avoid storing terabytes of map data, the client generates the world locally using a **Seed**.
*   **The Seed:** `Genesis 6:14` (Hashed to u32).
*   **The Algorithm:**
    1.  **Base Layer:** Simplex Noise (Low Frequency) defines the "Continent" vs "Ocean".
    2.  **The "C" Mask:** A radial gradient function forces a C-shape, wrapping around a central point (0,0).
    3.  **Erosion Pass:** A hydraulic erosion simulation runs for "1000 iterations" during the loading screen to carve realistic riverbeds and valleys.

---

## 2. Zonal Geography

### 2.1 The Center: The Garden of Eden (0,0)
*   **Status:** **FORBIDDEN ZONE**.
*   **Visuals:** A massive plateau, rising 2,000 meters sheer from the plains.
*   **The Guardian:** The "Flame of the Cherubim." A spinning, procedural shader effect that circles the plateau.
*   **Mechanic:** Any player attempting to fly, climb, or glitch into this zone is instantly vaporized by a "Holy Fire" damage tick (Infinite Damage).
*   **Purpose:** It acts as the "North Star" of the world. No matter where you are, you can see the Pillar of Fire in the distance. It is your compass.

### 2.2 The Inner Ring: The Land of Havilah
*   **Biblical Ref:** Genesis 2:11 *"where there is gold; and the gold of that land is good: there is bdellium and the onyx stone."*
*   **Biome:** Lush, golden savannahs and sparkling river deltas.
*   **Function:** **The Starter Zone**.
*   **Resources:** Abundant food, basic copper, gold (currency), herds of sheep.
*   **Threat Level:** Low. Predators are wolves and lions, manageable by duos.

### 2.3 The North: The Gopher Wood Forests
*   **Biome:** Primordial Gigantic Forest.
*   **Visuals:** Trees are 200+ feet tall. The canopy blocks 80% of sunlight, creating a perpetual twilight. Bioluminescent fungi light the paths.
*   **Landmark:** **The Ark Construction Site**.
    *   A massive clearing where Noah and his sons are working.
    *   The structure changes over server-time (Keel -> Ribs -> Planking -> Pitch).
*   **Threat Level:** High. This is the domain of the **Nephilim Scouts** who seek to burn the project.

### 2.4 The East: The Land of Nod / City of Enoch
*   **Biblical Ref:** Genesis 4:16-17.
*   **Biome:** Industrial Wasteland / High Desert.
*   **Visuals:** Smog chokes the air. The ground is cracked. Massive bronze foundries pump black smoke.
*   **Landmark:** **The City of Enoch**.
    *   A fortress of iron and bronze.
    *   Architecture is brutalist, sharp, and imposing.
    *   Music (Jubal's Lyres) plays constantly here—hypnotic and aggressive.
*   **Threat Level:** Extreme for Sethite players. PvP Zone.

### 2.5 The Hidden Zone: The Valley of Bethel
*   **Biome:** High Alpine Mountains.
*   **Secret:** Hidden deep within a cloud-covered peak.
*   **The Anomaly:** **Jacob's Ladder** (The Stairway to Heaven).
    *   This is a "Time Rift" (since Jacob is born post-flood, this is a prophetic vision location).
    *   Visual: A distortion in the air where entities of light (Angels) can be seen ascending and descending.
    *   Mechanic: Sleeping here restores "Spirit" (Mana) 500% faster and grants prophetic visions.

---

## 3. Environmental Mechanics

### 3.1 The Day/Night Cycle
*   **Duration:** 1 Game Day = 4 Real Hours.
*   **Night:** Night is *truly* dark. Players require torches or "Light of Truth" abilities.
*   **The Firmament:** The skybox is not a standard galaxy. It represents the "Waters Above." It shimmers like an ocean during the day and reveals "The Watchers" (stars that move incorrectly) at night.

### 3.2 The Weather (The Approaching Storm)
*   **Standard:** Mist, Light Rain, Heavy Heat.
*   **Dynamic:** Weather is global. If it rains in Havilah, it rains in Havilah for everyone.
*   **The Corruption Link:** As the Global Corruption Meter rises, the weather worsens.
    *   0-50%: Clear skies.
    *   51-80%: Perpetual overcast.
    *   81-99%: Thunderstorms, red lightning.
    *   100%: **The Deluge** (Game Over Event).

---

**END OF VOLUME II**
