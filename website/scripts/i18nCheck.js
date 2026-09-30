#!/usr/bin/env -S bun

import { readdir } from "node:fs/promises";
import { join, resolve } from "node:path";
import CODE_LI from "../src/lib/locales/CODE.js";
import { CRATE_LI } from "../src/lib/const.js";
import { EXAMPLE_KEY_LI } from "../src/lib/examples.js";
import { BENCHMARK_LI, ENGINE_LI } from "../src/lib/benchData.js";

const ROOT_DIR = resolve(import.meta.dirname, ".."),
  SRC_DIR = resolve(ROOT_DIR, "src"),
  LOCALES_DIR = resolve(SRC_DIR, "lib/locales"),
  // 合法业务命名空间集合
  NAMESPACE_SET = new Set([
    "meta",
    "nav",
    "bench",
    "hero",
    "pg",
    "about",
    "features",
    "syntax",
    "status",
    "out",
    "example",
    "crates",
    "embed",
    "checker",
    "foot",
  ]),
  // 递归检索源码目录下的 .svelte 与 .js 文件
  srcFileLiGet = async (dir) => {
    const entry_li = await readdir(dir, { withFileTypes: true }),
      result_li = [];

    for (let i = 0; i < entry_li.length; ++i) {
      const entry = entry_li[i],
        full_path = join(dir, entry.name);

      if (entry.isDirectory()) {
        if (full_path.includes("lib/locales") || full_path.includes("svg")) {
          continue;
        }
        const sub_li = await srcFileLiGet(full_path);
        for (let j = 0; j < sub_li.length; ++j) {
          result_li.push(sub_li[j]);
        }
      } else if (entry.isFile() && /\.(svelte|js)$/.test(entry.name)) {
        result_li.push(full_path);
      }
    }
    return result_li;
  },
  // 提取动态模式生成的真实 key 集合
  dynamicKeySetGet = async () => {
    const key_set = new Set();

    // 1. Crates 模块架构: crates.${name}
    for (let i = 0; i < CRATE_LI.length; ++i) {
      key_set.add("crates." + CRATE_LI[i]);
    }

    // 2. Playground 示例: example.${key}
    for (let i = 0; i < EXAMPLE_KEY_LI.length; ++i) {
      key_set.add("example." + EXAMPLE_KEY_LI[i]);
    }

    // 3. 性能基准明细与对比: bench.item.*, bench.param.*, bench.lang.*
    for (let i = 0; i < BENCHMARK_LI.length; ++i) {
      const item = BENCHMARK_LI[i];
      key_set.add("bench.item." + item.id);
      key_set.add("bench.item." + item.id + "_desc");
      key_set.add("bench.param." + item.id);
    }
    for (let i = 0; i < ENGINE_LI.length; ++i) {
      key_set.add("bench.lang." + ENGINE_LI[i].id.replace(/_jit$|_interp$/, ""));
    }

    // 4. AboutLuau 动态支柱卡片: about.p{1,2,3}_title, about.p{1,2,3}_desc
    const pillar_id_li = ["p1", "p2", "p3"];
    for (let i = 0; i < pillar_id_li.length; ++i) {
      key_set.add("about." + pillar_id_li[i] + "_title");
      key_set.add("about." + pillar_id_li[i] + "_desc");
    }

    // 5. Features 动态特性卡片: features.f{1,2,3}_title, features.f{1,2,3}_desc
    const feature_id_li = ["f1", "f2", "f3"];
    for (let i = 0; i < feature_id_li.length; ++i) {
      key_set.add("features." + feature_id_li[i] + "_title");
      key_set.add("features." + feature_id_li[i] + "_desc");
    }

    // 6. Syntax 语法分组与 45 张语法卡片
    const syntax_file = Bun.file(resolve(SRC_DIR, "components/Syntax.svelte")),
      syntax_text = await syntax_file.text(),
      group_regex = /\[\s*"([a-zA-Z0-9_]+)",\s*\[/g,
      card_regex = /\[\s*"([a-zA-Z0-9_]+)",\s*`/g;

    let m = null;
    while ((m = group_regex.exec(syntax_text)) !== null) {
      if (m[1].startsWith("lua_") || m[1].startsWith("luau_")) {
        key_set.add("syntax." + m[1]);
      }
    }
    while ((m = card_regex.exec(syntax_text)) !== null) {
      key_set.add("syntax." + m[1] + "_title");
      key_set.add("syntax." + m[1] + "_desc");
    }

    // 7. Playground 状态
    const status_li = [
      "status.ready",
      "status.running",
      "status.checking",
      "status.loading_wasm",
      "status.wasm_failed",
      "status.runtime_error",
      "status.error",
      "status.no_errors",
      "status.ran_ok",
      "status.errors",
    ];
    for (let i = 0; i < status_li.length; ++i) {
      key_set.add(status_li[i]);
    }

    return key_set;
  },
  // 扫描源文件中直接出现的字面量 key 引用
  codeKeySetExtract = async (file_li) => {
    const code_key_set = await dynamicKeySetGet(),
      // 匹配形如 "nav.playground", 'bench.title', `pg.run` 的键字面量，且末尾不带点（避免捕获拼接前缀如 "bench.item."）
      literal_regex = /(?:'|"|`)\b([a-z0-9_]+(?:\.[a-z0-9_]+)+)\b(?!\.)(?:'|"|`)/g;

    for (let i = 0; i < file_li.length; ++i) {
      const file_path = file_li[i],
        content = await Bun.file(file_path).text();

      let m = null;
      while ((m = literal_regex.exec(content)) !== null) {
        const candidate = m[1],
          ns = candidate.split(".")[0];
        if (NAMESPACE_SET.has(ns)) {
          code_key_set.add(candidate);
        }
      }
    }

    return code_key_set;
  },
  // 主执行函数
  main = async () => {
    console.log("ulua i18n 资源多余与遗漏扫描检查\n");

    const locale_map = {},
      all_keys_set = new Set();

    // 1. 加载并索引全部语言包
    for (let i = 0; i < CODE_LI.length; ++i) {
      const code = CODE_LI[i],
        mod = await import(resolve(LOCALES_DIR, code + ".js")),
        dict = mod.default ?? mod;

      locale_map[code] = dict;
      const key_li = Object.keys(dict);
      for (let j = 0; j < key_li.length; ++j) {
        all_keys_set.add(key_li[j]);
      }
    }

    const base_lang = "en",
      base_keys_set = new Set(Object.keys(locale_map[base_lang] ?? {}));

    console.log(
      `✓ 语言包已就绪: ${CODE_LI.length} 种语言, 基准 (${base_lang}) 词条数: ${base_keys_set.size}, 全集词条数: ${all_keys_set.size}`,
    );

    let parity_error_count = 0;

    // 2. 检查多语言对称性（遗漏、多余、空值）
    console.log("\n[第一步] 校验 20 种语言包对齐度 (Locale Parity Check)...");
    for (let i = 0; i < CODE_LI.length; ++i) {
      const code = CODE_LI[i],
        dict = locale_map[code],
        dict_keys = Object.keys(dict),
        missing_li = [],
        extra_li = [],
        empty_li = [];

      for (const base_key of base_keys_set) {
        if (!(base_key in dict)) {
          missing_li.push(base_key);
        } else if (typeof dict[base_key] === "string" && dict[base_key].trim() === "") {
          empty_li.push(base_key);
        }
      }

      for (let j = 0; j < dict_keys.length; ++j) {
        const k = dict_keys[j];
        if (!base_keys_set.has(k)) {
          extra_li.push(k);
        }
      }

      if (missing_li.length > 0 || extra_li.length > 0 || empty_li.length > 0) {
        ++parity_error_count;
        console.error(`  ✗ [${code}] 发现异常:`);
        if (missing_li.length > 0) {
          console.error(
            `    - 遗漏词条 (${missing_li.length}个):`,
            missing_li.slice(0, 5).join(", ") + (missing_li.length > 5 ? "..." : ""),
          );
        }
        if (extra_li.length > 0) {
          console.error(
            `    - 多余词条 (${extra_li.length}个):`,
            extra_li.slice(0, 5).join(", ") + (extra_li.length > 5 ? "..." : ""),
          );
        }
        if (empty_li.length > 0) {
          console.error(
            `    - 空值词条 (${empty_li.length}个):`,
            empty_li.slice(0, 5).join(", ") + (empty_li.length > 5 ? "..." : ""),
          );
        }
      } else {
        console.log(`  ✓ [${code}] 词条完整且对称 (${dict_keys.length} 词条)`);
      }
    }

    // 3. 检查源码调用与字典定义对齐度
    console.log("\n[第二步] 扫描前端工程源码引用 (Code vs Locale Check)...");
    const file_li = await srcFileLiGet(SRC_DIR),
      code_key_set = await codeKeySetExtract(file_li);

    console.log(`✓ 扫描源文件: ${file_li.length} 个, 提取到代码引用词条: ${code_key_set.size} 个`);

    const missing_in_locale_li = [],
      unused_in_code_li = [];

    for (const code_key of code_key_set) {
      if (!all_keys_set.has(code_key)) {
        missing_in_locale_li.push(code_key);
      }
    }

    for (const locale_key of all_keys_set) {
      if (!code_key_set.has(locale_key)) {
        unused_in_code_li.push(locale_key);
      }
    }

    let code_error_count = 0;
    if (missing_in_locale_li.length > 0) {
      ++code_error_count;
      console.error(
        `\n  ✗ 源码中引用但语言字典中【遗漏】未定义的词条 (${missing_in_locale_li.length}个):`,
      );
      for (let i = 0; i < missing_in_locale_li.length; ++i) {
        console.error(`    - ${missing_in_locale_li[i]}`);
      }
    } else {
      console.log("  ✓ 源码中调用的所有词条均已在字典中正确定义 (0 遗漏)");
    }

    if (unused_in_code_li.length > 0) {
      ++code_error_count;
      console.error(
        `\n  ✗ 语言字典中定义但源码中【完全未使用/多余】的废弃词条 (${unused_in_code_li.length}个):`,
      );
      for (let i = 0; i < unused_in_code_li.length; ++i) {
        console.error(`    - ${unused_in_code_li[i]}`);
      }
    } else {
      console.log("  ✓ 语言字典中无任何冗余无用的废弃词条 (0 多余)");
    }

    // 4. 汇总判定
    if (parity_error_count === 0 && code_error_count === 0) {
      console.log("\n全部检查通过：20 种语言包完全对称，零多余，零遗漏\n");
      process.exit(0);
    } else {
      console.error(
        `\n检查失败：发现 ${parity_error_count} 处多语言对齐异常, ${code_error_count} 处源码引用偏差\n`,
      );
      process.exit(1);
    }
  };

await main();
