<script>
  import { t } from "../lib/i18n.svelte.js";
</script>

<template lang="pug">
section.hero
  .wrap
    h1.hero-title
      span.hero-pre {t("hero.title_pre")}
      span.hero-grad {t("hero.title_grad")}
</template>

<style lang="stylus">
.hero
  padding 52px 0 32px
  text-align center
  background #ffffff

.hero-title
  font-size 48px
  font-weight 700
  font-variation-settings 'wght' 700
  letter-spacing -0.02em
  line-height 1.15
  margin 0
  display inline-flex
  align-items center
  justify-content center
  gap 4px
  flex-wrap wrap

.hero-pre
  background linear-gradient(180deg, #111827 0%, #374151 100%)
  background-clip text
  -webkit-background-clip text
  color transparent
  -webkit-text-fill-color transparent
  filter drop-shadow(0 1px 1px rgba(0, 0, 0, 0.04))

.hero-grad
  background linear-gradient(135deg, #0969da 0%, #2563eb 50%, #4f46e5 100%)
  background-clip text
  -webkit-background-clip text
  color transparent
  -webkit-text-fill-color transparent
  filter drop-shadow(0 1px 2px rgba(9, 105, 218, 0.12))

@media (max-width 640px)
  .hero
    padding 36px 0 24px
  .hero-title
    font-size 34px
</style>
