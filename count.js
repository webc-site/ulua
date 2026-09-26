#!/usr/bin/env -S bun

import { sep } from "node:path";
import Table from "cli-table3";

const COLOR = {
  reset: "\x1b[0m",
  bold: "\x1b[1m",
  dim: "\x1b[2m",
  yellow: "\x1b[33m",
  cyan: "\x1b[36m",
  green: "\x1b[32m",
  red: "\x1b[31m",
};

const numFormat = (n) => String(n);

const crateGroup = (crate_name) => {
  if (
    crate_name.includes("test") ||
    crate_name.includes("conformance") ||
    crate_name.includes("e2e")
  ) {
    return "测试";
  }
  if (crate_name.endsWith("-cli") || crate_name === "ulua-cli-lib") {
    return "CLI";
  }
  if (["ulua-analysis", "ulua-capi", "ulua-web"].includes(crate_name)) {
    return "分析与绑定";
  }
  return "核心";
};

const argsParse = () => {
  const arg_li = process.argv.slice(2),
    opts = {
      json: false,
      top: 0,
      crate: null,
      concurrency: 64,
      help: false,
    };

  for (let i = 0; i < arg_li.length; ++i) {
    const arg = arg_li[i];
    if (arg === "--json") {
      opts.json = true;
    } else if (arg === "--top" || arg === "-t") {
      const next = arg_li[i + 1];
      if (next && !next.startsWith("-") && !isNaN(parseInt(next, 10))) {
        opts.top = parseInt(next, 10);
        ++i;
      } else {
        opts.top = 10;
      }
    } else if (arg.startsWith("--top=")) {
      opts.top = parseInt(arg.split("=")[1], 10) || 10;
    } else if (arg === "--crate" || arg === "-c") {
      const next = arg_li[i + 1];
      if (next && !next.startsWith("-")) {
        opts.crate = next;
        ++i;
      }
    } else if (arg.startsWith("--crate=")) {
      opts.crate = arg.split("=")[1];
    } else if (arg === "--concurrency" || arg === "-j") {
      opts.concurrency = parseInt(arg_li[++i], 10) || 64;
    } else if (arg.startsWith("--concurrency=")) {
      opts.concurrency = parseInt(arg.split("=")[1], 10) || 64;
    } else if (arg === "--help" || arg === "-h") {
      opts.help = true;
    }
  }

  return opts;
};

const helpPrint = () => {
  console.log(`
ulua 代码与 unsafe 统计工具

用法:
  ./count.js [选项]

选项:
  -t, --top [N]          列出 unsafe 最密集的前 N 个文件（默认 10）
  -c, --crate <NAME>     仅查看指定包的详细统计
  -j, --concurrency <N>  设置并发读取数（默认 64）
  --json                 以 JSON 格式输出结果
  -h, --help             显示帮助信息

示例:
  ./count.js                  # 默认视图：分模块无边框汇总表格
  ./count.js --top 15         # 找出 unsafe 最多的 15 个文件
  ./count.js -c ulua-analysis # 专门分析 ulua-analysis
`);
};

const borderlessTableMake = (head, colAligns) =>
  new Table({
    head,
    colAligns,
    chars: {
      top: "",
      "top-mid": "",
      "top-left": "",
      "top-right": "",
      bottom: "",
      "bottom-mid": "",
      "bottom-left": "",
      "bottom-right": "",
      left: "",
      "left-mid": "",
      mid: "",
      "mid-mid": "",
      right: "",
      "right-mid": "",
      middle: "  ",
    },
    style: { "padding-left": 0, "padding-right": 0, head: ["cyan", "bold"] },
  });

const rsFileScan = async (dir = "crates") => {
  const glob = new Bun.Glob(`${dir}/**/*.rs`);
  return Array.from(glob.scanSync("."));
};

const UNSAFE_REGEX = /\bunsafe\b/g;

const lineCommentStrip = (line) => {
  const idx = line.indexOf("//");
  return idx !== -1 ? line.substring(0, idx) : line;
};

