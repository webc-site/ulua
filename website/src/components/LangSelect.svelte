<script>
  import { i18n_state, languageSet, LANG_OPTIONS_LI, t } from "../lib/i18n.svelte.js";
  import i18n_svg from "../svg/i18n.svg";
  import chevron_svg from "../svg/chevron.svg";

  let is_open = $state(false);

  const dropdownToggle = () => {
      is_open = !is_open;
    },
    langSelect = (l) => {
      is_open = false;
      languageSet(l, true);
    },
    currentLangName = () => {
      const cur = LANG_OPTIONS_LI.find((item) => item.code === i18n_state.current_lang);
      return cur ? cur.name : i18n_state.current_lang;
    };
</script>

<template lang="pug">
.lang-dropdown(class:open={is_open})
  button.btn-pill.lang-btn(type="button" onclick={dropdownToggle} aria-label={t("nav.lang")})
    img.lang-icon(src={i18n_svg} width="15" height="15" alt="Language")
    span.lang-text {currentLangName()}
    img.lang-arrow(src={chevron_svg} width="11" height="11" alt="")
  +if('is_open')
    .lang-menu
      +each('LANG_OPTIONS_LI as item')
        button.lang-item(
          type="button"
          class:active={i18n_state.current_lang === item.code}
          onclick={() => langSelect(item.code)}
        ) {item.name}
</template>

<style lang="stylus">
.lang-dropdown
  position relative
  display inline-block
  flex-shrink 0

.lang-btn
  color #1f2328
  gap 5px

.lang-icon
  width 15px
  height 15px
  display block
  color #1f2328

.lang-arrow
  width 11px
  height 11px
  display block
  color #1f2328
  transition transform 0.2s ease

.lang-dropdown.open .lang-arrow
  transform rotate(180deg)

.lang-menu
  position absolute
  right 0
  top calc(100% + 6px)
  background #ffffff
  border 1px solid #d0d7de
  border-radius 8px
  box-shadow 0 8px 24px rgba(140, 149, 159, 0.2)
  padding 4px
  min-width 140px
  max-height 320px
  overflow-y auto
  z-index 100
  display flex
  flex-direction column
  gap 2px

.lang-item
  display block
  width 100%
  text-align left
  padding 7px 10px
  border none
  background transparent
  border-radius 6px
  font-size 13px
  color #1f2328
  cursor pointer
  white-space nowrap
  transition background 0.12s ease
  &:hover
    background #f6f8fa
  &.active
    font-weight 600
    color #0969da
    background #ddf4ff

@media (max-width 480px)
  .lang-text
    display none
  .lang-btn
    padding 0 8px
</style>
