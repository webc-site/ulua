# Publishing to crates.io

ulua is published as independent crates so consumers can depend on exactly the layer they
need (`ulua-vm` alone, `ulua-analysis` alone, etc.). crates.io requires that every
dependency already exist on the registry at publish time, so crates **must be published in
dependency order** (a topological sort of the workspace graph).

## Prerequisites

- Each crate's path dependencies carry an explicit `version` (crates.io ignores `path` but
  requires `version`). This is set via `[workspace.package]` + `version.workspace = true`.
- Test/harness crates are marked `publish = false` and are **not** released:
  `ulua-unit-test`, `ulua-cli-test`, `ulua-conformance`.
- The `ulua-*` names are free on crates.io and must be owned by the publishing account.
- `cargo publish` runs a verification build per crate; budget time for ~21 builds.

## Publish order

Publish top-to-bottom; each layer only depends on layers above it.

```
# Layer 0 — foundations
1.  ulua-common

# Layer 1 — depend only on common
2.  ulua-ast
3.  ulua-bytecode
4.  ulua-vm

# Layer 2
5.  ulua-compiler      (ast, bytecode, common)
6.  ulua-code-gen      (common, vm)

# Layer 3
7.  ulua-config        (ast, bytecode, compiler, vm, common)

# Layer 4
8.  ulua-analysis      (ast, bytecode, compiler, config, vm, common)
9.  ulua-require       (+ config)
10. ulua-cli-lib       (+ config)
11. ulua-rt-derive     (proc-macro derives for ulua-rt)
12. ulua-rt            (vm, compiler, common — the mlua-style API; also ulua-config + ulua-analysis, optional, only under the `typecheck` feature — both already published above at 7 and 8)

# Layer 5 — umbrella + leaves (wasm + CLIs)
13. ulua-checked-macros (compile-time checked Luau source macros; depends on ulua-rt)
14. ulua               (umbrella: re-exports every lib + ulua-rt + checked macros — publish after them all)
15. ulua-web           (analysis, ...)
16. ulua-ast-cli
17. ulua-analyze-cli
18. ulua-bytecode-cli
19. ulua-compile-cli
20. ulua-reduce-cli
21. ulua-repl-cli
```

## Recommended dry run

```sh
# Verify every publishable crate packages cleanly, in order, without uploading:
for c in ulua-common ulua-ast ulua-bytecode ulua-vm ulua-compiler ulua-code-gen \
         ulua-config ulua-analysis ulua-require ulua-cli-lib ulua-rt-derive ulua-rt \
         ulua-checked-macros ulua ulua-web ulua-ast-cli ulua-analyze-cli \
         ulua-bytecode-cli ulua-compile-cli ulua-reduce-cli ulua-repl-cli; do
  cargo publish -p "$c" --dry-run || { echo "FAILED: $c"; break; }
done
```

Then drop `--dry-run` and run the same loop to release. If a later crate fails after
earlier ones uploaded, fix and resume from the failed crate (already-published versions are
immutable — bump the version if a re-publish is needed).
