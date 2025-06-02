

# 🎨 UI and Design Guidelines

### For Prompt Enhancement Application (Rust + Slint)

---

## 🖥️ Updated UI Layout Structure

```
--------------------------------------------------
|           PROMPT EDITOR                         |
--------------------------------------------------
| Role              |                             |
| [TextEdit]        |                             |
|------------------|        CONTROLS PANEL        |
| Context           |                             |
| [TextEdit]        |                             |
|------------------|     (Cards, Buttons, etc.)   |
| Task              |                             |
| [TextEdit]        |                             |
--------------------------------------------------
| Constraints & Format  |                         |
| [TextEdit]            |   Control row           |
-------------------------                         |
| Example Output        | (Cards, Checkboxes)     |
| [TextEdit]            |                         |
--------------------------------------------------
```

* Left column: vertically stacked input areas (Role → Example)
* Right column: responsive panel for cards, buttons, checkboxes, and state feedback
* Minimum width enforced for both columns (adaptive layout when resizing)

---

## 📑 Input Area Components

### 🔹 TextEdit Sections (Left Side)

| Field Name               | Description                                                  |
| ------------------------ | ------------------------------------------------------------ |
| **Role**                 | Auto-filled or user-confirmed persona.                       |
| **Context**              | Optional. Inserted via cards or manual input.                |
| **Task**                 | Core user prompt input. Triggers persona/ambiguity logic.    |
| **Constraints & Format** | Manually edited + auto-appended format rules via checkboxes. |
| **Example Output**       | Populated from selected LLM-generated examples.              |

---

## 🎛️ Control Panel Components (Right Side)

### 🧩 Card & Control Row Elements

#### ✅ Add Context (Feature 1)

* Horizontal **card carousel**

  * Max 2–3 visible domain suggestions
  * Each card: icon + short descriptor (e.g., `🧑‍⚖️ Legal Expert`)
* On click: appends short context sentence to `Context` TextEdit

---

#### ✅ Identify Ambiguities (Feature 2)

* **Button**: “🔍 Check for Ambiguities”
* Appears under Task
* On click or auto-trigger (≥100 chars or 40s), sends Task to LLM
* LLM returns up to 3 ambiguous elements, each with 2 clarification choices

##### UI Behavior:

* Each ambiguity displayed like:

  ```
  Ambiguity: "forecasting model"
  [ Use statistical forecasting ]   [ Use machine learning forecasting ]
  ```
* User clicks one disambiguation card → immediate LLM call with:

  * Current full prompt (Role + Context + Task + Constraints)
  * Selected disambiguation option
* LLM returns updated Task (or Context), replacing old content accordingly

---

#### ✅ Infer Persona (Feature 3)

* Triggered when Task has ≥ 100 characters and updated every 40s or 100+ characters
* LLM returns two inferred personas

##### UI Behavior:

* Displays **two horizontal persona cards** under **Role**

  ```
  [ 🧑‍💻 Data Scientist with Python ]   [ 🧑‍🏫 Educator in STEM ]
  ```
* Clicking one:

  * Inserts persona as Role (`You are responding to...`)
  * Removes both persona cards
* Action is **reversible** with global rollback mechanism

---

#### ✅ Output Format (Feature 4)

* Vertical stack of **checkboxes**:

  ```
  [✓] Markdown
  [ ] Bullet Points
  [ ] Table
  [ ] Code Block
  [ ] JSON
  ```
* On selection: appends respective format hint to Constraints & Format field
* Supports multi-select

---

#### ✅ Example Generation (Feature 5)

* Checkbox: `[ ] Generate 2 Example Outputs`

  * Triggers LLM request once checked
* Results shown as **two selectable cards** under Example field

  * Card titles: `Example 1`, `Example 2`
  * Short output preview (≤ 300 chars each)
* On selection: fills the `Example Output` TextEdit with chosen content

---

#### ✅ Table Column Generator (Feature 6)

* Button: `📊 Generate Table Columns`

  * Appears near Constraints field
  * Enabled when Task+Context ≥ 100 characters
* Injects list of recommended table columns into Constraints field:

  > `"Include a table with columns: Metric, Description, Value"`

---

## 🔄 Rollback Controls

* Global **Undo Button** in Control Row:

  > `⮌ Undo Last Change`
* Keyboard Shortcut: `Left Ctrl` pressed twice within 500ms
* Restores last state before user-invoked or LLM-driven mutation

---

## ⚙️ Additional UX Notes

| Component                   | Guideline                                                            |                      |
| --------------------------- | -------------------------------------------------------------------- | -------------------- |
| Character Threshold Tracker | Small counter near Task label → \`Characters: 134                    | Next update in 21s\` |
| LLM Call Feedback           | Loading spinner + toast if waiting/limited (`"LLM busy. Try in 5s"`) |                      |
| LLM Rate Limits             | Disable auto triggers if 10 calls used in current minute window      |                      |
| Visual Grouping             | Use spacing, dividers, and card background shading for clarity       |                      |
| Adaptive Layout             | Cards and buttons reflow on window resize or portrait display        |                      |

---

* ⚙️ Rust-side logic scaffolding and state update model
* 🧪 UI test scenarios for each card/button interaction
* 📂 File structure and module naming for Cursor AI to scaffold the implementation


