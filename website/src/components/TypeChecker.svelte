<script>
  import { i18n_state, t } from "../lib/i18n.svelte.js";

  const CHECK_RS_HTML = $derived(
    i18n_state.current_lang === "zh"
      ? `<span class="tok-com">// 脚本运行之前，对照宿主定义进行静态类型校验：</span>
<span class="tok-kw">use</span> ulua::check_with_definitions;

check_with_definitions(
    <span class="tok-str">"local n: number = add(1, 2)"</span>,
    <span class="tok-str">"declare function add(a: number, b: number): number"</span>,
)?;  <span class="tok-com">// 对照宿主声明完成精准的双向类型检查</span>`
      : `<span class="tok-com">// Statically check types against host declarations before running:</span>
<span class="tok-kw">use</span> ulua::check_with_definitions;

check_with_definitions(
    <span class="tok-str">"local n: number = add(1, 2)"</span>,
    <span class="tok-str">"declare function add(a: number, b: number): number"</span>,
)?;  <span class="tok-com">// Precise bidirectional type check against host definitions</span>`
  );
</script>

<template lang="pug">
section#checker.checker
  .wrap
    .section-head
      h2 {t("checker.title")}
      p {@html t("checker.desc")}

    .checker-content
      .code-pane
        .pane-head
          span.label check.rs
        pre.code-pane-code
          code {@html CHECK_RS_HTML}
</template>

<style lang="stylus">
.checker
  padding 64px 0
  border-top 1px solid #eaeef2
  background #fafbfc

.checker-content
  width 100%
  display flex
  flex-direction column
</style>

