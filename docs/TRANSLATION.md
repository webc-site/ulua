# How ulua was built

ulua is a faithful Rust translation of [Luau](https://github.com/luau-lang/luau),
produced by an agent-driven pipeline that treats a codebase as data: a typed semantic
graph of ~15k nodes, translated bottom-up, validated by two independent oracles, and kept
*convergent* — defects are just more nodes to detect, atomize, fix in parallel, and
recompose, so the port stays alive as upstream moves rather than being a fragile one-shot.

This is the as-built retrospective with real numbers. (For what *equivalence* was proven,
see [`CONFORMANCE.md`](CONFORMANCE.md).)

## The numbers

| | C++ (Luau `8f33df9`) | Rust (ulua) |
|---|---|---|
| Source LOC (excl. tests) | 205,058 | 419,909 |
| Test LOC | 133,793 | ~278,000 |
| Files | — | 16,185 |
| Published crates | 1 project | 20 crates |
| Body-to-body ratio (imports/comments/blanks stripped) | 1.0× | **1.96×** |

Largest C++ subsystems translated: Analysis (type checker) 95k, CodeGen 45k, VM 27k,
Ast 16k, Compiler 11k, plus Common/Config/Require/CLI.

**Validation:** 5,347 ported unit tests pass (0 fail), all **293/293**
upstream conformance scripts run byte-identically on the Rust VM, and a byte-exact
bytecode differential confirms the compiler→VM pipeline against the reference.

**Timeline:** first commit **2026-06-09**, feature-complete + green through **2026-06-24**
— ~15 days, 783 commits, peak 165 commits in a single day.

> **Scope note.** Everything in this document — and every number above — is about the
> *engine*: the faithful C++→Rust translation of Luau's compiler, VM, and type checker.
> The ergonomic `ulua-rt` crate (the mlua-style `Lua`/`UserData` API) is a **Rust-native
> addition** with no C++ counterpart — it is hand-written, not translated, and is **not**
> part of these stats or the "faithful" claim. It exists so ulua is pleasant to *use* as a
> library; the translation is what makes the engine underneath trustworthy.

## The method

The skeleton is deliberately unsurprising — and saying so up front is the point. Graphing
a codebase and topo-sorting it is in the literature (RustMap, EvoC2Rust). The published
work tops out around ~13k lines of C at ~87% equivalence with human patching because the
*atomization* breaks before scale. The contribution here is making the obvious approach
survive to ~205k lines of real C++17 as a convergent system. Concretely:

1. **Extract a Typed Semantic Translation Graph (TSTG).** Parse the C++ into one node per
   translatable item (record, method, free function, macro, enum), with typed edges:
   *declares*, *calls*, *type-uses*, *includes*, *inherits*. The graph — not the file tree
   — is the unit of work.
2. **One item per file.** Each node becomes its own Rust file with an explicit `use`
   header. This is why there are 16k files and why the raw line ratio (~2.05×) is higher
   than the body ratio (1.96×): the per-node import headers are real lines. The payoff is
   that every node is independently translatable, reviewable, and *re-translatable* — the
   atomization that makes the rest work.
3. **Topo-sort and translate bottom-up.** A node is only scheduled once its dependencies
   are translated. Each translation prompt is assembled from the node's C++ source *plus
   the already-translated Rust of everything it depends on* — so the model sees the real
   target types and signatures, not guesses. This per-node context engineering is the
   hard-won core.
4. **Gate every landing.** A translated node must compile in-tree and pass a drift check
   (it may not silently drop declarations, fake green by deleting `mod` entries, or stub
   out logic) before it lands. Failures are reverted and re-queued — for a different model,
   a richer context card, or the agentic layer.
5. **Converge.** Cycles (mutually-recursive types like Luau's 177-member `Type` SCC) are
   withheld from the per-item queue and landed as hand-designed clusters whose stub shape
   is a contract. Residue failures are re-translated per-module with an agentic error-clean
   loop. The loop runs until the gate is green.

## The model economics — two layers

ulua was translated by a **cheap-batch layer** for volume and an **agentic layer** for the
hard parts. They have very different cost profiles, and that split is the whole economic
argument.

**Cheap batch (the volume).** A round-robin roster of inexpensive models translated the
bulk of independent nodes through the OpenRouter boundary. Of 12,340 landed items, **9,552
(77%) came from the batch**; the other 2,788 were hand-ported foundations and cyclic
clusters. By model:

| model | landed items | one-shot accept¹ | survival² |
|---|---:|---:|---:|
| `qwen/qwen3-coder-next` | 3,602 | 76.9% | **24.6%** |
| `google/gemini-3-flash-preview` | 2,774 | 81.2% | 61.8% |
| `openai/gpt-5.4-nano` | 1,593 | 79.1% | 42.1% |
| `google/gemini-3.1-flash-lite` | 1,133 | 82.3% | 51.9% |
| `google/gemini-3.5-flash` | 288 | 68.6% | 73.3% |
| `deepseek/deepseek-v4-flash` | 105 | 63.8% | 57.1% |
| `xiaomi/mimo-v2.5` | 57 | 80.7% | 63.2% |

¹ landed on the first attempt (compiled + passed the drift gate), of those evaluated.
² model's output still unchanged in the final tree (whitespace/format-normalized).

**The most important number here is the gap between those last two columns, and it is the
central lesson of the project: _compiling is not correct._** The drift gate proves a
translation builds and didn't fake green — it does *not* prove it's semantically right.
Compiling-but-wrong code (e.g. a `restorestack` pointer cast whose precedence flipped to
`as *mut u8.add(n)`) sails through the gate as a "first-try success," then gets rewritten
during the correctness grind against the test suite. That is why `qwen` — the **volume
leader** with a healthy 77% one-shot accept — has the **lowest survival, 25%**: it produced
the most plausible-but-wrong code, concentrated in the hardest subsystems (type inference,
JIT codegen). Controlling for subsystem, qwen's analysis-engine output was rewritten **71%**
of the time versus **5%** for `gemini-3-flash-preview`.

So the real per-model quality leader is **`gemini-3-flash-preview`** (81% one-shot, 11%
revert, 5% rework) — *not* the volume leader. The full provenance / acceptance / survival
breakdown, and the four hypotheses tested to explain qwen's churn (three busted, one holds),
is in [`MODEL_QUALITY.md`](MODEL_QUALITY.md). The takeaway generalizes well beyond this
project: **rank translation models by survival against a real test oracle, never by "it
compiled."** Without the adversarial test suite as a second oracle, qwen would have looked
like the best model on the project.

**Agentic layer (the tail).** The parts cheap models can't one-shot — mutually-recursive
type clusters, the GC, the VM execution core, the type-inference engine, repair loops,
and the endgame — were driven by stronger agents: **Claude** (Claude Opus, via Claude
Code) and **Codex** (OpenAI's CLI), plus the `oneshot` multi-model patch driver for
surgical fixes. This layer isn't in `model-stats.jsonl` (it ran through the agent CLIs
directly), and it's where the genuinely hard equivalence work happened.

## Validation: two oracles, not spot checks

Equivalence validation is *the* named unsolved crux of automated translation, so it got
two independent checks:

- **The maintainers' own tests.** Luau's C++ doctest suite was ported to Rust `#[test]`s —
  5,347 of them — and must pass. These are adversarial: they were written to catch bugs in
  Luau, and they catch bugs in the translation just as well.
- **A byte-exact bytecode differential.** Real Luau programs are compiled by *C++* Luau and
  the resulting bytecode is executed on the *Rust* VM; outputs must match. This caught
  bugs the unit tests didn't — see below.

## War stories (the bugs that make the point)

A faithful port is where subtle semantic mismatches hide. A few representative ones, all
caught by the oracles or by instrumentation:

- **NP-hard subtyping false-positive.** A graph-coloring subtype query (`Triangle <:
  Uncolorable`, an intersection of curried functions over a union) spuriously tripped
  `LuauTypeInferIterationLimit`. Two strong models, asked to analyze it, stalled or
  concluded "leave it." Instrumenting *our* code found the real bug: the unifier's
  iteration counter wasn't reset per top-level unification (C++ resets it in the public
  `Unifier::tryUnify`; our wrappers called the recursive form directly). Fixed at the one
  boundary that covers every path — plus a **reflexive structural-equality fast-path** that
  is *better* than C++'s pointer-identity check, because our types aren't interned. The
  lesson: rigorous measurement beats reasoning about what the code "should" do.
- **traverse-vs-visit.** A recurring mis-port class: calling a single-node `visit(ty)` where
  the C++ recursed via `traverse(ty)`. Silent, because it type-checks and "mostly" works —
  caught across several type-visitors (`FreeTypeSearcher`, `SkipCacheForType`, …).
- **`!` vs `!`.** The bytecode differential found 6 runtime bugs in a VM that already
  "passed." The dominant class: C's logical `!x` mis-ported as Rust's bitwise `!x != 0`
  (which is *always* true). Invisible to a reviewer; obvious to a diff against the reference.
- **Bitfield packing.** Luau's `TKey` packs `unsigned tt:4; int next:28` into one word.
  Translated naively, `LuaNode` grew past its 32-byte budget and hash-table layout drifted;
  fixed with an explicit packed-accessor type.
- **"100%" that wasn't.** Early "fully translated" claims hid gutted methods (`// Logic
  elided`), `mod.rs`-exclusion fake-greens, and `todo!()` stubs. The gates and periodic
  audits exist precisely because "it compiles and the count says 100%" is not the same as
  "it's correct."
- **isocline → rustyline.** Luau vendors a ~9.8k-line C line-editor (`isocline`) for its
  REPL. The faithful analog of "vendor a third-party C dependency" isn't "hand-port 9.8k
  lines of raw-terminal C" — it's "depend on the idiomatic Rust equivalent." The REPL uses
  `rustyline`; semantics preserved, dependency-level fidelity kept.

## Honest caveats

- **`unsafe`.** Luau is pointer-soup by design (arena allocators, `TypeId = Type*`,
  intrusive GC). The faithful translation makes that explicit: `Type*` became `*mut Type`,
  derefs are wrapped in `unsafe`. About a third of files contain `unsafe`, concentrated in
  the type checker. Crucially it's *systematic* — a handful of pointer-representation
  patterns replicated many times — not thousands of independent hazards. Collapsing those
  patterns into safe arena-index abstractions is the obvious next project; it's deliberately
  *not* done pre-release, because it's a large rewrite with regression risk and no
  user-visible payoff until adoption says which layer matters.
- **New type solver gated; CodeGen JIT scoped.** See [`CONFORMANCE.md`](CONFORMANCE.md).

## Tools

The translation **output** (this repo) is public. The translation **pipeline** — the TSTG
extractor, scheduler, gate, and repair harness — is kept private. What it does is described
above; off-the-shelf pieces used along the way include
[`oneshot`](https://crates.io/) (a multi-model one-shot patch driver) and
[`zenpatch`](https://crates.io/crates/zenpatch) (a pure patch applier), plus Claude Code
and the Codex CLI for the agentic layer.
