import CODE_LI from "./locales/CODE.js";
import NAME_LI from "./locales/NAME.js";

const USER_LANG_KEY = "ulua_user_lang",
  LOCALE_IMPORTERS = {
    en: () => import("./locales/en.js"),
    zh: () => import("./locales/zh.js"),
    "zh-TW": () => import("./locales/zh-TW.js"),
    ja: () => import("./locales/ja.js"),
    de: () => import("./locales/de.js"),
    fr: () => import("./locales/fr.js"),
    ru: () => import("./locales/ru.js"),
    es: () => import("./locales/es.js"),
    pt: () => import("./locales/pt.js"),
    ko: () => import("./locales/ko.js"),
    it: () => import("./locales/it.js"),
    hi: () => import("./locales/hi.js"),
    ar: () => import("./locales/ar.js"),
    tr: () => import("./locales/tr.js"),
    id: () => import("./locales/id.js"),
    nl: () => import("./locales/nl.js"),
    pl: () => import("./locales/pl.js"),
    vi: () => import("./locales/vi.js"),
    th: () => import("./locales/th.js"),
    sv: () => import("./locales/sv.js"),
  },
  locale_cache = {},
  langNormalize = (raw) => {
    if (!raw) return null;
    const str = raw.trim();
    if (CODE_LI.includes(str)) return str;
    const lower = str.toLowerCase();
    for (let i = 0; i < CODE_LI.length; ++i) {
      if (CODE_LI[i].toLowerCase() === lower) return CODE_LI[i];
    }
    if (/^zh-(tw|hk|mo|hant)/i.test(lower)) return "zh-TW";
    if (/^zh-(cn|sg|hans)/i.test(lower) || lower === "zh") return "zh";
    const prefix = lower.split("-")[0];
    for (let i = 0; i < CODE_LI.length; ++i) {
      if (CODE_LI[i].toLowerCase() === prefix) return CODE_LI[i];
    }
    return null;
  },
  langFromUrl = () => {
    try {
      return langNormalize(new URLSearchParams(window.location.search).get("lang"));
    } catch (_) {
      return null;
    }
  },
  langFromStorage = () => {
    try {
      return langNormalize(localStorage.getItem(USER_LANG_KEY));
    } catch (_) {
      return null;
    }
  },
  langFromNavigator = () => {
    const raw_li = navigator.languages ?? [navigator.language];
    for (let i = 0; i < raw_li.length; ++i) {
      const matched = langNormalize(raw_li[i]);
      if (matched) return matched;
    }
    return null;
  },
  languageDetect = () => langFromUrl() || langFromStorage() || langFromNavigator() || "en",
  localeLoad = async (lang) => {
    if (locale_cache[lang]) return locale_cache[lang];
    const importer = LOCALE_IMPORTERS[lang] ?? LOCALE_IMPORTERS.en,
      mod = await importer(),
      data = mod.default ?? mod;
    locale_cache[lang] = data;
    return data;
  };

export const i18n_state = $state({
    current_lang: "zh",
    dict: {},
  }),
  LANG_OPTIONS_LI = CODE_LI.map((code, idx) => ({
    code,
    name: NAME_LI[idx] ?? code,
  })),
  languageSet = async (target_lang, is_user_action = false) => {
    const target = langNormalize(target_lang) ?? "en",
      [en_dict, target_dict] = await Promise.all([
        localeLoad("en"),
        target === "en" ? null : localeLoad(target),
      ]),
      merged = target === "en" ? en_dict : { ...en_dict, ...target_dict };

    i18n_state.current_lang = target;
    i18n_state.dict = merged;

    document.documentElement.lang =
      target === "zh" ? "zh-CN" : target === "zh-TW" ? "zh-TW" : target;

    if (merged["meta.title"]) {
      document.title = merged["meta.title"];
    }
    if (merged["meta.description"]) {
      const meta_el = document.querySelector('meta[name="description"]');
      if (meta_el) meta_el.setAttribute("content", merged["meta.description"]);
    }
    if (is_user_action) {
      try {
        localStorage.setItem(USER_LANG_KEY, target);
      } catch (_) {}
    }
  },
  i18nInit = async () => {
    const initial_lang = languageDetect();
    await languageSet(initial_lang, false);
  },
  t = (key) => i18n_state.dict[key] ?? key;
