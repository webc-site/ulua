<script>
  import BenchDropdown from "./BenchDropdown.svelte";
  import { i18n_state, t } from "../lib/i18n.svelte.js";
  import {
    BENCHMARK_LI,
    ENGINE_LI,
    RESULT_MAP,
    GEOMEAN_LI,
    METADATA,
  } from "../lib/benchData.js";

  let active_tab = $state("overview"),
    selected_bench = $state("fib"),
    show_table = $state(false);

  const env_summary = $derived(
    i18n_state.current_lang === "zh" || i18n_state.current_lang === "zh-TW"
      ? METADATA?.environment?.summary_zh ?? METADATA?.environment?.summary ?? ""
      : METADATA?.environment?.summary_en ?? METADATA?.environment?.summary ?? ""
  );

  const SVG_WIDTH = 840,
    BAR_START_X = 220,
    MAX_BAR_WIDTH = 430,
    ROW_HEIGHT = 46,
    BAR_HEIGHT = 20,
    ZONE1_HEADER_Y = 16,
    ZONE1_ROWS_Y = 52,

    GRAD_MAP = {
      ulua: "url(#grad-ulua)",
      "ulua-jit": "url(#grad-ulua-jit)",
      "mlua/luau": "url(#grad-luau)",
      "mlua/luau-jit": "url(#grad-jit)",
      "mlua/luajit": "url(#grad-luajit)",
      "mlua/luajit-interp": "url(#grad-luajit-interp)",
      "mlua/lua5.4": "url(#grad-lua54)",
    },
    barGrad = (key) => GRAD_MAP[key] ?? "url(#grad-ulua)",

    barItemCalc = (item, val, max_val, y) => ({
      ...item,
      y,
      bar_w: Math.max(16, (val / max_val) * MAX_BAR_WIDTH),
    }),

    overview_max_ms = $derived(
      Math.max(...GEOMEAN_LI.map(({ geomean_ms }) => geomean_ms), 50)
    ),

    overview_jit_sorted = $derived(
      [...GEOMEAN_LI.filter((eng) => eng.mode === "jit")].sort((first, second) => first.geomean_ms - second.geomean_ms)
    ),
    overview_interp_sorted = $derived(
      [...GEOMEAN_LI.filter((eng) => eng.mode === "interp")].sort((first, second) => first.geomean_ms - second.geomean_ms)
    ),

    zone2_header_y = $derived(ZONE1_ROWS_Y + overview_jit_sorted.length * ROW_HEIGHT + 32),
    zone2_rows_y = $derived(zone2_header_y + 36),
    chart_total_height = $derived(zone2_rows_y + overview_interp_sorted.length * ROW_HEIGHT + 14),

    overview_jit_li = $derived(
      overview_jit_sorted.map((eng, idx) =>
        barItemCalc(eng, eng.geomean_ms, overview_max_ms, ZONE1_ROWS_Y + idx * ROW_HEIGHT)
      )
    ),
    overview_interp_li = $derived(
      overview_interp_sorted.map((eng, idx) =>
        barItemCalc(eng, eng.geomean_ms, overview_max_ms, zone2_rows_y + idx * ROW_HEIGHT)
      )
    ),

    detail_all_items = $derived.by(() => {
      const bench_map = RESULT_MAP[selected_bench] ?? {},
        item_li = [];

      for (const eng of ENGINE_LI) {
        const { key } = eng,
          res = bench_map[key];
        if (res && res.time_ms !== null) {
          item_li.push({
            ...eng,
            time_ms: res.time_ms,
            ratio: res.ratio,
          });
        }
      }
      return item_li;
    }),

    detail_max_time = $derived(
      Math.max(...detail_all_items.map(({ time_ms }) => time_ms), 10)
    ),

    detail_jit_li = $derived(
      detail_all_items
        .filter((e) => e.mode === "jit")
        .sort((first, second) => first.time_ms - second.time_ms)
        .map((item, idx) =>
          barItemCalc(item, item.time_ms, detail_max_time, ZONE1_ROWS_Y + idx * ROW_HEIGHT)
        )
    ),

    detail_interp_li = $derived(
      detail_all_items
        .filter((e) => e.mode === "interp")
        .sort((first, second) => first.time_ms - second.time_ms)
        .map((item, idx) =>
          barItemCalc(item, item.time_ms, detail_max_time, zone2_rows_y + idx * ROW_HEIGHT)
        )
    ),

    current_jit_li = $derived(active_tab === "overview" ? overview_jit_li : detail_jit_li),
    current_interp_li = $derived(active_tab === "overview" ? overview_interp_li : detail_interp_li),

    cellInfo = (bench_id, eng) => {
      const { key, is_ulua } = eng,
        item = RESULT_MAP[bench_id]?.[key];
      if (!item || item.time_ms === null) return null;
      const { time_ms, ratio } = item;
      return {
        ms: time_ms + " ms",
        ratio: !is_ulua && ratio ? ratio + "×" : "",
      };
    },

    table_row_li = $derived(
      BENCHMARK_LI.map((bench) => ({
        bench,
        cell_li: ENGINE_LI.map((eng) => ({
          eng,
          info: cellInfo(bench.id, eng),
        })),
      }))
    ),

    tabOverviewSet = () => {
      active_tab = "overview";
    },
    cleanLabel = (label) => (label ? label.replace(/\s*\((?:JIT|解释|Interp)\)/gi, "") : ""),
    engineFamilyGet = (id) => (id ? id.replace(/_jit$|_interp$/, "") : id),
    onBenchSelect = (bench_id) => {
      selected_bench = bench_id;
      active_tab = "detail";
    },
    tableToggle = () => {
      show_table = !show_table;
    };
