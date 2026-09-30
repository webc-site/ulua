import compileStylus from "@1-/stylus";
import { readFile } from "node:fs/promises";

// Svelte style 预处理器：让组件内 <style lang="stylus"> 用 @1-/stylus 编译
export const stylusSveltePreprocess = () => ({
  name: "svelte-stylus-preprocessor",
  style: async ({ content, attributes, filename }) => {
    if (attributes.lang !== "stylus" && attributes.lang !== "styl") return;
    const renderer = compileStylus(content, {
      filename: filename || "style.styl",
      sourcemap: false,
    });
    const code = renderer.render();
    return {
      code,
      dependencies: renderer.deps ? renderer.deps() : [],
    };
  },
});

// Vite 插件：支持直接 import "*.styl"
export const stylusVitePlugin = () => ({
  name: "vite-plugin-custom-stylus",
  enforce: "pre",
  async resolveId(source, importer) {
    const [pathname, query] = source.split("?");
    if (pathname.endsWith(".styl")) {
      const resolved = await this.resolve(source, importer, { skipSelf: true });
      if (resolved) {
        const [res_path, res_query] = resolved.id.split("?"),
          suffix = query || res_query,
          target_path = res_path.endsWith(".css") ? res_path : res_path + ".css";
        return suffix ? target_path + "?" + suffix : target_path;
      }
    }
  },
  async load(id) {
    const [pathname] = id.split("?");
    if (pathname.endsWith(".styl.css")) {
      const original_path = pathname.slice(0, -4);
      this.addWatchFile(original_path);

      const content = await readFile(original_path, "utf-8"),
        renderer = compileStylus(content, {
          filename: original_path,
          sourcemap: true,
        }),
        css = renderer.render();

      return {
        code: css,
        map: renderer.sourcemap,
      };
    }
  },
});
