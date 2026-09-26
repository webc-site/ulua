<script>
  import { t } from "../lib/i18n.svelte.js";
  import { CRATE_LI, crateGithubUrlGet } from "../lib/const.js";
</script>

<template lang="pug">
section#crates.crates
  .wrap
    .section-head
      h2 {t("crates.title")}
      p {@html t("crates.desc")}

    .crate-table-wrap.box-card
      table.crate-table
        tbody
          +each('CRATE_LI as name')
            tr
              td
                a.crate-link(href={crateGithubUrlGet(name)} target="_blank" rel="noopener")
                  code {name}
                  span.link-icon ↗
              td {@html t("crates." + name)}

    p.crates-foot {@html t("crates.foot")}
</template>

<style lang="stylus">
.crates
  padding 64px 0
  border-top 1px solid #eaeef2
  background #fafbfc

.crate-table-wrap
  width 100%


.crate-table
  width 100%
  border-collapse collapse
  font-size 14px

.crate-table tr
  border-bottom 1px solid #eaeef2
  &:last-child
    border-bottom none
  &:hover
    background #f6f8fa

.crate-table td
  padding 12px 18px
  vertical-align middle
  color #1f2328

.crate-table td:first-child
  width 210px
  font-family 'JetBrains Mono', monospace

/* Ghost 风格链接：透明背景、纯净现代 */
.crate-link
  display inline-flex
  align-items center
  gap 4px
  text-decoration none
  padding 4px 8px
  border-radius 6px
  background transparent
  color #1f2328
  font-weight 600
  transition all 0.15s ease
  &:hover
    background #f6f8fa
    color #0969da
    .link-icon
      color #0969da
      transform translate(1px, -1px)

.crate-table code
  background transparent
  padding 0
  font-size 13px
  color inherit
  font-weight inherit

.link-icon
  font-size 11px
  color #8c959f
  transition transform 0.15s ease, color 0.15s ease

.crates-foot
  text-align center
  font-size 13px
  color #656d76
  margin-top 24px
</style>
