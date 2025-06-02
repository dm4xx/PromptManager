# 🧠 Prompt Enhancement Application

### Feature Definition & Implementation Guidelines (Rust + Slint)

---

## 📦 General Architecture & Constraints

* Application is built with:

  * **Frontend/UI**: Slint
  * **Backend/Logic**: Rust
* Prompt construction is based on five distinct `TextEdit` inputs:

  1. **Role**
  2. **Context**
  3. **Task**
  4. **Constraints & Format**
  5. **Example Output**
* The **final prompt** is dynamically built from concatenating the content of these five fields in fixed order.

### Global Rules

| Rule                                         | Description                                                                                                                                                |
| -------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **LLM calls**                                | Limited to **10 per minute**                                                                                                                               |
| **Stateless LLM**                            | All calls are atomic and carry no memory                                                                                                                   |
| **No LLM use before 100 characters**         | None of the LLM-triggering features should activate before 100 characters are entered into the Task `TextEdit`                                             |
| **Auto-trigger interval**                    | Role/persona inference and ambiguity checks are automatically re-evaluated: <br> → Every **40 seconds**, OR <br> → Every **+100 characters typed** in Task |
| **Rollback**                                 | Implemented globally via **GUI “Undo” button** or **double Left Ctrl key press**                                                                           |
| **No speculative suggestion before trigger** | All LLM-enhanced features must be explicitly triggered or gated behind character/interval thresholds                                                       |

---

## ✳️ Feature 1: Add Context

### Description

Inserts a predefined domain-specific context into the **Context** `TextEdit`.

### Implementation

* UI Element: Clickable horizontal card selector (roles/domains)
* On click:

  * Append phrase into Context:
    `"You are a [Selected Role] helping with [default topic]"`
* LLM infers two pairs of (Role/topic) which will then be presented as two cards

---

## ✳️ Feature 2: Identify Ambiguities

### Description

Detects and surfaces ambiguities in the **Task** section using an LLM.

### Implementation

* Trigger:

  * User click: **“Check for Ambiguities”**
  * Auto: (Only if Task content ≥ 100 characters) Every **40 seconds** OR on **+100 characters** in Task
* Sends LLM call with Task text
* Response includes up to **3 unclear elements**
* UI presents them as labeled tags with predefined dropdowns for clarification
* Selected clarifications are injected into Task or Context as applicable

---

## ✳️ Feature 3: Infer Target Persona

### Description

The user persona is inferred from the **Task** content and shown visually. No manual persona selection is needed.

### Implementation

* Trigger:

  * Auto-triggered LLM call every 40 seconds OR after Task grows by 100 characters
  * Minimum Task length: 100 characters
* Injected LLM result appears in **Role** `TextEdit`:
  → e.g., `"You are responding to a request from a data analyst with Python experience."`
* No user action is required to set persona

---

## ✳️ Feature 4: Add Output Format

### Description

User can define the preferred response format.

### Implementation

* UI Element: Checkbox group labeled "Preferred Output Format"
* Formats:

  * Markdown (default)
  * Bullet Points
  * Table
  * Code Block
  * JSON
* On click:

  * Add format hint to **Constraints & Format** `TextEdit`:
    → e.g., `"Respond using Markdown."`, `"Use a table format with appropriate headers."`
* No LLM call involved

---

## ✳️ Feature 5: Generate Example Outputs

### Description

Automatically generates two example outputs to guide the LLM.

### Implementation

* UI Element:

  * `TextEdit` under **Example Output**
  * Checkbox: `[ ] Generate 2 Example Outputs`
* Trigger:

  * Only upon **checking the box**
  * Sends full prompt (concatenated from the 4 other fields) to LLM
* LLM returns 2 examples to be diplayed as cards (with a maximum number of characters) and the user will choose one

  * Populated into Example Output `TextEdit` as:

    ```
    Example 1:
    [Generated content]

    Example 2:
    [Generated content]
    ```

---

## ✳️ Feature 6: Create Summary Table with Column Suggestion

### Description

Generates a list of column names to include in a summary table.

### Implementation

* UI Element: “📊 Generate Table Columns” button
* Trigger:

  * Manual click only
  * Only active if prompt text ≥ 100 characters
* Sends Task and Context to LLM
* Response: a prompt instruction to create a table with the column names → inserted in **Constraints & Format**:
  → `"Include a table with columns: Term, Description, Value"`

---

## 🧩 Final Prompt Construction Logic

* Live preview of full prompt assembled from the five `TextEdit` fields:

```text
1. Role:
[Role field text]

2. Context:
[Context field text]

3. Task:
[Task field text]

4. Constraints and Format:
[Constraints field text]

5. Example Output:
[Example field text]
```