</script>


<template lang="pug">
section#benchmark.benchmark
  .wrap
    .section-head
      h2.section-title {t("bench.title")}
      p.section-desc {@html t("bench.desc")}
      +if('env_summary')
        p.section-env
          svg.env-icon(viewBox="0 0 24 24" width="13" height="13" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round")
            rect(x="2" y="2" width="20" height="8" rx="2" ry="2")
            rect(x="2" y="14" width="20" height="8" rx="2" ry="2")
            line(x1="6" y1="6" x2="6.01" y2="6")
            line(x1="6" y1="18" x2="6.01" y2="18")
          span.env-label {t("bench.env_label")}：
          span.env-text {env_summary}

    .bench-nav
      .tab-pill-group
        button.tab-btn(class:active={active_tab === 'overview'} onclick={tabOverviewSet})
          span {t("bench.tab_overview")}
        BenchDropdown(
          active={active_tab === 'detail'}
          selected={selected_bench}
          items={BENCHMARK_LI}
          onselect={onBenchSelect}
        )

    .bench-card.box-card
      +if('active_tab === "detail"')
        .bench-meta-banner
          .meta-title-row
            h3.meta-name {t("bench.item." + selected_bench)}
          .meta-info-row
            span.meta-param {t("bench.param." + selected_bench)}
            span.meta-divider •
            span.meta-desc {t("bench.item." + selected_bench + "_desc")}

      .chart-container
        svg.chart-svg(viewBox="0 0 840 " + chart_total_height width="100%" height={chart_total_height})
          defs
            linearGradient#grad-ulua(x1="0%" y1="0%" x2="100%" y2="0%")
              stop(offset="0%" stop-color="#2563eb")
              stop(offset="100%" stop-color="#06b6d4")
            linearGradient#grad-ulua-jit(x1="0%" y1="0%" x2="100%" y2="0%")
              stop(offset="0%" stop-color="#0284c7")
              stop(offset="100%" stop-color="#38bdf8")
            linearGradient#grad-luau(x1="0%" y1="0%" x2="100%" y2="0%")
              stop(offset="0%" stop-color="#059669")
              stop(offset="100%" stop-color="#10b981")
            linearGradient#grad-jit(x1="0%" y1="0%" x2="100%" y2="0%")
              stop(offset="0%" stop-color="#ea580c")
              stop(offset="100%" stop-color="#f59e0b")
            linearGradient#grad-luajit(x1="0%" y1="0%" x2="100%" y2="0%")
              stop(offset="0%" stop-color="#7c3aed")
              stop(offset="100%" stop-color="#a855f7")
            linearGradient#grad-luajit-interp(x1="0%" y1="0%" x2="100%" y2="0%")
              stop(offset="0%" stop-color="#6366f1")
              stop(offset="100%" stop-color="#818cf8")
            linearGradient#grad-lua54(x1="0%" y1="0%" x2="100%" y2="0%")
              stop(offset="0%" stop-color="#64748b")
              stop(offset="100%" stop-color="#94a3b8")

          // 分区 1：即时编译区
          rect.zone-accent(x="16" y={ZONE1_HEADER_Y} width="4" height="16" rx="2")
          text.zone-title(x="27" y={ZONE1_HEADER_Y + 8} dominant-baseline="central") {t("bench.zone_jit")}

          // JIT 行渲染
          +each('current_jit_li as row')
            g.chart-row(class:is-ulua={row.is_ulua})
              text.label-engine(x="16" y={row.y + 13})
                | {cleanLabel(row.label)}
                +if('row.is_ulua')
                  tspan.badge-current(dx="6") ★ {t("bench.current")}

              text.label-lang(x="16" y={row.y + 27}) {t("bench.lang." + engineFamilyGet(row.id))}

              rect.bar-track(
                x={BAR_START_X}
                y={row.y}
                width={MAX_BAR_WIDTH}
                height={BAR_HEIGHT}
                rx="6"
              )

              rect.bar-rect(
                x={BAR_START_X}
                y={row.y}
                width={row.bar_w}
                height={BAR_HEIGHT}
                rx="6"
                fill={barGrad(row.key)}
              )

              text.val-text(x={BAR_START_X + row.bar_w + 14} y={row.y + 10})
                tspan.ms-val {row.geomean_ms ?? row.time_ms}
                tspan.ms-unit(dx="3") ms

          // 分区 2：解释执行区
          rect.zone-accent(x="16" y={zone2_header_y} width="4" height="16" rx="2")
          text.zone-title(x="27" y={zone2_header_y + 8} dominant-baseline="central") {t("bench.zone_interp")}

          // 解释执行行渲染
          +each('current_interp_li as row')
            g.chart-row(class:is-ulua={row.is_ulua})
              text.label-engine(x="16" y={row.y + 13})
                | {cleanLabel(row.label)}
                +if('row.is_ulua')
                  tspan.badge-current(dx="6") ★ {t("bench.current")}

              text.label-lang(x="16" y={row.y + 27}) {t("bench.lang." + engineFamilyGet(row.id))}

              rect.bar-track(
                x={BAR_START_X}
                y={row.y}
                width={MAX_BAR_WIDTH}
                height={BAR_HEIGHT}
                rx="6"
              )

              rect.bar-rect(
                x={BAR_START_X}
                y={row.y}
                width={row.bar_w}
                height={BAR_HEIGHT}
                rx="6"
                fill={barGrad(row.key)}
              )

              text.val-text(x={BAR_START_X + row.bar_w + 14} y={row.y + 10})
                tspan.ms-val {row.geomean_ms ?? row.time_ms}
                tspan.ms-unit(dx="3") ms

      .bench-footer
        .method-info
          svg.bulb-icon(viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round")
            path(d="M9 18h6")
            path(d="M10 22h4")
            path(d="M15.09 14c.18-.98.65-1.74 1.41-2.5A4.65 4.65 0 0 0 18 8 6 6 0 0 0 6 8c0 1 .23 2.23 1.5 3.5A4.61 4.61 0 0 1 8.91 14")
          span.bulb-tip {t("bench.tip_faster")}

        button.btn.btn-secondary.toggle-table-btn(onclick={tableToggle})
          span {show_table ? t("bench.toggle_table_hide") : t("bench.toggle_table_show")}

      +if('show_table')
        .table-wrap
          table.matrix-table
            thead
              tr
                th.th-bench {t("bench.col_bench")}
                +each('ENGINE_LI as eng')
                  th(class:th-ulua={eng.is_ulua})
                    .th-box
                      span {eng.label}
                      +if('eng.mode === "jit"')
                        span.badge-pill.pill-jit JIT
                      +if('eng.mode === "interp"')
                        span.badge-pill.pill-interp Interp
            tbody
              +each('table_row_li as row')
                tr
                  td.td-bench
                    strong {t("bench.item." + row.bench.id)}
                    span.td-desc {t("bench.item." + row.bench.id + "_desc")}
                  +each('row.cell_li as { eng, info }')
                    td(class:td-ulua={eng.is_ulua})
                      +if('info')
                        .cell-stat
                          .num-ms {info.ms}
                          +if('info.ratio')
                            .num-ratio {info.ratio}
                      +if('!info')
                        span.num-na -

</template>

<style lang="stylus">
.benchmark
  padding 56px 0 72px
  background #ffffff

.section-head
  text-align center
  margin-bottom 36px

.section-title
  font-size 1.95rem
  font-weight 750
  color #0f172a
  letter-spacing -0.025em
  margin 0 0 14px

.section-desc
  font-size 1.05rem
  line-height 1.65
  color #475569
  max-width 780px
  margin 0 auto
  :global(code)
    background #f1f5f9
    padding 2px 7px
    border-radius 5px
    font-size 0.9em
    font-weight 600
    color #2563eb

.section-env
  display inline-flex
  align-items center
  justify-content center
  gap 6px
  margin 14px auto 0
  padding 5px 16px
  background #f8fafc
  border 1px solid #e2e8f0
  border-radius 9999px
  font-size 0.82rem
  color #64748b
  box-shadow 0 1px 2px rgba(0, 0, 0, 0.03)

.env-icon
  color #0284c7
  flex-shrink 0

.env-label
  font-weight 600
  color #475569

.env-text
  font-weight 500
  color #0f172a
  font-family ui-monospace, SFMono-Regular, "Roboto Mono", Menlo, monospace
  font-size 0.8rem

.bench-card
  background #ffffff
  border 1px solid #e2e8f0
  border-radius 16px
  overflow hidden
  box-shadow 0 1px 3px rgba(0, 0, 0, 0.04), 0 10px 30px -10px rgba(0, 0, 0, 0.05)

.bench-nav
  display flex
  align-items center
  justify-content center
  flex-wrap wrap
  gap 12px
  margin-bottom 20px

.tab-pill-group
  display inline-flex
  align-items center
  gap 3px
  background #f1f5f9
  padding 4px
  border-radius 9999px
  border 1px solid #e2e8f0
  box-shadow 0 1px 3px rgba(0, 0, 0, 0.04)

.tab-btn
  border none
  background transparent
  padding 7px 18px
  font-size 0.86rem
  font-weight 550
  color #64748b
  border-radius 9999px
  cursor pointer
  transition all 0.18s ease
  user-select none
  &:hover
    color #0f172a
  &.active
    background #ffffff
    color #2563eb
    font-weight 650
    box-shadow 0 2px 6px rgba(0, 0, 0, 0.07), 0 1px 2px rgba(0, 0, 0, 0.04)

.bench-meta-banner
  padding 14px 24px
  background #f8fafc
  border-bottom 1px solid #f1f5f9
  display flex
  flex-direction column
  gap 4px

.meta-title-row
  display flex
  align-items center
  gap 8px

.meta-name
  margin 0
  font-size 0.95rem
  font-weight 700
  color #0f172a

.meta-info-row
  display flex
  align-items center
  flex-wrap wrap
  gap 8px
  font-size 0.82rem
  color #475569

.meta-param
  font-weight 600
  color #2563eb

.meta-divider
  color #cbd5e1

.meta-desc
  color #64748b

.chart-container
  padding 24px 24px 20px
  background #ffffff
  min-height 260px

.chart-svg
  display block
  width 100%
  height auto
  font-family ui-sans-serif, system-ui, -apple-system, sans-serif

.zone-accent
  fill #0284c7

.zone-title
  fill #0f172a
  font-size 13.5px
  font-weight 750
  letter-spacing -0.01em

.bar-track
  fill #f1f5f9
  opacity 0.95

.chart-row
  cursor default
  &:hover
    rect.bar-rect
      opacity 0.92

.badge-current
  font-size 10px
  font-weight 700
  fill #2563eb

.label-engine
  font-size 13px
  font-weight 650
  fill #1e293b
  letter-spacing -0.01em

.is-ulua .label-engine
  fill #0284c7
  font-weight 750

.label-lang
  font-size 10.5px
  fill #64748b

.bar-rect
  transition width 0.4s cubic-bezier(0.16, 1, 0.3, 1)

.val-text
  font-size 12.5px
  dominant-baseline central

.ms-val
  font-size 13px
  font-weight 700
  fill #0f172a
  font-family ui-monospace, SFMono-Regular, "Roboto Mono", Menlo, monospace

.ms-unit
  font-size 11px
  font-weight 500
  fill #64748b

.is-ulua .ms-val
  fill #0284c7

.bench-footer
  display flex
  align-items center
  justify-content space-between
  flex-wrap wrap
  gap 12px
  padding 14px 24px
  background #f8fafc
  border-top 1px solid #e2e8f0

.method-info
  display inline-flex
  align-items center
  gap 8px
  font-size 0.84rem

.bulb-icon
  color #eab308
  flex-shrink 0
  filter drop-shadow(0 1px 2px rgba(234, 179, 8, 0.3))

.bulb-tip
  font-weight 600
  color #334155

.toggle-table-btn
  background #ffffff
  border 1px solid #cbd5e1
  padding 6px 14px
  font-size 0.82rem
  font-weight 600
  color #2563eb
  border-radius 8px
  cursor pointer
  transition all 0.15s ease
  &:hover
    background #f1f5f9
    border-color #94a3b8

.table-wrap
  padding 0 24px 24px
  background #ffffff
  overflow-x auto

.matrix-table
  width 100%
  border-collapse collapse
  font-size 0.86rem
  text-align right
  th, td
    padding 11px 16px
    border-bottom 1px solid #f1f5f9
  th
    background #f8fafc
    color #475569
    font-weight 650
    font-size 0.82rem
    border-top 1px solid #e2e8f0
  .th-box
    display flex
    align-items center
    justify-content flex-end
    gap 6px
  .badge-pill
    display inline-block
    padding 1px 5px
    border-radius 4px
    font-size 0.72rem
    font-weight 700
  .pill-jit
    background #fef3c7
    color #d97706
  .pill-interp
    background #f1f5f9
    color #64748b
  .th-bench, .td-bench
    text-align left
  .th-ulua, .td-ulua
    background rgba(37, 99, 235, 0.04)
    font-weight 650
    color #2563eb
  .td-bench
    strong
      display block
      color #0f172a
    .td-desc
      font-size 0.77rem
      color #64748b
      font-weight 400
  .cell-stat
    display flex
    flex-direction column
    align-items flex-end
    gap 2px
  .num-ms
    font-weight 650
    font-family ui-monospace, SFMono-Regular, monospace
  .num-ratio
    font-size 0.75rem
    color #64748b
    font-weight 550
  .num-na
    color #cbd5e1
</style>
