#!/usr/bin/env bash
# Harvest as many REAL Luau / Lua scripts as possible into a corpus directory, so
# they can be compiled + run through ulua (bug-hunting: real scripts exercise
# feature combinations no generator invents — this is how the `run` fuzzer found
# the multi-value-return stack overrun).
#
#   scripts/harvest-scripts.sh                  # clone curated repos -> corpus
#   OUT=/tmp/luau-corpus scripts/harvest-scripts.sh
#   scripts/harvest-scripts.sh --search         # ALSO GitHub code-search for more
#
# Output: $OUT/*.luau, deduped by content hash. Source provenance in $OUT/SOURCES.tsv.
# Skips files > 512 KiB (usually generated/minified) and empty files.
set -uo pipefail

OUT="${OUT:-$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)/fuzz/corpus-harvest}"
DO_SEARCH=0
[[ "${1:-}" == "--search" ]] && DO_SEARCH=1
mkdir -p "$OUT"
: > "$OUT/SOURCES.tsv"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

# High-yield Luau (Roblox ecosystem) + Lua repos. Luau is based on Lua 5.1, so
# both parse; self-contained ones also run. Chosen for many .luau/.lua files
# without being gigantic. owner/repo[:sparse,paths].
REPOS=(
  # ---- Luau: the language + its own tests ----
  "luau-lang/luau:tests,bench,Analysis,Compiler,VM"
  "luau-lang/rfcs"
  "lune-org/lune:tests,crates"
  # ---- Luau: BIG real codebases (goldmines: hundreds of modules) ----
  "Quenty/NevermoreEngine"
  "jsdotlua/react-lua"
  "jsdotlua/luau-polyfill"
  "jsdotlua/collections"
  "jsdotlua/jest-roblox"
  "jsdotlua/promise"
  "jsdotlua/graphql-lua"
  "jsdotlua/rxlua"
  "roblox/wiki"
  "Roblox/creator-docs"
  # ---- Luau: UI / ECS / state / util libraries ----
  "Roblox/roact"
  "Roblox/rodux"
  "Roblox/roact-rodux"
  "Roblox/testez"
  "Sleitnick/Knit"
  "Sleitnick/RbxUtil"
  "Sleitnick/Component"
  "evaera/roblox-lua-promise"
  "osyrisrblx/t"
  "osyrisrblx/rbx-resource"
  "matter-ecs/matter"
  "centau/vide"
  "centau/ecr"
  "red-blox/Signal"
  "red-blox/Net"
  "Ukendio/jecs"
  "seaofvoices/darklua"
  "littensy/charm"
  "littensy/ripple"
  "littensy/rbxts-pretty-react-hooks"
  "roblox/tarmac"
  "rojo-rbx/rojo"
  "rojo-rbx/rbx-dom"
  "UpliftGames/wally"
  # ---- Lua: reference + widely-used pure-Lua libraries (Lua 5.1-compatible) ----
  "lua/lua:testes"
  "LuaJIT/LuaJIT:test"
  "rxi/json.lua"
  "rxi/classic"
  "rxi/lume"
  "rxi/log.lua"
  "rxi/flux"
  "kikito/inspect.lua"
  "kikito/middleclass"
  "kikito/bump.lua"
  "kikito/anim8"
  "kikito/tween.lua"
  "kikito/cron.lua"
  "kikito/stateful.lua"
  "Yonaba/Moses"
  "Yonaba/30log"
  "bakpakin/Fennel"
  "bakpakin/binser"
  "luvit/luvit:deps"
  "hoelzro/lua-term"
  "APItools/router.lua"
  "vrld/hump"
  "vrld/HardonCollider"
  "EmmanuelOga/easing"
  "starwing/luaunit"
  "bluebird75/luaunit"
  "lunarmodules/luassert"
  "lunarmodules/say"
  "lunarmodules/Penlight:lua"
  "lunarmodules/luafilesystem:tests"
  "leafo/lapis:lapis,spec"
  "leafo/moonscript:moonscript,spec"
  "pkulchenko/serpent"
  "diegonehab/luasocket:src,test"
  "openresty/lua-resty-core:lib,t"
  "openresty/lua-resty-lrucache"
  "openresty/lua-resty-redis"
  "openresty/lua-cjson:tests"
  "Kong/kong:kong,spec"
  "torch/torch7:test"
  "nmap/nmap:scripts,nselib"
)

