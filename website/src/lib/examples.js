import zh from "./examples/zh.js";
import en from "./examples/en.js";

export const EXAMPLE_KEY_LI = [
    "hello",
    "fibonacci",
    "tables",
    "metatables",
    "strings",
    "coroutines",
    "typed",
    "globals",
    "type_error",
  ],
  exampleCodeGet = (name, lang = "zh") => {
    const dict = lang === "zh" ? zh : en;
    return dict[name] ?? en[name] ?? "";
  };
