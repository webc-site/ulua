<script>
  import Nav from "./components/Nav.svelte";
  import Hero from "./components/Hero.svelte";
  import Playground from "./components/Playground.svelte";
  import Benchmark from "./components/Benchmark.svelte";
  import AboutLuau from "./components/AboutLuau.svelte";
  import Features from "./components/Features.svelte";
  import Syntax from "./components/Syntax.svelte";
  import Embed from "./components/Embed.svelte";
  import TypeChecker from "./components/TypeChecker.svelte";
  import Crates from "./components/Crates.svelte";
  import Footer from "./components/Footer.svelte";
</script>

<template lang="pug">
Nav
main#top
  Hero
  Playground
  Benchmark
  AboutLuau
  Features
  Syntax
  Embed
  TypeChecker
  Crates
Footer
</template>

<style lang="stylus">
main
  display block
  background #ffffff
</style>
