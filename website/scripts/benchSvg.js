#!/usr/bin/env -S bun

/**
 * 将 benchmarks/results.json 转换为独立高质量矢量 SVG 图表
 * 严格遵循 .agents/skills/js_review 规范
 * 输出由 benchSvgI18n.js 动态驱动，彻底消除硬编码
 */

import { dirname, join } from "node:path";
import { benchEnvGet } from "./benchEnv.js";
import { I18N_TEXT, svgFileNamesGet } from "./benchSvgI18n.js";

const HERE = import.meta.dirname,
  REPO_ROOT = dirname(dirname(HERE)),
  BENCHMARKS_DIR = join(REPO_ROOT, "benchmarks"),
  RESULTS_JSON = join(BENCHMARKS_DIR, "results.json"),
  xmlEsc = (str) => str.replaceAll("&", "&amp;").replaceAll("<", "&lt;").replaceAll(">", "&gt;"),
  geoMeanCalc = (num_li) => {
    if (num_li.length === 0) return 0;
    const log_sum = num_li.reduce((acc, val) => acc + Math.log(val), 0);
    return Math.exp(log_sum / num_li.length);
  },
  svgRowMarkup = (row, y, max_ms, bar_start_x, max_bar_w, i18n) => {
    const bar_w = Math.max(16, (row.geomean_ms / max_ms) * max_bar_w),
      is_ulua = row.is_ulua,
      is_jit = row.mode === "jit",
      grad_id = is_ulua
        ? is_jit
          ? "grad-ulua-jit"
          : "grad-ulua"
        : is_jit
          ? row.id.includes("luajit")
            ? "grad-luajit"
            : "grad-jit"
          : row.id.includes("luajit")
            ? "grad-luajit-interp"
            : row.id.includes("luau")
              ? "grad-luau"
              : "grad-lua54",
      clean_label = row.label.replace(/\s*\((?:JIT|解释|Interp)\)/gi, ""),
      clean_lang = i18n.engine_lang[row.id] || row.lang;

    return `
    <g class="row${is_ulua ? " row-ulua" : ""}">
      <text x="24" y="${y + 17}" class="label-name">${xmlEsc(clean_label)}${is_ulua ? ` <tspan class="badge-star" dx="6">${xmlEsc(i18n.star)}</tspan>` : ""}</text>
      <text x="24" y="${y + 33}" class="label-sub">${xmlEsc(clean_lang)}</text>
      <rect x="${bar_start_x}" y="${y + 7}" width="${max_bar_w}" height="24" rx="6" class="bar-track" />
      <rect x="${bar_start_x}" y="${y + 7}" width="${bar_w}" height="24" rx="6" fill="url(#${grad_id})" class="bar-fill" />
      <text x="${bar_start_x + bar_w + 14}" y="${y + 24}" class="val-text">
        <tspan class="val-ms">${row.geomean_ms}</tspan>
        <tspan class="val-unit"> ms</tspan>
      </text>
    </g>`;
  },
  svgUnifiedBuild = (jit_li, interp_li, env_info, i18n) => {
    const width = 840,
      bar_start_x = 220,
      max_bar_w = 430,
      row_h = 48,
      all_li = [...jit_li, ...interp_li],
      max_ms = Math.max(...all_li.map((r) => r.geomean_ms), 50),
      title_y = 38,
      subtitle_y = 60,
      zone1_header_y = 88,
      zone1_rows_start_y = 126,
      zone1_height = jit_li.length * row_h,
      zone2_header_y = zone1_rows_start_y + zone1_height + 34,
      zone2_rows_start_y = zone2_header_y + 38,
      zone2_height = interp_li.length * row_h,
      total_height = zone2_rows_start_y + zone2_height + 36;

    let jit_markup = "";
    jit_li.forEach((row, idx) => {
      jit_markup += svgRowMarkup(
        row,
        zone1_rows_start_y + idx * row_h,
        max_ms,
        bar_start_x,
        max_bar_w,
        i18n,
      );
    });

    let interp_markup = "";
    interp_li.forEach((row, idx) => {
      interp_markup += svgRowMarkup(
        row,
        zone2_rows_start_y + idx * row_h,
        max_ms,
        bar_start_x,
        max_bar_w,
        i18n,
      );
    });

    return `<!-- 由 website/scripts/benchSvg.js 自动生成，请勿手动编辑 -->
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 ${width} ${total_height}" width="${width}" height="${total_height}">
  <defs>
    <!-- ulua interp: 现代极光钴蓝向天青渐变 -->
    <linearGradient id="grad-ulua" x1="0%" y1="0%" x2="100%" y2="0%">
      <stop offset="0%" stop-color="#2563eb" />
      <stop offset="100%" stop-color="#06b6d4" />
    </linearGradient>
    <!-- ulua jit: 现代电光青蓝向天蓝渐变 -->
    <linearGradient id="grad-ulua-jit" x1="0%" y1="0%" x2="100%" y2="0%">
      <stop offset="0%" stop-color="#0284c7" />
      <stop offset="100%" stop-color="#38bdf8" />
    </linearGradient>
    <!-- mlua/luau interp: 翡翠绿 -->
    <linearGradient id="grad-luau" x1="0%" y1="0%" x2="100%" y2="0%">
      <stop offset="0%" stop-color="#059669" />
      <stop offset="100%" stop-color="#10b981" />
    </linearGradient>
    <!-- mlua/luau jit: 琥珀亮橙 -->
    <linearGradient id="grad-jit" x1="0%" y1="0%" x2="100%" y2="0%">
      <stop offset="0%" stop-color="#ea580c" />
      <stop offset="100%" stop-color="#f59e0b" />
    </linearGradient>
    <!-- LuaJIT JIT: 柔雾鸢尾蓝紫 -->
    <linearGradient id="grad-luajit" x1="0%" y1="0%" x2="100%" y2="0%">
      <stop offset="0%" stop-color="#7c3aed" />
      <stop offset="100%" stop-color="#a855f7" />
    </linearGradient>
    <!-- LuaJIT Interp: 柔和靛蓝 -->
    <linearGradient id="grad-luajit-interp" x1="0%" y1="0%" x2="100%" y2="0%">
      <stop offset="0%" stop-color="#6366f1" />
      <stop offset="100%" stop-color="#818cf8" />
    </linearGradient>
    <!-- Lua 5.4: 钛金板岩灰 -->
    <linearGradient id="grad-lua54" x1="0%" y1="0%" x2="100%" y2="0%">
      <stop offset="0%" stop-color="#64748b" />
      <stop offset="100%" stop-color="#94a3b8" />
    </linearGradient>
    <style>
      .bg { fill: #ffffff; }
      .title { font-family: ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif; font-size: 18px; font-weight: 750; fill: #0f172a; }
      .subtitle { font-family: ui-sans-serif, system-ui, -apple-system, sans-serif; font-size: 12px; fill: #64748b; }
      .zone-accent { fill: #0284c7; }
      .zone-title { font-family: ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif; font-size: 14px; font-weight: 750; fill: #0f172a; letter-spacing: -0.01em; }
      .bar-track { fill: #f8fafc; }
      .label-name { font-family: ui-sans-serif, system-ui, -apple-system, sans-serif; font-size: 13px; font-weight: 650; fill: #1e293b; }
      .label-sub { font-family: ui-sans-serif, system-ui, -apple-system, sans-serif; font-size: 10.5px; fill: #64748b; }
      .badge-mode { font-family: ui-sans-serif, system-ui, -apple-system, sans-serif; font-size: 9.5px; font-weight: 650; }
      .badge-star { font-family: ui-sans-serif, system-ui, -apple-system, sans-serif; font-size: 10.5px; font-weight: 700; fill: #2563eb; }
      .row-ulua .label-name { fill: #0284c7; font-weight: 750; }
      .val-text { font-family: ui-monospace, SFMono-Regular, "Roboto Mono", Menlo, monospace; }
      .val-ms { font-size: 13px; font-weight: 750; fill: #0f172a; }
      .row-ulua .val-ms { fill: #0284c7; }
      .val-unit { font-size: 11px; font-weight: 500; fill: #64748b; }
      .footer-note { font-family: ui-sans-serif, system-ui, -apple-system, sans-serif; font-size: 11px; fill: #94a3b8; }
      @media (prefers-color-scheme: dark) {
        .bg { fill: #0f172a; }
        .title { fill: #f8fafc; }
        .subtitle { fill: #94a3b8; }
        .zone-accent { fill: #38bdf8; }
        .zone-title { fill: #f8fafc; }
        .bar-track { fill: #1e293b; }
        .label-name { fill: #f1f5f9; }
        .label-sub { fill: #94a3b8; }
        .val-ms { fill: #f8fafc; }
        .footer-note { fill: #64748b; }
      }
    </style>
  </defs>

  <!-- 背景底板 -->
  <rect width="${width}" height="${total_height}" rx="14" class="bg" />

  <!-- 标题与副标题 (无顶部刻度数字) -->
  <text x="24" y="${title_y}" class="title">${xmlEsc(i18n.title)}</text>
  <text x="24" y="${subtitle_y}" class="subtitle">${xmlEsc(i18n.subtitle)}</text>

  <!-- 分区 1：即时编译区 -->
  <rect x="24" y="${zone1_header_y}" width="4" height="16" rx="2" class="zone-accent" />
  <text x="36" y="${zone1_header_y + 8}" dominant-baseline="central" class="zone-title">${xmlEsc(i18n.zone_jit)}</text>

  <!-- JIT 数据行 -->
  ${jit_markup}

  <!-- 分区 2：解释执行区 -->
  <rect x="24" y="${zone2_header_y}" width="4" height="16" rx="2" class="zone-accent" />
  <text x="36" y="${zone2_header_y + 8}" dominant-baseline="central" class="zone-title">${xmlEsc(i18n.zone_interp)}</text>

  <!-- 解释执行数据行 -->
  ${interp_markup}

  <!-- 底部说明 -->
  <text x="24" y="${total_height - 14}" class="footer-note">${xmlEsc(i18n.footer(i18n.env_summary(env_info)))}</text>
</svg>
`;
  },
  main = async () => {
    const file = Bun.file(RESULTS_JSON);
    if (!(await file.exists())) {
      console.error("未找到 " + RESULTS_JSON + "，请先运行 ./bench.sh");
      return;
    }

    const data = await file.json(),
      benchmarks = data.benchmarks ?? [],
      engines = data.engines ?? [],
      raw_data = data.data ?? {},
      raw_ref = data.reference ?? {},
      env_info = benchEnvGet(data.environment),
      merged_data = {};

    benchmarks.forEach((b) => {
      merged_data[b.id] = {
        ...(raw_ref[b.id] ?? {}),
        ...(raw_data[b.id] ?? {}),
      };
    });

    const engine_geomean_li = engines
        .map((eng) => {
          const time_li = benchmarks
            .map((b) => merged_data[b.id]?.[eng.key])
            .filter((t) => typeof t === "number" && t > 0);

          if (time_li.length === 0) return null;
          return {
            ...eng,
            geomean_ms: Number(geoMeanCalc(time_li).toFixed(1)),
          };
        })
        .filter(Boolean),
      jit_li = engine_geomean_li
        .filter((e) => e.mode === "jit")
        .sort((first, second) => first.geomean_ms - second.geomean_ms),
      interp_li = engine_geomean_li
        .filter((e) => e.mode === "interp")
        .sort((first, second) => first.geomean_ms - second.geomean_ms);

    // 循环遍历所有配置语言，文件名由 key 动态派生，零硬编码
    for (const [lang, i18n] of Object.entries(I18N_TEXT)) {
      const svg = svgUnifiedBuild(jit_li, interp_li, env_info, i18n);

      for (const file_name of svgFileNamesGet(lang)) {
        const file_path = join(BENCHMARKS_DIR, file_name);
        await Bun.write(file_path, svg);
        console.log(`✓ [${lang}] 生成: ${file_name}`);
      }
    }
  };

await main();
