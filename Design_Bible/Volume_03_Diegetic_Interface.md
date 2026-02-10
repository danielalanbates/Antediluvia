# ANTEDILUVIA DESIGN BIBLE - VOLUME III: THE DIEGETIC INTERFACE & IMMERSION

**Version:** 1.0 Alpha
**Philosophy:** "The Screen is the Eye"
**Constraint:** Zero UI Elements (HUD-less)

---

## 1. The Philosophy of "No UI"

In *Antediluvia*, there are no health bars floating above heads. There are no damage numbers popping up like popcorn. There is no mini-map in the corner. The screen represents the *retina* of the character. If the character cannot see a "hit point," the player cannot see it.

This forces players to pay attention to the world, not the interface.

---

## 2. Vitality Visualization (Health & Stamina)

### 2.1 Health (The Body State)
Instead of a Red Bar, we use **Audio-Visual Feedback**:
*   **100-75% HP:** Character stands tall. Breathing is silent. Movement is crisp.
*   **74-50% HP:** Light bruising on skin textures. Breathing becomes audible during sprinting.
*   **49-25% HP:** Character hunches slightly. Breathing is ragged/labored. Blood overlays appear on the edges of the screen (tunnel vision). Movement speed is reduced by 10%.
*   **24-0% HP:** Character limps. Vision blurs/pulses. Heartbeat audio drowns out ambient sound. Death is imminent.

### 2.2 Stamina (The Breath)
*   **Mechanic:** Sprinting, jumping, and swinging heavy weapons consume Stamina.
*   **Feedback:** The sound of the character's intake of breath. A sharp gasp indicates full depletion. The screen slightly desaturates (loses color) when exhausted.

---

## 3. The Navigation System (The Paper Map)

Modern GPS has no place in the Antediluvian age.

### 3.1 The Map Item
*   **Item Type:** `MapItem` (e.g., "Scroll of Havilah", "Tablet of Nod").
*   **Usage:**
    1.  Player presses `M` (or assigned key).
    2.  **Animation:** The character's left hand raises the physical item into the bottom half of the screen. The camera focus shifts to the map.
    3.  **Realism:** The map sways with the character's breathing. It is affected by world lighting—if it is night, you need a torch to read the map.
*   **No "You Are Here":** The map is a drawing. It does not know where you are. You must look at the "Pillar of Eden" (Center) and the "Mountains of the North" and triangulate your position mentally.

### 3.2 Compasses & Orientation
*   **The Compass:** A "Lodestone" on a string.
*   **Usage:** Held in hand. It points toward Magnetic North (or towards Eden, depending on the item enchantment).

---

## 4. The Inventory System (The Satchel)

Pressing `I` does not open a grid of icons.

### 4.1 The 3D Bag
*   **Usage:**
    1.  Player presses `I`.
    2.  **Animation:** The camera pans down to the character's hip/satchel. The flap opens.
    3.  **View:** The player sees the *actual 3D models* of their items inside the bag.
*   **Organization:** Items are physically slotted. A sword hangs on the belt loop. Potions are in loops. Scrolls are rolled up.
*   **Capacity:** Limited by volume, not "slots." You can fit many small gems, but only one large stone tablet.
*   **Interaction:** The mouse cursor unlocks. You click and drag the 3D object from the bag to your "Hand" (Equip) or to the "Ground" (Drop).

---

## 5. Dialogue & Interaction

### 5.1 NPC Interaction
*   **Trigger:** Proximity. You do not click NPCs. You walk up to them.
*   **The "Look" Mechanic:** If you center your camera on an NPC's face, they will make eye contact and acknowledge you.
*   **Conversation:**
    *   No multiple-choice boxes.
    *   **Input:** You type (or speak via microphone) your question.
    *   **Output:** The NPC speaks (Text-to-Speech + Subtitles floating near their mouth).
    *   **Context:** "Greetings, traveler. The rain smells heavy today."

### 5.2 Looting
*   **Visual Loot:** When a wolf dies, it does not turn into a loot bag. It remains a corpse.
*   **Harvesting:** To get fur, you must use a `Skinning Knife`. You physically see the character kneel and work.
*   **Drop:** If a Nephilim drops a sword, the sword physically falls to the ground with physics. You must look at the ground and pick it up.

---

**END OF VOLUME III**