stage() { # $1 = file path, $2 = source label
  local f="$1" src="$2"
  [[ -s "$f" ]] || return 0
  local sz; sz=$(wc -c < "$f" 2>/dev/null || echo 0)
  (( sz > 524288 )) && return 0
  # valid UTF-8 only (ulua reads source as from_utf8; non-UTF-8 is not a program)
  iconv -f UTF-8 -t UTF-8 "$f" >/dev/null 2>&1 || return 0
  local h; h="$(shasum "$f" 2>/dev/null | cut -c1-16)" || return 0
  local dst="$OUT/s-$h.luau"
  if [[ ! -e "$dst" ]]; then
    cp "$f" "$dst" && printf '%s\t%s\n' "s-$h.luau" "$src" >> "$OUT/SOURCES.tsv"
  fi
}

harvest_dir() { # $1 = dir, $2 = source label
  local n=0
  while IFS= read -r f; do stage "$f" "$2"; n=$((n+1)); done < <(
    find "$1" \( -name '*.luau' -o -name '*.lua' \) -type f 2>/dev/null
  )
  echo "   staged from $2 (scanned $n)"
}

echo ">> vendored conformance suite"
harvest_dir "$(dirname "$OUT")/../crates/ulua-conformance" "vendored-conformance"

for spec in "${REPOS[@]}"; do
  repo="${spec%%:*}"; sparse="${spec#*:}"; [[ "$sparse" == "$repo" ]] && sparse=""
  echo ">> $repo"
  d="$TMP/${repo//\//_}"
  if [[ -n "$sparse" ]]; then
    git clone --depth 1 --filter=blob:none --sparse "https://github.com/$repo" "$d" >/dev/null 2>&1 \
      && ( cd "$d" && git sparse-checkout set ${sparse//,/ } >/dev/null 2>&1 )
  else
    git clone --depth 1 "https://github.com/$repo" "$d" >/dev/null 2>&1
  fi
  if [[ -d "$d" ]]; then harvest_dir "$d" "$repo"; else echo "   (clone failed / skipped)"; fi
  rm -rf "$d"
done

if [[ "$DO_SEARCH" == "1" ]]; then
  echo ">> GitHub code search for more .luau repos (rate-limited)"
  # Discover repos that contain Luau, then shallow-clone the top few.
  mapfile -t found < <(gh search code --extension luau --limit 60 --json repository \
    --jq '.[].repository.nameWithOwner' 2>/dev/null | sort -u | head -20)
  for repo in "${found[@]}"; do
    grep -q "$repo" <<< "${REPOS[*]}" && continue
    echo ">> (search) $repo"
    d="$TMP/${repo//\//_}"
    git clone --depth 1 "https://github.com/$repo" "$d" >/dev/null 2>&1 && harvest_dir "$d" "search:$repo"
    rm -rf "$d"
  done
fi

total=$(find "$OUT" -name '*.luau' -type f | wc -l | tr -d ' ')
echo "==> harvested $total unique scripts into $OUT"

# Wire the harvest into the AFL seed corpus (unless --no-wire). Only the
# DIRECT-SOURCE targets (compile/run/typeck) take a .luau file as raw bytes, so
# real scripts are meaningful seeds only there; run_afl.sh reads
# `fuzz/corpus/<target>/`. The GENERATOR targets drive a grammar from the bytes
# (a .luau file is just random driver bytes) and `splice` embeds its corpus at
# build time, so neither is seeded here. `fuzz/corpus/` is gitignored.
if [[ "${*:-}" != *--no-wire* ]]; then
  FUZZ_CORPUS="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)/fuzz/corpus"
  for t in compile run typeck; do
    mkdir -p "$FUZZ_CORPUS/$t"
    cp "$OUT"/s-*.luau "$FUZZ_CORPUS/$t/" 2>/dev/null || true
  done
  echo "==> wired into fuzz/corpus/{compile,run,typeck} (AFL seeds for make fuzz-run / fuzz-compile / fuzz-typeck)"
fi
