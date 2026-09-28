<script>
  import { t } from "../lib/i18n.svelte.js";

  const FEATURE_LI = [
    [
      "f1",
      `<svg width="44" height="44" viewBox="0 0 24 24" fill="none" stroke="#0969da" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/><path d="m8 11 2 2 4-4"/></svg>`,
    ],
    [
      "f2",
      `<svg width="44" height="44" viewBox="0 0 24 24" fill="none" stroke="#d97706" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2"/></svg>`,
    ],
    [
      "f3",
      `<svg width="44" height="44" viewBox="0 0 24 24" fill="none" stroke="#1a7f37" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/><circle cx="12" cy="11" r="1.5"/><path d="M12 12.5v3"/></svg>`,
    ],
  ];
</script>

<template lang="pug">
section#features.features
  .wrap
    .section-head
      h2 {t("features.title")}
      p {@html t("features.desc")}

    .features-grid
      +each('FEATURE_LI as [id, svg]')
        .feature-card.box-card.box-card-hover
          .feature-text
            h3.feature-title {t("features." + id + "_title")}
            p.feature-desc {t("features." + id + "_desc")}
          .feature-icon
            span {@html svg}
</template>

<style lang="stylus">
.features
  padding 40px 0 60px
  background #ffffff

.features-grid
  display grid
  grid-template-columns repeat(3, 1fr)
  gap 20px
  width 100%

@media (max-width 768px)
  .features-grid
    grid-template-columns 1fr

.feature-card
  border-radius 14px
  padding 24px 22px 28px
  display flex
  flex-direction column
  justify-content space-between

.feature-text

  display flex
  flex-direction column

.feature-title
  font-size 16px
  font-weight 700
  color #1f2328
  margin 0 0 10px
  line-height 1.4

.feature-desc
  font-size 13px
  color #57606a
  line-height 1.6
  margin 0

.feature-icon
  display flex
  align-items center
  justify-content center
  margin-top 24px
  padding-top 12px
  flex-shrink 0
  transition transform 0.2s ease
  .feature-card:hover &
    transform scale(1.08)
</style>
