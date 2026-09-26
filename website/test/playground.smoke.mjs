#!/usr/bin/env -S bun

// Playground smoke test — boots real static site in headless Chromium and asserts typed run/error behavior.

import { extname, join, normalize } from "node:path";
import { chromium } from "playwright";

const ROOT_DIST = join(import.meta.dirname, "../dist"),
  ROOT_DIR = (await Bun.file(join(ROOT_DIST, "index.html")).exists())
    ? ROOT_DIST
    : join(import.meta.dirname, ".."),
  MIME_MAP = {
    ".html": "text/html; charset=utf-8",
    ".js": "text/javascript; charset=utf-8",
    ".mjs": "text/javascript; charset=utf-8",
    ".css": "text/css; charset=utf-8",
    ".wasm": "application/wasm",
    ".json": "application/json",
    ".luau": "text/plain; charset=utf-8",
    ".lua": "text/plain; charset=utf-8",
  },
  failure_li = [],
  serverStart = () =>
    Bun.serve({
      port: 0,
      hostname: "127.0.0.1",
      fetch: async (req) => {
        try {
          const url = new URL(req.url);
          let path_str = normalize(decodeURIComponent(url.pathname));
          if (path_str === "/" || path_str === "") path_str = "/index.html";
          if (path_str.includes("..")) {
            return new Response("forbidden", { status: 403 });
          }
          const file = Bun.file(join(ROOT_DIR, path_str));
          if (!(await file.exists())) {
            return new Response("not found", { status: 404 });
          }
          return new Response(file, {
            headers: { "content-type": MIME_MAP[extname(path_str)] ?? "application/octet-stream" },
          });
        } catch {
          return new Response("not found", { status: 404 });
        }
      },
    }),
  testCheck = (name, cond, detail) => {
    if (cond) {
      console.log("  ok  " + name);
    } else {
      console.log("FAIL  " + name + "  — " + (detail ?? ""));
      failure_li.push(name);
    }
  },
  testRun = async () => {
    const server = serverStart(),
      base = "http://127.0.0.1:" + server.port,
      browser = await chromium.launch(),
      page = await browser.newPage(),
      page_error_li = [];

    page.on("pageerror", (err) => page_error_li.push(String(err)));
    await page.goto(base + "/index.html?lang=en", { waitUntil: "load" });
    await page.waitForSelector("#example-select");

    // ── 1. engine layer: typed {output, error}, fresh module per case ─────────
    const eng = await page.evaluate(async () => {
      const app_handler = globalThis.__uluaOnRuntimeError,
        out = {},
        fresh = async (tag) => {
          const mod = await import("./pkg/ulua_web.js?smoke=" + tag);
          await mod.default();
          return mod;
        };
      let last_err = "";
      globalThis.__uluaOnRuntimeError = (msg) => {
        last_err = String(msg ?? "");
      };

      let mod = await fresh("g");
      {
        const res = mod.run('print(_G)\nfor i, v in _G do print(i) end\nprint("Done")');
        out.gG = { errorEmpty: res.error === "", endsDone: res.output.trim().endsWith("Done") };
        res.free();
      }
      {
        const res = mod.run("local x =");
        out.compile = { errorNonEmpty: res.error.length > 0 };
        res.free();
      }

      mod = await fresh("rt");
      last_err = "";
      {
        let is_rt = false;
        try {
          mod.run('error("kaboom-smoke")');
        } catch (err) {
          is_rt = err instanceof WebAssembly.RuntimeError;
        }
        out.runtime = { isWasmRuntimeError: is_rt, msg: last_err };
      }
      globalThis.__uluaOnRuntimeError = app_handler;
      return out;
    });

    testCheck("engine: _G run has empty (typed) error", eng.gG.errorEmpty);
    testCheck("engine: _G output ends with Done", eng.gG.endsDone);
    testCheck("engine: compile error sets the error field", eng.compile.errorNonEmpty);
    testCheck(
      "engine: runtime error traps as WebAssembly.RuntimeError",
      eng.runtime.isWasmRuntimeError,
    );
    testCheck(
      "engine: runtime error message bridged to JS",
      eng.runtime.msg.includes("kaboom-smoke"),
      "got: " + eng.runtime.msg,
    );

    // ── 2. real UI: the globals example must classify OK (out-ok), not red ────
    const ui1 = await page.evaluate(async () => {
      const sel = document.getElementById("example-select");
      sel.value = "globals";
      sel.dispatchEvent(new Event("change"));
      await new Promise((resolve) => setTimeout(resolve, 80));
      document.getElementById("btn-run").click();
      await new Promise((resolve) => setTimeout(resolve, 500));
      const span = document.querySelector("#output span");
      return {
        cls: span?.className,
        text: (document.getElementById("output").textContent || "").trim(),
      };
    });
    testCheck(
      "ui: globals example is present + classified out-ok",
      Boolean(ui1.cls?.includes("out-ok")),
      "class=" + ui1.cls,
    );
    testCheck("ui: globals output ends with Done", ui1.text.endsWith("Done"));

    // ── 3. real UI: a runtime error → out-err with bridged message ────────
    await page.locator(".cm-content").click();
    await page.keyboard.press("ControlOrMeta+a");
    await page.locator(".cm-content").fill('error("ui-smoke-runtime")');
    const ui2 = await page.evaluate(async () => {
      document.getElementById("btn-run").click();
      await new Promise((resolve) => setTimeout(resolve, 700));
      const span = document.querySelector("#output span");
      return {
        cls: span?.className,
        text: span?.textContent || "",
        status: document.getElementById("status")?.textContent,
      };
    });
    testCheck(
      "ui: runtime error classified out-err",
      Boolean(ui2.cls?.includes("out-err")),
      "class=" + ui2.cls,
    );
    testCheck(
      "ui: runtime error shows the bridged message",
      ui2.text.includes("ui-smoke-runtime"),
      "got: " + ui2.text,
    );
    testCheck(
      "ui: status reads 'runtime error'",
      ui2.status === "runtime error",
      "got: " + ui2.status,
    );

    // ── 4. real UI: engine recovers after the trap (next run works) ───────────
    const ui3 = await page.evaluate(async () => {
      const sel = document.getElementById("example-select");
      sel.value = "hello";
      sel.dispatchEvent(new Event("change"));
      await new Promise((resolve) => setTimeout(resolve, 100));
      document.getElementById("btn-run").click();
      await new Promise((resolve) => setTimeout(resolve, 600));
      const span = document.querySelector("#output span");
      return { cls: span?.className, first: (span?.textContent || "").split("\n")[0] };
    });
    testCheck(
      "ui: engine recovers after a trap (next run is out-ok)",
      Boolean(ui3.cls?.includes("out-ok")),
      "class=" + ui3.cls,
    );
    testCheck("ui: recovered run produces output", ui3.first.length > 0);

    // ── 4.5. real UI: type checker detects static type errors in type_error example ──
    const ui_typecheck = await page.evaluate(async () => {
      const sel = document.getElementById("example-select");
      sel.value = "type_error";
      sel.dispatchEvent(new Event("change"));
      await new Promise((resolve) => setTimeout(resolve, 150));
      document.getElementById("btn-check").click();
      await new Promise((resolve) => setTimeout(resolve, 400));

      const status = document.getElementById("status")?.textContent || "",
        diag_report = Boolean(document.querySelector(".diag-report")),
        diag_rows = Array.from(document.querySelectorAll(".diag-row")).map((row) => ({
          ln: row.querySelector(".diag-ln")?.textContent.trim(),
          msg: row.querySelector(".diag-msg")?.textContent.trim(),
        }));

      return { status, diag_report, diag_rows };
    });

    testCheck(
      "ui: type checker triggers diagnostic report",
      ui_typecheck.diag_report && ui_typecheck.diag_rows.length === 2,
      "rows=" + ui_typecheck.diag_rows.length,
    );
    testCheck(
      "ui: type checker reports line 10 and line 12 errors",
      ui_typecheck.diag_rows[0]?.ln === "L10" && ui_typecheck.diag_rows[1]?.ln === "L12",
      "got: " + JSON.stringify(ui_typecheck.diag_rows),
    );
    testCheck(
      "ui: type checker diagnosis explains type mismatch",
      ui_typecheck.diag_rows[0]?.msg?.includes("Expected this to be 'number'") ||
        ui_typecheck.diag_rows[0]?.msg?.includes("string"),
      "msg=" + ui_typecheck.diag_rows[0]?.msg,
    );

    // ── 5. Chinese locale & Syntax documentation completeness ───────────────
    const zh_context = await browser.newContext({ locale: "zh-CN" }),
      zh_page = await zh_context.newPage();
    zh_page.on("pageerror", (err) => console.log("zh_page pageerror:", err));
    zh_page.on("console", (msg) => console.log("zh_page console:", msg.text()));
    await zh_page.goto(base + "/index.html", { waitUntil: "load" });
    await zh_page.waitForSelector(".syntax-card");

    const zh_check = await zh_page.evaluate(async () => {
      const has_faq = Boolean(document.getElementById("faq")),
        has_lua_section = Boolean(document.getElementById("lua-syntax")),
        has_luau_section = Boolean(document.getElementById("luau-syntax")),
        lua_card_li = Array.from(
          document.querySelectorAll("#lua-syntax .syntax-card .syntax-title"),
        ).map((el) => el.textContent.trim()),
        luau_card_li = Array.from(
          document.querySelectorAll("#luau-syntax .syntax-card .syntax-title"),
        ).map((el) => el.textContent.trim()),
        lua_group_li = Array.from(document.querySelectorAll("#lua-syntax .syntax-group-title")).map(
          (el) => el.textContent.trim(),
        ),
        luau_group_li = Array.from(
          document.querySelectorAll("#luau-syntax .syntax-group-title"),
        ).map((el) => el.textContent.trim()),
        all_group_titles = [...lua_group_li, ...luau_group_li],
        has_untranslated_groups = all_group_titles.some((g) => g.startsWith("syntax.")),
        all_card_titles = [...lua_card_li, ...luau_card_li],
        has_english_notes = all_card_titles.some((title) => /\([A-Za-z\s-]+\)/.test(title)),
        html_lang = document.documentElement.lang,
        nav_text =
          document.querySelector(".nav-section-select option:checked")?.textContent.trim() ||
          document.querySelector(".nav-active-pill")?.textContent.trim() ||
          "",
        x_btn = document.querySelector('a[aria-label="X (Twitter)"]'),
        bsky_btn = document.querySelector('a[aria-label="Bluesky"]'),
        x_has_no_text = x_btn && x_btn.textContent.trim() === "",
        bsky_has_no_text = bsky_btn && bsky_btn.textContent.trim() === "",
        wrap_el = document.querySelector(".wrap"),
        wrap_max_width = wrap_el ? getComputedStyle(wrap_el).maxWidth : "",
        crate_link = document.querySelector(".crate-link"),
        crate_link_bg = crate_link ? getComputedStyle(crate_link).backgroundColor : "",
        intro_boxes = document.querySelectorAll(".syntax-intro-box").length,
        default_lua_open = document.querySelectorAll("#lua-syntax .syntax-card.open").length,
        lua_toggle_btn = document.querySelector("#lua-syntax .syntax-toggle-btn");

      if (lua_toggle_btn) lua_toggle_btn.click();
      await new Promise((resolve) => setTimeout(resolve, 50));
      const expanded_lua_count = document.querySelectorAll("#lua-syntax .syntax-card.open").length;
      if (lua_toggle_btn) lua_toggle_btn.click();
      await new Promise((resolve) => setTimeout(resolve, 50));
      const collapsed_lua_count = document.querySelectorAll("#lua-syntax .syntax-card.open").length;

      const editor_text_initial = document.querySelector(".cm-content")?.textContent || "",
        ready_prompt_initial = document.querySelector("#output .out-ready")?.textContent || "",
        has_zh_comment = editor_text_initial.includes("经典的初始示例"),
        lang_btn = document.querySelector(".lang-btn");

      if (lang_btn) lang_btn.click();
      await new Promise((resolve) => setTimeout(resolve, 50));
      const lang_item_li = Array.from(document.querySelectorAll(".lang-item")).map((el) =>
          el.textContent.trim(),
        ),
        en_option = Array.from(document.querySelectorAll(".lang-item")).find(
          (el) => el.textContent.trim() === "English",
        );
      if (en_option) en_option.click();
      await new Promise((resolve) => setTimeout(resolve, 200));

      const editor_text_after_switch = document.querySelector(".cm-content")?.textContent || "",
        ready_prompt_en = document.querySelector("#output .out-ready")?.textContent || "",
        has_en_comment = editor_text_after_switch.includes("The classic first program"),
        mobile_select = document.querySelector(".nav-section-select, .nav-mobile-select"),
        mobile_option_li = mobile_select
          ? Array.from(mobile_select.options).map((opt) => opt.value)
          : [],
        feature_svg_li = document.querySelectorAll(".feature-icon svg"),
        luau_link = document.querySelector(".luau-link"),
        has_luau_link = luau_link && luau_link.getAttribute("href") === "https://luau.org",
        has_benchmark = Boolean(document.querySelector("#benchmark")),
        syntax_tags_count = document.querySelectorAll(".syntax-intro-tag").length,
        about_badge_count = document.querySelectorAll(".about-badge").length;

      return {
        has_faq,
        has_lua_section,
        has_luau_section,
        lua_card_count: lua_card_li.length,
        lua_group_count: lua_group_li.length,
        luau_card_count: luau_card_li.length,
        luau_group_count: luau_group_li.length,
        intro_boxes,
        syntax_tags_count,
        about_badge_count,
        lang_count: lang_item_li.length,
        first_lang: lang_item_li[0] || "",
        second_lang: lang_item_li[1] || "",
        has_untranslated_groups,
        has_english_notes,
        html_lang,
        nav_text,
        first_title: lua_card_li[0] || "",
        x_has_no_text: Boolean(x_has_no_text),
        bsky_has_no_text: Boolean(bsky_has_no_text),
        wrap_max_width,
        crate_link_bg,
        default_lua_open,
        expanded_lua_count,
        collapsed_lua_count,
        has_zh_comment,
        has_en_comment,
        ready_prompt_initial,
        ready_prompt_en,
        mobile_options_len: mobile_option_li.length,
        feature_svgs_len: feature_svg_li.length,
        has_luau_link: Boolean(has_luau_link),
        has_benchmark,
      };
    });

    testCheck(
      "i18n: default zh-CN browser displays Chinese without ?lang=",
      zh_check.html_lang === "zh-CN" && zh_check.nav_text === "在线体验",
      "got " + zh_check.nav_text,
    );
    testCheck(
      "i18n: ready prompt updates synchronously on language switch",
      zh_check.ready_prompt_initial.includes("引擎已就绪") &&
        zh_check.ready_prompt_en.includes("Engine ready"),
      "zh=" + zh_check.ready_prompt_initial + " en=" + zh_check.ready_prompt_en,
    );
    testCheck(
      "syntax: vertical layout with separate Lua 5.1 and Luau sections",
      zh_check.has_lua_section &&
        zh_check.has_luau_section &&
        zh_check.lua_card_count === 18 &&
        zh_check.lua_group_count === 5 &&
        zh_check.luau_card_count === 27 &&
        zh_check.luau_group_count === 6,
      "lua=" + zh_check.lua_card_count + " luau=" + zh_check.luau_card_count,
    );
    testCheck(
      "syntax: narrative intro callout boxes rendered for both sections",
      zh_check.intro_boxes === 2,
    );
    testCheck(
      "syntax: decorative tags completely removed across sections",
      zh_check.syntax_tags_count === 0 && zh_check.about_badge_count === 0,
    );
    testCheck(
      "i18n: 20 languages supported ordered by national influence",
      zh_check.lang_count === 20 &&
        zh_check.first_lang === "English" &&
        zh_check.second_lang === "中文 (简体)",
      "count=" + zh_check.lang_count + " 1st=" + zh_check.first_lang,
    );
    const { exitCode: i18n_code } = Bun.spawnSync([
      "bun",
      join(import.meta.dirname, "../scripts/i18nCheck.js"),
    ]);
    testCheck("i18n: zero parity error and zero unused/missing keys", i18n_code === 0);
    testCheck(
      "syntax: group titles cleanly localized without raw syntax.* keys",
      !zh_check.has_untranslated_groups,
    );
    testCheck(
      "syntax: Chinese titles have no English annotations in parentheses",
      !zh_check.has_english_notes,
    );
    testCheck(
      "syntax: default collapsed and expand/collapse all works",
      zh_check.default_lua_open === 0 &&
        zh_check.expanded_lua_count === 18 &&
        zh_check.collapsed_lua_count === 0,
    );
    testCheck(
      "nav: X and Bluesky icons present with no text",
      zh_check.x_has_no_text && zh_check.bsky_has_no_text,
    );
    testCheck(
      "layout: wrap max-width is 980px",
      zh_check.wrap_max_width === "980px",
      "got " + zh_check.wrap_max_width,
    );
    testCheck(
      "crates: ghost link background is transparent",
      zh_check.crate_link_bg === "rgba(0, 0, 0, 0)" || zh_check.crate_link_bg === "transparent",
    );
    testCheck(
      "examples: responsive comment sync between zh and en",
      zh_check.has_zh_comment && zh_check.has_en_comment,
      "zh=" + zh_check.has_zh_comment + " en=" + zh_check.has_en_comment,
    );
    testCheck("faq: FAQ section completely removed", !zh_check.has_faq);
    testCheck(
      "nav: mobile select dropdown present with 10 options",
      zh_check.mobile_options_len === 10,
      "got " + zh_check.mobile_options_len,
    );
    testCheck(
      "features: 3 cards have colored svgs without background",
      zh_check.feature_svgs_len === 3,
      "got " + zh_check.feature_svgs_len,
    );
    testCheck("benchmark: Benchmark section present with SVG chart", zh_check.has_benchmark);
    testCheck("about: Luau introduction section present with link", zh_check.has_luau_link);

    await zh_context.close();

    testCheck("no uncaught page errors", page_error_li.length === 0, page_error_li.join(" | "));

    await browser.close();
    server.stop();

    if (failure_li.length) {
      console.error("\n" + failure_li.length + " check(s) failed: " + failure_li.join(", "));
      process.exit(1);
    }
    console.log("\nAll playground smoke checks passed.");
  };

if (import.meta.main) {
  await testRun();
}
