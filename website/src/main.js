import { mount } from "svelte";
import "./app.styl";
import { i18nInit } from "./lib/i18n.svelte.js";
import App from "./App.svelte";

await i18nInit();

const app = mount(App, {
  target: document.getElementById("app"),
});

export default app;
