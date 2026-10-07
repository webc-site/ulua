<script>
  import { t } from "../lib/i18n.svelte.js";

  let {
    active = false,
    selected = "fib",
    items = [],
    onselect = () => {},
  } = $props();

  let is_open = $state(false),
    container_el = $state(null);

  const toggleDropdown = (e) => {
    e.stopPropagation();
    is_open = !is_open;
  },

  selectItem = (id) => {
    is_open = false;
    onselect(id);
  },

  onDocClick = (e) => {
    if (container_el && !container_el.contains(e.target)) {
      is_open = false;
    }
  };

  $effect(() => {
    if (is_open) {
      document.addEventListener("click", onDocClick);
      return () => document.removeEventListener("click", onDocClick);
    }
  });
</script>

<template lang="pug">
.bench-dropdown(bind:this={container_el})
  button.dropdown-trigger(
    type="button"
    class:active={active}
    class:open={is_open}
    onclick={toggleDropdown}
    aria-haspopup="listbox"
    aria-expanded={is_open}
  )
    span.trigger-title {active ? t("bench.item." + selected, selected) : t("bench.tab_single", "单项基准明细")}
    svg.arrow-icon(
      viewBox="0 0 20 20"
      width="14"
      height="14"
      fill="none"
      stroke="currentColor"
      stroke-width="2"
      stroke-linecap="round"
      stroke-linejoin="round"
    )
      path(d="M6 8l4 4 4-4")

  +if('is_open')
    .dropdown-menu(role="listbox")
      +each('items as item')
        button.dropdown-item(
          type="button"
          role="option"
          aria-selected={selected === item.id}
          class:selected={selected === item.id}
          onclick={() => selectItem(item.id)}
        )
          span.item-label {t("bench.item." + item.id)}
          +if('selected === item.id')
            span.item-check ✓
</template>

<style lang="stylus">
.bench-dropdown
  position relative
  display inline-block

.dropdown-trigger
  display inline-flex
  align-items center
  gap 8px
  border none
  background transparent
  padding 7px 18px
  font-size 0.88rem
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
  &.open .arrow-icon
    transform rotate(180deg)

.arrow-icon
  color #94a3b8
  transition transform 0.2s ease
  flex-shrink 0

.active .arrow-icon
  color #2563eb

.dropdown-menu
  position absolute
  top calc(100% + 6px)
  right 0
  min-width 200px
  background #ffffff
  border 1px solid #e2e8f0
  border-radius 12px
  padding 6px
  box-shadow 0 12px 28px -6px rgba(15, 23, 42, 0.12), 0 4px 12px -2px rgba(15, 23, 42, 0.04)
  z-index 50
  display flex
  flex-direction column
  gap 2px
  backdrop-filter blur(12px)
  animation dropdownFade 0.15s ease-out

@keyframes dropdownFade
  from
    opacity 0
    transform translateY(-4px)
  to
    opacity 1
    transform translateY(0)

.dropdown-item
  display flex
  align-items center
  justify-content space-between
  width 100%
  border none
  background transparent
  padding 8px 12px
  border-radius 8px
  font-size 0.84rem
  font-weight 550
  color #334155
  text-align left
  cursor pointer
  transition all 0.12s ease
  &:hover
    background #f1f5f9
    color #0f172a
  &.selected
    background #eff6ff
    color #2563eb
    font-weight 650

.item-check
  font-size 0.84rem
  font-weight 700
  color #2563eb
</style>
