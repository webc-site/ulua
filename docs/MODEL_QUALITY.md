# Per-model quality analysis

This is the data behind the central lesson of the project: **for code translation,
"it compiled" is a weak signal — rank models by whether their output _survives_ against a
real test oracle.** Snapshot taken **2026-06-24**, before that day's endgame correctness
edits, so the survival numbers reflect the batch's own output, not later hand-fixes.

The question that started it: *"qwen looks like the volume leader — is it actually good?"*
Answer at the bottom: no.

## 1. Provenance — who landed the 12,340 translated items

- **Batch (model-attributed): 9,552 (77.4%)**
- **Hand-ported / clusters (no model): 2,788 (22.6%)**

| model | items |
|---|---:|
| qwen/qwen3-coder-next | 3,602 |
| *(none — hand/clusters)* | 2,788 |
| google/gemini-3-flash-preview | 2,774 |
| openai/gpt-5.4-nano | 1,593 |
| google/gemini-3.1-flash-lite | 1,133 |
| google/gemini-3.5-flash | 288 |
| deepseek/deepseek-v4-flash | 105 |
| xiaomi/mimo-v2.5 | 57 |

Caveat: this counts tracked items (library + 1,957 tracked test nodes). The larger inline
test-port effort (4,886/5,226 files) was written outside the topology state machine and is
in neither bucket, so the batch's true share of *all* Rust is lower than 77%.

## 2. Acceptance (at landing) — one-shot accept & revert rate

`accept = first_try / evaluated`, where `evaluated = first_try + repaired + reverted`
(API failures & cascade-reverts excluded — not quality signals).

| model | evaluated | accept (1st-try) | revert |
|---|---:|---:|---:|
| google/gemini-3.1-flash-lite | 4,569 | **82.3%** | 16% |
| google/gemini-3-flash-preview | 501 | **81.2%** | **11%** |
| xiaomi/mimo-v2.5 | 83 | 80.7% | 8% |
| openai/gpt-5.4-nano | 4,448 | 79.1% | 19% |
| qwen/qwen3-coder-next | 5,254 | 76.9% | 20% |
| google/gemini-3.5-flash | 175 | 68.6% | 17% |
| deepseek/deepseek-v4-flash | 116 | 63.8% | 28% |

## 3. Survival — model output still unchanged in the final tree

Each model's originally-landed file compared against the file on disk today
(`unchanged = identical + formatting-only`).

| model | landed | **still unchanged** | modified |
|---|---:|---:|---:|
| google/gemini-3.5-flash | 288 | 73.3% | 77 |
| xiaomi/mimo-v2.5 | 57 | 63.2% | 21 |
| google/gemini-3-flash-preview | 2,774 | 61.8% | 1,060 |
| deepseek/deepseek-v4-flash | 105 | 57.1% | 45 |
| google/gemini-3.1-flash-lite | 1,133 | 51.9% | 545 |
| openai/gpt-5.4-nano | 1,593 | 42.1% | 923 |
| qwen/qwen3-coder-next | 3,602 | **24.6%** | 2,717 |
| **TOTAL** | **9,552** | **43.6%** | **5,388** |

For qwen, even discounting `cargo fmt` reflow **and** the snakeify casing migration:
identical 803 / fmt-only 351 / case-only 4 / **genuine rewrite 2,444 (67.9%)**. Formatting
explains ~10%, snakeify ~0%; the rest is real rewriting.

## 4. Subsystem-controlled — real-change rate within `luau-analysis/methods`

Controls for the fact that the type-inference engine is re-edited far more than other
subsystems regardless of author.

| model | identical | real | **real-change rate** |
|---|---:|---:|---:|
| qwen/qwen3-coder-next | 364 | 878 | **71%** |
| openai/gpt-5.4-nano | 308 | 279 | 48% |
| deepseek/deepseek-v4-flash | 19 | 10 | 34% |
| google/gemini-3.1-flash-lite | 228 | 102 | 31% |
| google/gemini-3-flash-preview | 123 | 7 | **5%** |
| google/gemini-3.5-flash | 14 | 0 | 0% |

## 5. Recency — per-model median translation date

Tests the "oldest files just absorbed the most later passes" hypothesis.

| model | n | earliest | median | latest |
|---|---:|---|---|---|
| google/gemini-3-flash-preview | 2,774 | 06-08 | **06-09** | 06-14 |
| google/gemini-3.1-flash-lite | 1,133 | 06-09 | 06-11 | 06-15 |
| openai/gpt-5.4-nano | 1,593 | 06-09 | 06-11 | 06-17 |
| qwen/qwen3-coder-next | 3,602 | 06-09 | **06-12** | 06-20 |

## Conclusion: why qwen looks terrible

Four hypotheses; three busted, one holds.

- **❌ Formatting** — discounting whitespace + `cargo fmt` reflow, only ~10% of qwen's churn
  is cosmetic.
- **❌ Snakeify migration** — only a handful of files differ solely by identifier casing.
- **❌ Recency** — *reversed* in the data: qwen has the **latest** median date (06-12) yet
  the highest churn, while the oldest model (gemini-3-flash-preview, 06-09) has the **lowest**
  (5% rework in analysis methods). Age does not explain it.
- **✅ Genuine quality gap** — three independent signals converge:
  1. Same subsystem (analysis methods): qwen rewritten **71%** vs gpt-nano 48% /
     gemini-3.1 31% / gemini-3-flash-preview **5%**.
  2. Revert rate **20%** — second-worst among the workhorses.
  3. **Mechanism: "first-try" means *compiled*, not *correct*.** qwen's healthy one-shot
     accept (77%) overstates quality, because compiling-but-wrong code (e.g. a `restorestack`
     pointer cast whose precedence flipped to `as *mut u8.add(n)`) passes the build/drift
     gate, then gets rewritten during the correctness grind against the test suite. That is
     the 77% → 25% survival collapse.

**Amplifier (not cause):** qwen carried the most volume (3,602), concentrated in the hardest,
most-revised subsystems (type inference + JIT codegen), so a moderately-higher per-file
defect rate produced huge absolute churn.

**Takeaways:**
- The real quality leader is **google/gemini-3-flash-preview** (81% one-shot, 11% revert,
  5% rework) — the slot to promote.
- Treat qwen-authored analysis/codegen files as higher-risk for correctness review (most
  likely compiling-but-wrong).
- **Rank model quality by survival against a real test oracle, not by "it compiled."** A
  compile gate is necessary but nowhere near sufficient; without the adversarial test suite
  as the second oracle, qwen would have looked like the best model on the project.
