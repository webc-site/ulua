<script>
  import { i18n_state, t } from "../lib/i18n.svelte.js";

  const MAIN_RS_HTML = $derived(
    i18n_state.current_lang === "zh"
      ? `<span class="tok-com">// 向 Luau 暴露 Rust 闭包并调用运行</span>
<span class="tok-kw">use</span> ulua::{Lua, UserData, UserDataMethods};

<span class="tok-kw">let</span> lua = Lua::new();
lua.globals().set(<span class="tok-str">"add"</span>,
    lua.create_function(|_, (a, b): (i64, i64)| <span class="tok-fn">Ok</span>(a + b))?)?;
<span class="tok-kw">let</span> sum: i64 = lua.load(<span class="tok-str">"return add(2, 3)"</span>).eval()?;   <span class="tok-com">// 5</span>

<span class="tok-com">// 暴露带方法和元方法的自定义 Rust 类型：</span>
<span class="tok-kw">struct</span> <span class="tok-ty">Vec2</span> { x: f64, y: f64 }
<span class="tok-kw">impl</span> <span class="tok-ty">UserData</span> <span class="tok-kw">for</span> <span class="tok-ty">Vec2</span> {
    <span class="tok-kw">fn</span> <span class="tok-fn">add_methods</span>&lt;M: UserDataMethods&lt;Self&gt;&gt;(m: &amp;<span class="tok-kw">mut</span> M) {
        m.add_method(<span class="tok-str">"magnitude"</span>, |_, v, ()| {
            <span class="tok-fn">Ok</span>((v.x * v.x + v.y * v.y).sqrt())
        });
    }
}
lua.globals().set(<span class="tok-str">"v"</span>, lua.create_userdata(Vec2 { x: 3.0, y: 4.0 })?)?;
<span class="tok-kw">let</span> m: f64 = lua.load(<span class="tok-str">"return v:magnitude()"</span>).eval()?;   <span class="tok-com">// 5.0</span>`
      : `<span class="tok-com">// Expose Rust closures to Luau and execute</span>
<span class="tok-kw">use</span> ulua::{Lua, UserData, UserDataMethods};

<span class="tok-kw">let</span> lua = Lua::new();
lua.globals().set(<span class="tok-str">"add"</span>,
    lua.create_function(|_, (a, b): (i64, i64)| <span class="tok-fn">Ok</span>(a + b))?)?;
<span class="tok-kw">let</span> sum: i64 = lua.load(<span class="tok-str">"return add(2, 3)"</span>).eval()?;   <span class="tok-com">// 5</span>

<span class="tok-com">// Expose custom Rust types with methods and metamethods:</span>
<span class="tok-kw">struct</span> <span class="tok-ty">Vec2</span> { x: f64, y: f64 }
<span class="tok-kw">impl</span> <span class="tok-ty">UserData</span> <span class="tok-kw">for</span> <span class="tok-ty">Vec2</span> {
    <span class="tok-kw">fn</span> <span class="tok-fn">add_methods</span>&lt;M: UserDataMethods&lt;Self&gt;&gt;(m: &amp;<span class="tok-kw">mut</span> M) {
        m.add_method(<span class="tok-str">"magnitude"</span>, |_, v, ()| {
            <span class="tok-fn">Ok</span>((v.x * v.x + v.y * v.y).sqrt())
        });
    }
}
lua.globals().set(<span class="tok-str">"v"</span>, lua.create_userdata(Vec2 { x: 3.0, y: 4.0 })?)?;
<span class="tok-kw">let</span> m: f64 = lua.load(<span class="tok-str">"return v:magnitude()"</span>).eval()?;   <span class="tok-com">// 5.0</span>`
  );
</script>

<template lang="pug">
section#embed.embed
  .wrap
    .section-head
      h2 {t("embed.title")}
      p {@html t("embed.desc")}

    .embed-content
      .code-pane
        .pane-head
          span.label main.rs
        pre.code-pane-code
          code {@html MAIN_RS_HTML}
</template>

<style lang="stylus">
.embed
  padding 64px 0
  border-top 1px solid #eaeef2
  background #ffffff

.embed-content
  width 100%
  display flex
  flex-direction column
</style>

