#!/usr/bin/env -S bun

import { $ } from "bun";
import { optimize } from "svgo";

const ARG_LI = process.argv.slice(2),
  svgContentMinify = (content) => {
    if (!content || !content.trim().startsWith("<")) return content;
    try {
      return optimize(content, {
        multipass: true,
        plugins: ["preset-default"],
      }).data;
    } catch (err) {
      console.error(err);
      return content;
    }
  },
  stagedFileFind = async () => {
    try {
      const res = await $`git diff --cached --name-only --diff-filter=ACM`.text();
      return res.trim().split("\n").filter(Boolean);
    } catch {
      return [];
    }
  },
  svgFileMinify = async (file) => {
    try {
      const original = await Bun.file(file).text();
      if (!original) return false;
      const data = svgContentMinify(original);
      if (data && data.length < original.length) {
        await Bun.write(file, data);
        const saved_len = original.length - data.length,
          ratio_val = ((saved_len / original.length) * 100).toFixed(2);
        console.log("[" + ratio_val + "%] 压缩: " + file + " (减少 " + saved_len + " 字节)");
        return true;
      }
    } catch (err) {
      console.error("压缩失败: " + file, err);
    }
    return false;
  },
  main = async () => {
    const file_li = ARG_LI.length ? ARG_LI : await stagedFileFind(),
      svg_file_li = file_li.filter((file) => file.endsWith(".svg"));
    let has_changed = false;

    for (const file of svg_file_li) {
      if (await svgFileMinify(file)) {
        has_changed = true;
      }
    }

    if (has_changed && !ARG_LI.length) {
      try {
        await $`git add -u`;
      } catch (err) {
        console.error("git add failed:", err);
      }
    }
  };

if (import.meta.main) {
  await main();
}
