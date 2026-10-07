#!/usr/bin/env bash
# 本地快速词条守卫：website/scripts/i18nCheck.js 只需 bun（无需 cargo / 站点构建），
# 数秒内校验 20 种语言包与源码引用零多余零遗漏；仅在暂存区触及 website/{src,scripts}
# 时触发，把 Pages 作业里 ~7 分钟才暴露的 bench.lang.* 缺键问题挡在提交之前。
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

if git diff --cached --name-only --diff-filter=ACMR | grep -qE '^website/(src|scripts)/'; then
  if ! command -v bun >/dev/null 2>&1; then
    echo "i18n-guard: bun not found; install bun (https://bun.sh) or push with --no-verify as a last resort" >&2
    exit 1
  fi
  # benchData.js 是 gitignore 的生成物；本地缺失时用已入库输入重建，保证守卫可独立运行。
  if [ ! -f website/src/lib/benchData.js ]; then
    bun website/scripts/benchConvert.js
  fi
  exec bun website/scripts/i18nCheck.js
fi