const fileAnalyze = async (file_path) => {
  const text = await Bun.file(file_path).text();

  let prod_code = 0,
    test_code = 0,
    prod_unsafe = 0,
    test_unsafe = 0,
    in_block_comment = false,
    in_cfg_test = false,
    cfg_test_braces = 0,
    pending_cfg_test = false,
    start = 0;

  const part_li = file_path.split(sep),
    crate_name = part_li[1] || "",
    is_src = part_li[2] === "src",
    len = text.length;

  while (start < len) {
    let end = text.indexOf("\n", start);
    if (end === -1) end = len;

    const line = text.substring(start, end);
    start = end + 1;

    const trimmed = line.trim();
    if (trimmed.length === 0) continue;

    // 块注释处理
    if (in_block_comment) {
      if (trimmed.includes("*/")) in_block_comment = false;
      continue;
    }
    if (trimmed.startsWith("/*")) {
      if (!trimmed.includes("*/")) in_block_comment = true;
      continue;
    }
    // 行注释处理（包含 //, ///, //!）
    if (trimmed.startsWith("//")) continue;

    // 检查 #[cfg(test)] 或 #![cfg(test)]
    if (is_src) {
      if (trimmed.startsWith("#[cfg(test)]") || trimmed.startsWith("#![cfg(test)]")) {
        pending_cfg_test = true;
        const open = (trimmed.match(/{/g) || []).length,
          close = (trimmed.match(/}/g) || []).length;
        if (open > 0) {
          in_cfg_test = true;
          pending_cfg_test = false;
          cfg_test_braces += open - close;
        }
        continue;
      }

      if (pending_cfg_test) {
        const open = (trimmed.match(/{/g) || []).length,
          close = (trimmed.match(/}/g) || []).length;
        if (open > 0) {
          in_cfg_test = true;
          pending_cfg_test = false;
          cfg_test_braces += open - close;
        } else if (trimmed.endsWith(";")) {
          pending_cfg_test = false;
          ++test_code;
          continue;
        }
      } else if (in_cfg_test) {
        const open = (trimmed.match(/{/g) || []).length,
          close = (trimmed.match(/}/g) || []).length;
        cfg_test_braces += open - close;
        if (cfg_test_braces <= 0) {
          in_cfg_test = false;
        }
        ++test_code;
        const code_part = lineCommentStrip(trimmed);
        const matches = code_part.match(UNSAFE_REGEX);
        if (matches) test_unsafe += matches.length;
        continue;
      }
    }

    // 正常代码与 unsafe 统计
    const code_part = lineCommentStrip(trimmed);
    const matches = code_part.match(UNSAFE_REGEX);
    const unsafe_cnt = matches ? matches.length : 0;

    if (is_src) {
      ++prod_code;
      prod_unsafe += unsafe_cnt;
    } else {
      ++test_code;
      test_unsafe += unsafe_cnt;
    }
  }

  return {
    path: file_path,
    crate: crate_name,
    is_src,
    code: prod_code,
    test_code,
    unsafe_count: prod_unsafe + test_unsafe,
    prod_unsafe,
    test_unsafe,
  };
};

const fileProcessAll = async (file_path_li, concurrency = 64) => {
  const result_li = new Array(file_path_li.length);
  let current_index = 0;

  const worker_count = Math.min(concurrency, file_path_li.length),
    worker_li = Array.from({ length: worker_count }, async () => {
      while (current_index < file_path_li.length) {
        const idx = current_index++;
        result_li[idx] = await fileAnalyze(file_path_li[idx]);
      }
    });

  await Promise.all(worker_li);
  return result_li;
};

const cratePrint = (crate) => {
  console.log("\n" + COLOR.bold + crate.name + COLOR.reset);
  console.log("代码: " + numFormat(crate.code) + " 行");
  console.log("测试: " + numFormat(crate.test_code) + " 行");
  console.log("unsafe: " + numFormat(crate.unsafe_count) + " 处\n");

  const top_file_li = crate.file_li
    .filter((file_item) => file_item.unsafe_count > 0)
    .sort((a, b) => b.unsafe_count - a.unsafe_count)
    .slice(0, 15);

  if (top_file_li.length > 0) {
    console.log("Top " + top_file_li.length + " unsafe 文件:");
    const table = borderlessTableMake(
      ["#", "文件", "代码", "测试", "unsafe"],
      ["right", "left", "right", "right", "right"],
    );

    top_file_li.forEach((file_item, i) => {
      const rel_path = file_item.path.split(sep).slice(2).join("/");
      table.push([
        String(i + 1),
        rel_path,
        numFormat(file_item.code),
        numFormat(file_item.test_code),
        file_item.unsafe_count > 0
          ? COLOR.yellow + numFormat(file_item.unsafe_count) + COLOR.reset
          : "0",
      ]);
    });

    console.log(table.toString());
  } else {
    console.log("零 unsafe");
  }
};

