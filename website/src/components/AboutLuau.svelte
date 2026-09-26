<script>
  import { t } from "../lib/i18n.svelte.js";

  const PILLAR_LI = [
    [
      "p1",
      `<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="#1f2328" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/><polyline points="9 12 11 14 15 10"/></svg>`,
    ],
    [
      "p2",
      `<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="#1f2328" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><rect width="20" height="14" x="2" y="5" rx="2"/><line x1="2" x2="22" y1="10" y2="10"/></svg>`,
    ],
    [
      "p3",
      `<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="#1f2328" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><polygon points="12 2 2 7 12 12 22 7 12 2"/><polyline points="2 17 12 22 22 17"/><polyline points="2 12 12 17 22 12"/></svg>`,
    ],
  ];
</script>

<template lang="pug">
section#about.about
  .wrap
    .about-flex
      .about-main
        .about-head
          h2.about-title {@html t("about.title")}
        .about-body
          p.about-p {@html t("about.lead")}
          p.about-p {@html t("about.text")}

      .about-pillars
        +each('PILLAR_LI as [id, icon]')
          .pillar-card.box-card.box-card-hover
            .pillar-head
              span.pillar-icon {@html icon}
              h3.pillar-title {t("about." + id + "_title")}
            p.pillar-desc {@html t("about." + id + "_desc")}
</template>

<style lang="stylus">
.about
  padding 52px 0 36px
  background #ffffff

.about-flex
  display flex
  align-items stretch
  gap 36px
  width 100%

@media (max-width 860px)
  .about-flex
    flex-direction column
    gap 24px

.about-main
  flex 1.2
  display flex
  flex-direction column
  justify-content center

.about-head
  display flex
  flex-direction column

.about-title
  font-size 26px
  font-weight 800
  color #1f2328
  letter-spacing -0.02em
  line-height 1.3
  margin 0 0 16px

@media (max-width 480px)
  .about-title
    font-size 21px
    margin 0 0 14px

.about-body
  display flex
  flex-direction column
  gap 14px

.about-p
  font-size 14.5px
  font-weight 400
  color #24292f
  line-height 1.8
  margin 0

:global(.crate-code-link)

  text-decoration none
  color inherit
  display inline-flex
  align-items center
  transition opacity 0.15s ease
  &:hover
    opacity 0.85

:global(.crate-code-link:hover code)
  border-color #0969da

.about-pillars
  flex 1
  display flex
  flex-direction column
  justify-content space-between
  gap 12px

.pillar-card
  flex 1
  display flex
  flex-direction column
  justify-content center
  padding 16px 18px


.pillar-head
  display flex
  align-items center
  gap 10px
  margin-bottom 6px

.pillar-icon
  display flex
  align-items center
  justify-content center
  color #1f2328
  flex-shrink 0

.pillar-title
  font-size 14px
  font-weight 700
  color #1f2328
  margin 0
  line-height 1.4

.pillar-desc
  font-size 12.5px
  color #57606a
  line-height 1.6
  margin 0

:global(.pillar-desc code)
  font-family 'JetBrains Mono', monospace
  font-size 11.5px
  background #f6f8fa
  color #0969da
  font-weight 600
  padding 2px 7px
  margin 0 5px
  border-radius 4px
  border 1px solid #d0d7de
</style>
