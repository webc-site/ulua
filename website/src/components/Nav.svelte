<script>
  import { onMount } from "svelte";
  import LangSelect from "./LangSelect.svelte";
  import { t } from "../lib/i18n.svelte.js";
  import { GITHUB_URL, LUAU_UPSTREAM_URL, X_URL, BLUESKY_URL } from "../lib/const.js";
  import { scrollSpyInit } from "../lib/scrollSpy.js";
  import logo_svg from "../svg/logo.svg";
  import github_svg from "../svg/github.svg";
  import x_svg from "../svg/x.svg";
  import bluesky_svg from "../svg/bluesky.svg";
  import chevron_svg from "../svg/chevron.svg";

  let active_section = $state("playground"),
    spy_control = null;

  const NAV_SECTION_LI = [
      ["playground", "nav.playground"],
      ["benchmark", "nav.benchmark"],
      ["about", "nav.about"],
      ["features", "nav.features"],
      ["lua-syntax", "nav.lua_syntax"],
      ["luau-syntax", "nav.luau_syntax"],
      ["embed", "nav.embed"],
      ["checker", "nav.checker"],
      ["crates", "nav.crates"],
    ],
    navChange = (e) => {
      const { target } = e,
        { value: val } = target;
      if (!val) return;
      if (val.startsWith("http")) {
        window.open(val, "_blank", "noopener");
        target.value = "#" + active_section;
        return;
      }
      const el = document.querySelector(val);
      if (el) {
        spy_control?.programmaticSet();
        active_section = val.replaceAll("#", "");
        el.scrollIntoView({ behavior: "smooth" });
      }
    };

  onMount(() => {
    spy_control = scrollSpyInit(
      NAV_SECTION_LI.map(([id]) => id),
      (id) => {
        active_section = id;
      }
    );
    return () => {
      spy_control?.destroy();
    };
  });
</script>

<template lang="pug">
header.nav
  .wrap.nav-inner
    .nav-left
      a.brand(href="#top" aria-label="ulua — home")
        img.brand-mark(src={logo_svg} width="28" height="28" alt="ulua logo")
        span.brand-name
          b u
          | lua

      .nav-section-wrapper
        select.nav-section-select.nav-mobile-select(value={'#' + active_section} onchange={navChange} aria-label="Active Section")
          +each('NAV_SECTION_LI as [id, label_key]')
            option(value={'#' + id}) {t(label_key)}
          option(value={LUAU_UPSTREAM_URL}) Luau ↗
        img.nav-section-arrow(src={chevron_svg} width="10" height="10" alt="")
        svg.nav-section-line(preserveAspectRatio="none" viewBox="0 0 100 1")
          line(x1="0" y1="0.5" x2="100" y2="0.5" stroke="currentColor" stroke-width="0.5" vector-effect="non-scaling-stroke")

    .nav-actions
      LangSelect

      a.btn-pill.nav-icon-btn(href={X_URL} target="_blank" rel="noopener" aria-label="X (Twitter)")
        img.nav-icon(src={x_svg} width="13" height="13" alt="X")
      a.btn-pill.nav-icon-btn(href={BLUESKY_URL} target="_blank" rel="noopener" aria-label="Bluesky")
        img.nav-icon(src={bluesky_svg} width="14" height="14" alt="Bluesky")

      a.btn-pill.nav-gh(href={GITHUB_URL} target="_blank" rel="noopener" aria-label="ulua on GitHub")
        img.gh-icon(src={github_svg} width="15" height="15" alt="GitHub")
        span.gh-text GitHub
</template>

<style lang="stylus">
.nav
  position sticky
  top 0
  z-index 50
  background rgba(255, 255, 255, 0.95)
  backdrop-filter blur(12px)
  border-bottom 1px solid #d0d7de

.nav-inner
  display flex !important
  flex-direction row !important
  flex-wrap nowrap !important
  align-items center !important
  justify-content space-between !important
  height 60px
  gap 12px

.nav-left
  display flex
  align-items center
  gap 14px
  flex-shrink 0

.brand
  display inline-flex
  align-items center
  gap 8px
  text-decoration none
  color #1f2328
  flex-shrink 0

.brand-mark
  width 28px
  height 28px
  display block

.brand-name
  font-size 19px
  font-weight 800
  letter-spacing -0.03em
  b
    color #0969da

.nav-section-wrapper
  position relative
  display inline-flex
  align-items center
  height 28px
  background transparent
  border none
  color #c1c9d2
  transition color 0.15s ease
  &:hover
    color #1f2328
  &:focus-within
    color #0969da

.nav-section-select
  height 100%
  padding 0 20px 2px 2px
  font-size 13.5px
  font-weight 600
  color #1f2328
  background transparent
  border none
  border-radius 0
  cursor pointer
  outline none
  appearance none
  -webkit-appearance none
  line-height 26px
  letter-spacing -0.01em
  transition color 0.15s ease
  &:hover
    color #0969da

.nav-section-arrow
  position absolute
  right 2px
  top 50%
  transform translateY(-50%)
  pointer-events none
  opacity 0.6
  transition transform 0.15s ease, opacity 0.15s ease
  .nav-section-wrapper:hover &
    opacity 0.95

.nav-section-line
  position absolute
  bottom 0
  left 0
  width 100%
  height 1px
  display block
  pointer-events none
  overflow visible

.nav-actions
  display flex
  align-items center
  gap 8px
  flex-shrink 0

.nav-icon-btn
  padding 0
  width 32px
  height 32px
  display inline-flex
  align-items center
  justify-content center

.nav-icon
  display block
  opacity 0.85
  transition opacity 0.15s ease
  .nav-icon-btn:hover &
    opacity 1

.nav-gh
  display inline-flex
  align-items center
  gap 6px
  height 32px
  padding 0 12px
  font-size 13px
  font-weight 600
  color #1f2328

.gh-icon
  width 15px
  height 15px
  display block

@media (max-width 480px)
  .nav-gh .gh-text
    display none
  .nav-gh
    padding 0
    width 32px
    height 32px
    justify-content center
</style>
