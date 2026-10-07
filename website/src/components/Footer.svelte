<script>
  import { t } from "../lib/i18n.svelte.js";
  import { GITHUB_URL } from "../lib/const.js";
</script>

<template lang="pug">
footer.foot
  .wrap
    p {@html t("foot.license")}
    p.foot-base {@html t("foot.commit")}
    p.foot-base
      span {t("foot.source")} 
      a(href={GITHUB_URL} target="_blank" rel="noopener") {GITHUB_URL.replace("https://", "")}
</template>

<style lang="stylus">
.foot
  padding 40px 0
  border-top 1px solid #eaeef2
  background #f6f8fa
  text-align center
  font-size 13px
  color #656d76
  line-height 1.6

.foot p
  margin 0 0 6px
  &:last-child
    margin-bottom 0

.foot-base
  color #8c959f

.foot a
  color #0969da
  text-decoration none
  font-weight 500
  &:hover
    text-decoration underline
</style>
