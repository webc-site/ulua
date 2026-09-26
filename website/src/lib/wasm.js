let engine = null,
  engine_gen = 0,
  last_runtime_error = "",
  engine_loading_promise = null;

globalThis.__uluaOnRuntimeError = (msg) => {
  last_runtime_error = String(msg || "");
};

const basePathGet = () => (window.location?.pathname || "/").replace(/\/[^/]*$/, "/"),
  wasmResultUnpack = (res) => {
    if (res && typeof res === "object") {
      const output = String(res.output || ""),
        error = String(res.error || "");
      if (typeof res.free === "function") res.free();
      return { output, error };
    }
    return { output: String(res || ""), error: "" };
  },
  dynImport = (url) => new Function("u", "return import(u)")(url),
  wasmErrorExtract = (err) => {
    let msg = last_runtime_error || err?.message || String(err);
    if (!msg || msg === "unreachable") {
      msg = last_runtime_error || "WebAssembly trap: execution aborted";
    }
    return msg;
  };

export const engineLoad = async () => {
    if (engine_loading_promise) return engine_loading_promise;
    engine_loading_promise = (async () => {
      try {
        ++engine_gen;
        const pkg_url = basePathGet() + "pkg/ulua_web.js?gen=" + engine_gen,
          mod = await dynImport(pkg_url);
        // 触发自举：`mod.default()` 是 wasm-bindgen 的 init（就地实例化引擎），
        // 其返回值（Instance exports）在 host 垫片退役后不再有 JS 侧消费者。
        await mod.default();

        engine = { run: mod.run, check: mod.check };
        return engine;
      } finally {
        engine_loading_promise = null;
      }
    })();
    return engine_loading_promise;
  },
  engineEnsure = async () => engine || engineLoad(),
  engineRun = async (source) => {
    await engineEnsure();
    last_runtime_error = "";

    try {
      const res = engine.run(source),
        { output, error } = wasmResultUnpack(res);
      return { output, error, is_runtime_error: false };
    } catch (err) {
      const is_wasm_trap = err instanceof WebAssembly.RuntimeError,
        error = wasmErrorExtract(err);

      if (is_wasm_trap) {
        engine = null;
        try {
          await engineLoad();
        } catch (reload_err) {
          console.error("Engine reload failed:", reload_err);
        }
      }

      return {
        output: "",
        error,
        is_runtime_error: true,
      };
    }
  },
  engineCheck = async (source) => {
    await engineEnsure();
    const raw = engine.check(source);
    if (!raw || raw === "No errors.") return [];

    const line_li = raw.trim().split("\n"),
      diag_li = [];

    for (let i = 0; i < line_li.length; ++i) {
      const line_str = line_li[i].trim();
      if (!line_str) continue;
      const colon_idx = line_str.indexOf(":");
      if (colon_idx !== -1) {
        const line_num = parseInt(line_str.slice(0, colon_idx).trim(), 10),
          message = line_str.slice(colon_idx + 1).trim();
        if (!isNaN(line_num)) {
          diag_li.push({ line: line_num, message });
          continue;
        }
      }
      diag_li.push({ line: 0, message: line_str });
    }
    return diag_li;
  };