const main = async () => {
  const opts = argsParse();
  if (opts.help) {
    helpPrint();
    return;
  }

  const file_path_li = await rsFileScan("crates"),
    file_res_li = await fileProcessAll(file_path_li, opts.concurrency);

  const crate_map = new Map();
  let total_code = 0,
    total_test = 0,
    total_unsafe = 0;

  for (const item of file_res_li) {
    let crate_data = crate_map.get(item.crate);
    if (!crate_data) {
      crate_data = {
        name: item.crate,
        group: crateGroup(item.crate),
        code: 0,
        test_code: 0,
        unsafe_count: 0,
        prod_unsafe: 0,
        test_unsafe: 0,
        file_li: [],
      };
      crate_map.set(item.crate, crate_data);
    }

    crate_data.file_li.push(item);
    crate_data.code += item.code;
    crate_data.test_code += item.test_code;
    crate_data.unsafe_count += item.unsafe_count;
    crate_data.prod_unsafe += item.prod_unsafe;
    crate_data.test_unsafe += item.test_unsafe;

    total_code += item.code;
    total_test += item.test_code;
    total_unsafe += item.unsafe_count;
  }

  if (opts.json) {
    const json_output = {
      summary: {
        code: total_code,
        test: total_test,
        unsafe: total_unsafe,
      },
      crates: Array.from(crate_map.values()).map((crate) => ({
        name: crate.name,
        group: crate.group,
        code: crate.code,
        test: crate.test_code,
        unsafe: crate.unsafe_count,
      })),
    };
    console.log(JSON.stringify(json_output, null, 2));
    return;
  }

  if (opts.crate) {
    const crate = crate_map.get(opts.crate);
    if (!crate) {
      console.error(COLOR.red + '错误: 未找到包 "' + opts.crate + '"' + COLOR.reset);
      return;
    }
    cratePrint(crate);
    return;
  }

  const group_li = [
    { key: "核心", title: "包 (核心)" },
    { key: "分析与绑定", title: "包 (分析与绑定)" },
    { key: "CLI", title: "包 (CLI)" },
    { key: "测试", title: "包 (测试)" },
  ];

  for (const g of group_li) {
    const group_crate_li = Array.from(crate_map.values())
      .filter((crate) => crate.group === g.key)
      .sort((a, b) => {
        const diff = b.unsafe_count - a.unsafe_count;
        return diff !== 0 ? diff : b.code + b.test_code - (a.code + a.test_code);
      });

    const table = borderlessTableMake(
      [g.title, "代码", "测试", "unsafe"],
      ["left", "right", "right", "right"],
    );

    let g_code = 0,
      g_test = 0,
      g_unsafe = 0;

    for (const item of group_crate_li) {
      g_code += item.code;
      g_test += item.test_code;
      g_unsafe += item.unsafe_count;

      const unsafe_str =
        item.unsafe_count > 0
          ? COLOR.yellow + numFormat(item.unsafe_count) + COLOR.reset
          : COLOR.green + "0" + COLOR.reset;

      table.push([item.name, numFormat(item.code), numFormat(item.test_code), unsafe_str]);
    }

    table.push([
      COLOR.bold + "汇总" + COLOR.reset,
      COLOR.bold + numFormat(g_code) + COLOR.reset,
      COLOR.bold + numFormat(g_test) + COLOR.reset,
      COLOR.bold + COLOR.yellow + numFormat(g_unsafe) + COLOR.reset,
    ]);

    console.log(table.toString());
    console.log("");
  }

  const total_table = borderlessTableMake(
    ["模块 (总计)", "代码", "测试", "unsafe"],
    ["left", "right", "right", "right"],
  );

  for (const g of group_li) {
    const group_crate_li = Array.from(crate_map.values()).filter((crate) => crate.group === g.key),
      code_sum = group_crate_li.reduce((acc, item) => acc + item.code, 0),
      test_sum = group_crate_li.reduce((acc, item) => acc + item.test_code, 0),
      unsafe_sum = group_crate_li.reduce((acc, item) => acc + item.unsafe_count, 0);

    total_table.push([g.key, numFormat(code_sum), numFormat(test_sum), numFormat(unsafe_sum)]);
  }

  total_table.push([
    COLOR.bold + "总计" + COLOR.reset,
    COLOR.bold + numFormat(total_code) + COLOR.reset,
    COLOR.bold + numFormat(total_test) + COLOR.reset,
    COLOR.bold + COLOR.yellow + numFormat(total_unsafe) + COLOR.reset,
  ]);

  console.log(total_table.toString());

  if (opts.top > 0) {
    const top_file_li = file_res_li
      .filter((file_item) => file_item.unsafe_count > 0)
      .sort((a, b) => b.unsafe_count - a.unsafe_count)
      .slice(0, opts.top);

    console.log("\nTop " + opts.top + " unsafe 文件:");
    const top_table = borderlessTableMake(
      ["#", "文件", "代码", "测试", "unsafe"],
      ["right", "left", "right", "right", "right"],
    );

    top_file_li.forEach((file_item, i) => {
      top_table.push([
        String(i + 1),
        file_item.path,
        numFormat(file_item.code),
        numFormat(file_item.test_code),
        COLOR.yellow + numFormat(file_item.unsafe_count) + COLOR.reset,
      ]);
    });

    console.log(top_table.toString() + "\n");
  }
};

try {
  await main();
} catch (err) {
  console.error(COLOR.red + "执行失败:" + COLOR.reset, err);
  process.exit(1);
}
