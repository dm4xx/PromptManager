Excellent point — GitHub-hosted runners have **no GUI or interactive input environment**, which introduces several **important constraints** for simulating user interactions like **key presses**, especially for frameworks like **Slint** that depend on runtime UI states.

Here’s an updated and **corrected review** of the test plan, specifically addressing the limitations around **keypress emulation** and **UI interactions** in GitHub CI. I’ll identify which test types remain feasible and where **manual or local testing is required**.

---

# ✅ Revised Test Plan: GitHub CI-Compatible Test Scenarios

| Test Area                    | Status in GitHub CI       | Notes                                                                                  |
| ---------------------------- | ------------------------- | -------------------------------------------------------------------------------------- |
| 🧪 TextEdit State Logic      | ✅ Fully Automatable       | Use unit tests in `cargo test` to simulate data/state updates                          |
| 🧪 LLM Call Triggering Logic | ✅ Automatable (Mocked)    | Use mock clients to simulate response and rate limits                                  |
| 🧪 Prompt Assembly Logic     | ✅ Automatable             | Concatenation and format logic testable via pure Rust                                  |
| 🧪 Rollback via GUI or Key   | ⚠️ **Partial**            | UI rollback button can be tested with mocks; **key-press (Ctrl+Ctrl)** is NOT testable |
| 🧪 Slint UI Rendering Tests  | ⚠️ Partially testable     | Can be tested **locally with Slint’s testing harness**, but **not in headless CI**     |
| 🧪 Checkbox/Radio Logic      | ✅ Via State Simulation    | Use Slint model bindings and simulate value changes programmatically                   |
| 🧪 Timed Triggers (40s etc.) | ✅ With mocked time        | Abstract timer logic for testing with time mocking (e.g., `tokio::time::pause`)        |
| 🧪 Rate Limiting             | ✅ Automatable             | Counter-based enforcement works in headless CI                                         |
| 🧪 Ambiguity and Persona LLM | ✅ If LLM calls are mocked | Unit/integration tests with mocked LLM output                                          |
| 🧪 UI Card Selections        | ⚠️ Needs local Slint test | Slint click simulation **can’t be CI-run** without headless graphics backend           |

---

## 🔴 What **Cannot** Be Tested in GitHub CI

These tests require **manual testing or local automation frameworks** (or `cargo test` run in a windowed environment):

| Scenario                          | Why It Can’t Run in CI                                      |
| --------------------------------- | ----------------------------------------------------------- |
| `Ctrl + Ctrl` rollback test       | No keypress support in headless GitHub runners              |
| Card click simulations            | No real event loop/UI thread in CI                          |
| Rendering of visual card elements | Requires a graphics backend                                 |
| Undo via visual interactions      | Needs a real UI session or Slint test runner with GUI hooks |

---

## ✅ Recommendations for GitHub Automation

| Area            | Recommendation                                       |
| --------------- | ---------------------------------------------------- |
| Rollback logic  | Test via **state delta tracking**, not keypress      |
| Timer behavior  | Mock time (inject time manager)                      |
| UI logic        | Abstract UI logic into Rust structs & unit test them |
| LLM integration | Fully mockable with fixture JSON or fake clients     |

---

## ✅ Suggested Test Split Strategy

| Layer             | Test Type               | Where to Run      |
| ----------------- | ----------------------- | ----------------- |
| Prompt logic      | `cargo test`            | GitHub CI         |
| Rate limit        | `cargo test`            | GitHub CI         |
| Timer logic       | `cargo test` with mocks | GitHub CI         |
| Slint UI behavior | Manual/local test       | Local dev machine |
| LLM interactions  | Mocked tests + live QA  | CI + Manual       |

---

## 🧪 Code Structure Tip: Key Separation

Structure your code for testability:

* **Pure logic** (prompt, timers, LLM request triggers):
  Put in `mod logic`, fully unit-tested

* **UI interaction handlers**:
  Wrap in `mod ui`, test with mocks

* **Undo/rollback manager**:
  Track app state in a history stack, test with unit functions like `push_state()` / `restore_last()`

---

Would you like a ready-made Rust module that defines a `PromptStateHistory` stack + mock LLM client for these tests?
