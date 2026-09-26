import pug2htm from "./toHtm.js";

const extractReplaceInclude = (begin, end, replace, html) => {
  const p = html.indexOf(begin);
  if (p < 0) return;
  if (end) {
    let e = html.indexOf(end, p + begin.length);
    if (e < 0) return;
    e += end.length;
    return html.slice(0, p) + replace(html.slice(p, e)) + html.slice(e);
  }
  return html.slice(0, p) + replace(html.slice(p));
};

const render = (content, filename, locals = {}) =>
  pug2htm(content, filename, locals)[0].replace(/=(["'])\{([^]*?)\}\1/g, "={$2}");

export default () => ({
  name: "svelte-pug-preprocessor",
  markup: ({ content, filename }) => {
    const code = extractReplaceInclude(
      '<template lang="pug">',
      "</template>",
      (pug_block) => "\n" + render(pug_block.slice(21, -11), filename),
      content,
    );
    if (code) {
      return {
        code,
        map: {
          version: 3,
          file: filename,
          sources: [filename],
          sourcesContent: [content],
          names: [],
          mappings: "",
        },
      };
    }
  },
});
