#!/usr/bin/env node
// ulua 基准 fixtures 确定性生成器。
//
// 生成编译吞吐组 (compile_cases/*.luau) 与类型检查组 (analysis_cases/*.luau)
// 的代表性大源文件：体量控制在 100–300 KB 量级，避免超大文件拖慢 CI 克隆。
// 全程使用带种子的 LCG 伪随机，输出逐字节确定，重复运行不产生 diff。
//
// 用法：node benchmarks/fixtures/generate.mjs
import { writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const OUT = join(dirname(fileURLToPath(import.meta.url)), "..");

// 种子化 LCG（数值分布无关紧要，只要求确定性）
function rng(seed) {
  let s = seed >>> 0;
  return () => {
    s = (s * 1664525 + 1013904223) >>> 0;
    return s / 0x100000000;
  };
}

function pick(rand, list) {
  return list[Math.floor(rand() * list.length)];
}

// —— 1. parse_dense.luau：混合表达式密度，压 Parser 语法吞吐 ——
function genParseDense(lines = 3800) {
  const rand = rng(20260901);
  const names = ["alpha", "beta", "gamma", "delta"];
  const out = [];
  out.push("-- 生成物：表达式密集型源文件（benchmarks/fixtures/generate.mjs，勿手改）");
  out.push("local acc = 0.0");
  out.push("local bag, cb, m, q, r = nil, nil, nil, nil, nil");
  out.push('local names = { "alpha", "beta", "gamma", "delta" }');
  out.push('local cfg = { depth = 3, scale = 1.5, tags = { "a", "b" } }');
  out.push("");
  out.push("local function blend(a: number, b: number, k: number): number");
  out.push("  return a * (1 - k) + b * k");
  out.push("end");
  out.push("");
  for (let i = 0; i < lines; i++) {
    const kind = Math.floor(rand() * 8);
    const x = (rand() * 100).toFixed(2);
    const y = Math.floor(rand() * 9999);
    const s = pick(rand, ["red", "green", "blue", "warm", "cold", "fast"]);
    const n = pick(rand, names);
    if (kind === 0) {
      out.push(`acc = acc + blend(${x}, ${y % 97}, math.sin(${x} / 7)) * (1 + ${y % 13} / 31)`);
    } else if (kind === 1) {
      out.push(
        `bag = { id = ${y}, name = "${s}-${n}", pos = { ${x}, ${y}.5, ${x * 2} }, ok = ${y % 2 === 0} }`,
      );
    } else if (kind === 2) {
      out.push(
        `if acc > ${x} then acc = acc / ${(y % 89) + 1} elseif acc < -${x} then acc = -acc else acc = acc + ${y} end`,
      );
    } else if (kind === 3) {
      out.push(
        `cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * ${x}) end`,
      );
    } else if (kind === 4) {
      out.push(
        `m = { ["k${y}"] = { nested = { deeper = { value = "${s}", list = { 1, 2, ${y} } } } } }`,
      );
    } else if (kind === 5) {
      out.push(`q = "${s} {math.floor(${x})} {string.rep('=', ${y % 7} + 1)} ${n} done"`);
    } else if (kind === 6) {
      out.push(`for j = 1, ${(y % 40) + 1} do acc = acc + (j * ${x}) % 97 / (j + 1) end`);
    } else {
      out.push(
        `r = (${x} > ${y % 50}) and (acc ~= 0 and "${s}" or "${n}") or blend(${x}, ${y}, 0.25)`,
      );
    }
  }
  out.push("");
  out.push(`print("parse_dense checksum", acc % 1000, cfg.depth)`);
  out.push("");
  return out.join("\n");
}

// —— 2. func_many.luau：海量独立函数 + 控制流，压 Compiler 逐函数codegen ——
function genFuncMany(funcs = 300, body = 14) {
  const rand = rng(20260902);
  const out = [];
  out.push("-- 生成物：多函数型源文件（benchmarks/fixtures/generate.mjs，勿手改）");
  const names = ["measure", "project", "fold", "spread", "clampv", "weight", "derive", "compose"];
  for (let i = 0; i < funcs; i++) {
    const name = `${pick(rand, names)}${i}`;
    out.push("do");
    out.push(`local function ${name}(a: number, b: number, seed: number): number`);
    out.push(`  local acc = a + b * seed`);
    out.push(`  local bag = { a, b, seed, acc }`);
    for (let j = 0; j < body; j++) {
      const kind = Math.floor(rand() * 5);
      const c = (rand() * 50).toFixed(2);
      if (kind === 0) {
        out.push(`  if acc % 7 > 3 then acc = acc * ${c} / 17 + bag[1] else acc = acc - ${c} end`);
      } else if (kind === 1) {
        out.push(`  for k = 1, 4 do acc = math.min(acc + k * ${c}, bag[k] * 2) end`);
      } else if (kind === 2) {
        out.push(`  local tmp = { x = acc, y = ${c}, label = "f${i}_l${j}" }`);
        out.push(`  acc = tmp.x + tmp.y * (seed % 5)`);
      } else if (kind === 3) {
        out.push(`  local cb = function(v) return v * acc + ${c} end`);
        out.push(`  acc = cb(bag[${(j % 4) + 1}])`);
      } else {
        out.push(`  while acc > ${c} * 40 do acc = acc / 2 end`);
      }
    }
    out.push(`  return acc % 9973`);
    out.push(`end`);
    out.push(`acc = acc + ${name}(1, 2, ${i})`);
    out.push("end");
    out.push("");
  }
  return out.join("\n");
}

// —— 3. types_annotations.luau：类型注解密集，压类型语法解析 + 注解编译 ——
function genTypesDense(alias = 700, fns = 160) {
  const rand = rng(20260903);
  const out = [];
  out.push("-- 生成物：类型注解密集型源文件（benchmarks/fixtures/generate.mjs，勿手改）");
  out.push("export type Id = string");
  out.push("export type Count = number");
  out.push("export type Payload = { kind: string, bytes: { number } }");
  out.push("export type Handler = (Payload) -> boolean");
  out.push("");
  for (let i = 0; i < alias; i++) {
    const kind = Math.floor(rand() * 4);
    if (kind === 0) {
      out.push(
        `type Shape${i} = { x: number, y: number, tag: "${pick(rand, ["a", "b", "c"])}", meta: { [string]: number } }`,
      );
    } else if (kind === 1) {
      out.push(
        `type Bag${i} = { items: { Shape${Math.max(0, i - 1)} }, cursor: number, done: boolean }`,
      );
    } else if (kind === 2) {
      out.push(
        `type Fn${i} = (a: number, b: string, opts: { scale: number?, label: string? }) -> (number, string)`,
      );
    } else {
      out.push(
        `type Alias${i} = Id | Count | { nested: Alias${Math.max(0, i - 2)}? } | { array: { Fn${Math.max(0, i - 3)} } }`,
      );
    }
  }
  out.push("");
  for (let i = 0; i < fns; i++) {
    out.push(`local function step${i}<T, R>(input: T, f: (T) -> R, bag: Bag${i % 700}): R`);
    out.push(`  local out: R = f(input)`);
    out.push(`  for _ = 1, bag.cursor do`);
    out.push(`    out = f(input)`);
    out.push(`  end`);
    out.push(`  return out`);
    out.push(`end`);
  }
  out.push("");
  return out.join("\n");
}

// —— 4. strict_lib.luau：严格模式重类型库，压类型检查器（分析组）——
function genStrictLib(rounds = 42) {
  const rand = rng(20260904);
  const out = [];
  out.push("--!strict");
  out.push("-- 生成物：重类型 Luau 库（benchmarks/fixtures/generate.mjs，勿手改）");
  out.push("export type Vec2 = { x: number, y: number }");
  out.push('export type Color = "red" | "green" | "blue" | "warm" | "cold"');
  out.push("export type Event<T> = { kind: string, at: number, payload: T }");
  out.push("export type Listener<T> = (T) -> ()");
  out.push("export type Filter<T, U> = (T, U) -> boolean");
  out.push("export type Store<S> = {");
  out.push("  get: () -> S,");
  out.push("  set: (S) -> (),");
  out.push("  subscribe: (Listener<S>) -> () -> (),");
  out.push("}");
  out.push("");
  out.push("export type Marker = { name: string, rank: number, tags: { string } }");
  out.push("");
  out.push("local function label(m: Marker): string");
  out.push('  return m.name .. "#" .. tostring(m.rank)');
  out.push("end");
  out.push("");
  for (let r = 0; r < rounds; r++) {
    out.push(`-- 区块 ${r}`);
    out.push(
      `export type Config${r} = {{ id: number, color: Color, pos: Vec2, flags: { [string]: boolean } } }`,
    );
    out.push(
      `export type Result${r} = { ok: boolean, value: number, events: { Event<Config${r}> } }`,
    );
    out.push(`local function build${r}(seed: number): Config${r}`);
    out.push(`  local rows: Config${r} = {}`);
    out.push(`  for i = 1, 16 do`);
    out.push(
      `    local color: Color = if i % 5 == 0 then "warm" elseif i % 3 == 0 then "cold" elseif i % 2 == 0 then "green" else "blue"`,
    );
    out.push(`    rows[i] = {`);
    out.push(`      id = seed * 97 + i,`);
    out.push(`      color = color,`);
    out.push(`      pos = { x = i * 1.5, y = i / 3 },`);
    out.push(`      flags = { active = i % 2 == 0, visible = i < 12 },`);
    out.push(`    }`);
    out.push(`  end`);
    out.push(`  return rows`);
    out.push(`end`);
    out.push(`local function reduce${r}<T, U>(src: { T }, f: (U, T) -> U, init: U): U`);
    out.push(`  local acc: U = init`);
    out.push(`  for _, v in src do`);
    out.push(`    acc = f(acc, v)`);
    out.push(`  end`);
    out.push(`  return acc`);
    out.push(`end`);
    out.push(`local function score${r}(cfg: Config${r}): Result${r}`);
    out.push(
      `  local total = reduce${r}(cfg, function(a: number, row: { id: number, color: Color, pos: Vec2, flags: { [string]: boolean } })`,
    );
    out.push(`    return a + row.id * row.pos.x + (if row.flags.active then 10 else 1)`);
    out.push(`  end, 0)`);
    out.push(`  local marker: Marker = { name = "m${r}", rank = total % 13, tags = {} }`);
    out.push(`  table.insert(marker.tags, label(marker))`);
    out.push(`  return {`);
    out.push(`    ok = total > ${Math.floor(rand() * 500)},`);
    out.push(`    value = total / 3,`);
    out.push(`    events = { { kind = label(marker), at = total, payload = cfg } },`);
    out.push(`  }`);
    out.push(`end`);
    out.push(`local store${r}: Store<Result${r}> = (function()`);
    out.push(`  local current: Result${r} = score${r}(build${r}(${r}))`);
    out.push(`  local listeners: { Listener<Result${r}> } = {}`);
    out.push(`  return {`);
    out.push(`    get = function() return current end,`);
    out.push(`    set = function(next)`);
    out.push(`      current = next`);
    out.push(`      for _, fn in listeners do`);
    out.push(`        fn(next)`);
    out.push(`      end`);
    out.push(`    end,`);
    out.push(`    subscribe = function(fn)`);
    out.push(`      table.insert(listeners, fn)`);
    out.push(`      return function()`);
    out.push(`        for i = #listeners, 1, -1 do`);
    out.push(`          if listeners[i] == fn then table.remove(listeners, i) end`);
    out.push(`        end`);
    out.push(`      end`);
    out.push(`    end,`);
    out.push(`  }`);
    out.push(`end)()`);
    out.push(
      `local filtered${r} = if store${r}.get().ok then store${r}.get().value else -store${r}.get().value`,
    );
    out.push(
      `local _lifted${r} = if typeof(filtered${r}) == "number" then filtered${r} * 2 else 0`,
    );
    out.push("");
  }
  return out.join("\n");
}

const targets = [
  ["compile_cases/parse_dense.luau", genParseDense()],
  ["compile_cases/func_many.luau", genFuncMany()],
  ["compile_cases/types_annotations.luau", genTypesDense()],
  ["analysis_cases/strict_lib.luau", genStrictLib()],
];

for (const [rel, text] of targets) {
  const path = join(OUT, rel);
  writeFileSync(path, text);
  console.log(`${rel}: ${(text.length / 1024).toFixed(0)} KB`);
}
