<script>
  import { onMount } from "svelte";
  import { EditorView, basicSetup } from "codemirror";
  import { EditorState } from "@codemirror/state";
  import { keymap } from "@codemirror/view";
  import { StreamLanguage, syntaxHighlighting, HighlightStyle } from "@codemirror/language";
  import { lua } from "@codemirror/legacy-modes/mode/lua";
  import { tags } from "@lezer/highlight";

  import { i18n_state, t } from "../lib/i18n.svelte.js";
  import { EXAMPLE_KEY_LI, exampleCodeGet } from "../lib/examples.js";
  import { engineLoad, engineRun, engineCheck } from "../lib/wasm.js";

  let editor_container,
    editor_bottom_el,
    cm_view = null,
    selected_example = $state("hello"),
    status_key = $state("ready"),
    status_count = $state(0),
    status_kind = $state("ready"),
    output_state = $state({ kind: "loading", text: "", is_error: false, diag_li: [] }),
    is_ready = $state(false),
    auto_check_timer = null,
    suppress_check = false,
    last_lang = i18n_state.current_lang;

  const status_display = $derived.by(() => {
    if (status_count > 0) {
      const unit =
        status_count === 1
          ? t("status.error")
          : t("status.errors");
      return `${status_count} ${unit}`;
    }
    return t("status." + status_key);
  });

  $effect(() => {
    const { current_lang } = i18n_state;
    if (current_lang !== last_lang) {
      if (cm_view) {
        const { state } = cm_view,
          current_doc = state.doc.toString(),
          prev_code = exampleCodeGet(selected_example, last_lang);
        if (current_doc === prev_code) {
          const new_code = exampleCodeGet(selected_example, current_lang);
          cm_view.dispatch({
            changes: { from: 0, to: state.doc.length, insert: new_code },
          });
        }
      }
      last_lang = current_lang;
    }
  });

  // 高饱和、高对比度明亮现代语法高亮主题（告别泛白过淡）
  const VIVID_LIGHT_HIGHLIGHT_STYLE = HighlightStyle.define([
      { tag: [tags.keyword, tags.definitionKeyword, tags.controlKeyword], color: "#0550ae", fontWeight: "600" },
      { tag: [tags.typeName, tags.className], color: "#8250df", fontWeight: "600" },
      { tag: [tags.string, tags.special(tags.string)], color: "#0a6c2f", fontWeight: "500" },
      { tag: [tags.number, tags.integer, tags.float], color: "#b35900", fontWeight: "500" },
      { tag: [tags.bool, tags.null], color: "#cf222e", fontWeight: "600" },
      { tag: [tags.comment, tags.lineComment, tags.blockComment], color: "#57606a", fontStyle: "italic" },
      { tag: [tags.operator], color: "#0550ae", fontWeight: "600" },
      { tag: [tags.punctuation, tags.bracket, tags.paren], color: "#24292f" },
      { tag: [tags.variableName, tags.name], color: "#1f2328" },
      { tag: [tags.definition(tags.variableName), tags.function(tags.variableName)], color: "#6639ba", fontWeight: "600" },
    ]),
    EDITOR_THEME = EditorView.theme({
      "&": {
        height: "100%",
        fontSize: "14px",
        backgroundColor: "#ffffff",
        color: "#1f2328",
      },
      ".cm-content": {
        fontFamily: "'JetBrains Mono', monospace",
        padding: "12px 0",
      },
      ".cm-gutters": {
        backgroundColor: "#f6f8fa",
        color: "#8c959f",
        borderRight: "1px solid #e1e4e8",
        fontFamily: "'JetBrains Mono', monospace",
      },
      ".cm-activeLineGutter": {
        backgroundColor: "#eaeef2",
        color: "#24292f",
      },
      ".cm-activeLine": {
        backgroundColor: "#f6f8fa",
      },
      ".cm-selectionMatch": {
        backgroundColor: "#ddf4ff",
      },
      "&.cm-focused .cm-cursor": {
        borderLeftColor: "#0969da",
        borderLeftWidth: "2px",
      },
      "&.cm-focused .cm-selectionBackground, .cm-selectionBackground, ::selection": {
        backgroundColor: "#b6e3ff !important",
      },
    }),
    statusSet = (key, kind, count = 0) => {
      status_key = key;
      status_kind = kind;
      status_count = count;
    },
    autoCheckCancel = () => {
      if (auto_check_timer) {
        clearTimeout(auto_check_timer);
        auto_check_timer = null;
      }
    },
    lineJump = (line_no) => {
      if (!cm_view || typeof line_no !== "number") return;
      const { doc } = cm_view.state,
        line_count = doc.lines,
        target = Math.max(1, Math.min(line_no, line_count)),
        line = doc.line(target);
      cm_view.dispatch({
        selection: { anchor: line.from },
        scrollIntoView: true,
      });
      cm_view.focus();
    },
    diagnosticsRender = (diag_li, explicit = false) => {
      if (!diag_li || diag_li.length === 0) {
        statusSet("no_errors", "ready", 0);
        if (explicit) {
          output_state = { kind: "diag_ok", text: "", is_error: false, diag_li: [] };
        }
        return;
      }

      statusSet("errors", "error", diag_li.length);
      output_state = { kind: "diag_err", text: "", is_error: true, diag_li };
    },
    typeCheckRun = async (quiet_if_clean = false) => {
      if (!cm_view || !is_ready) return;
      const source = cm_view.state.doc.toString();
      try {
        const diag_li = await engineCheck(source);
        diagnosticsRender(diag_li, !quiet_if_clean);
      } catch (err) {
        console.error("Check failed:", err);
        statusSet("error", "error", 0);
      }
    },
    autoCheckSchedule = () => {
      if (suppress_check) return;
      autoCheckCancel();
      auto_check_timer = setTimeout(() => {
        typeCheckRun(true);
      }, 450);
    },
    codeRun = async () => {
      autoCheckCancel();
      suppress_check = true;
      if (!cm_view || !is_ready) return;

      if (typeof window !== "undefined" && window.innerWidth <= 768) {
        editor_bottom_el?.scrollIntoView({ behavior: "smooth", block: "start" });
      }

      const source = cm_view.state.doc.toString();
      statusSet("running", "running", 0);
      output_state = { kind: "output", text: "", is_error: false, diag_li: [] };

      const res = await engineRun(source),
        { error, is_runtime_error, output } = res;
      if (error) {
        statusSet(is_runtime_error ? "runtime_error" : "error", "error", 0);
        output_state = { kind: "output", text: error, is_error: true, diag_li: [] };
      } else {
        statusSet("ready", "ready", 0);
        output_state = { kind: "output", text: output, is_error: false, diag_li: [] };
      }
      setTimeout(() => {
        suppress_check = false;
      }, 800);
    },
    editorClear = () => {
      if (!cm_view) return;
      cm_view.dispatch({
        changes: { from: 0, to: cm_view.state.doc.length, insert: "" },
      });
      output_state = { kind: "ready", text: "", is_error: false, diag_li: [] };
      statusSet("ready", "ready", 0);
      cm_view.focus();
    },
    exampleLoad = (key) => {
      const code = exampleCodeGet(key, i18n_state.current_lang);
      if (!code || !cm_view) return;
      selected_example = key;
      suppress_check = true;
      cm_view.dispatch({
        changes: { from: 0, to: cm_view.state.doc.length, insert: code },
      });
      output_state = { kind: "ready", text: "", is_error: false, diag_li: [] };
      setTimeout(() => {
        suppress_check = false;
        typeCheckRun(true);
      }, 100);
    },
    outputClick = (e) => {
      const target = e.target.closest(".diag-ln[data-line]");
      if (target) {
        e.preventDefault();
        const line = parseInt(target.getAttribute("data-line"), 10);
        lineJump(line);
      }
    };

  onMount(() => {
    const run_keymap = keymap.of([
        {
          key: "Mod-Enter",
          run: () => {
            codeRun();
            return true;
          },
        },
        {
          key: "Ctrl-Enter",
          run: () => {
            codeRun();
            return true;
          },
        },
      ]),
      state = EditorState.create({
        doc: exampleCodeGet(selected_example, i18n_state.current_lang),
        extensions: [
          basicSetup,
          run_keymap,
          EditorView.lineWrapping,
          StreamLanguage.define(lua),
          syntaxHighlighting(VIVID_LIGHT_HIGHLIGHT_STYLE),
          EDITOR_THEME,
          EditorView.updateListener.of((update) => {
            if (update.docChanged) autoCheckSchedule();
          }),
        ],
      });

    cm_view = new EditorView({
      state,
      parent: editor_container,
    });

    const select_el = document.getElementById("example-select"),
      selectChange = ({ target }) => exampleLoad(target.value);
    if (select_el) {
      select_el.addEventListener("change", selectChange);
    }

    statusSet("loading_wasm", "loading", 0);
    output_state = { kind: "loading", text: "", is_error: false, diag_li: [] };

    (async () => {
      try {
        await engineLoad();
        is_ready = true;
        statusSet("ready", "ready", 0);
        output_state = { kind: "ready", text: "", is_error: false, diag_li: [] };
        await typeCheckRun(true);
      } catch (err) {
        statusSet("wasm_failed", "error", 0);
        output_state = { kind: "wasm_failed", text: (err.message ?? String(err)), is_error: true, diag_li: [] };
      }
    })();

    return () => {
      if (auto_check_timer) clearTimeout(auto_check_timer);
      if (select_el) select_el.removeEventListener("change", selectChange);
      if (cm_view) cm_view.destroy();
    };
  });
</script>

<template lang="pug">
section#playground.playground
  .wrap
    .pg
      .pg-toolbar
        label.pg-examples
          span {t("pg.example")}
          select#example-select(value={selected_example} onchange={({ target }) => exampleLoad(target.value)})
            +each('EXAMPLE_KEY_LI as key')
              option(value={key}) {t("example." + key)}

        .pg-actions
          button#btn-run.btn.btn-primary(disabled={!is_ready} onclick={codeRun})
            span.btn-glyph ▶
            span {t("pg.run")}
          button#btn-check.btn.btn-secondary(disabled={!is_ready} onclick={() => typeCheckRun(false)})
            span.btn-glyph ✓
            span {t("pg.check")}
          button#btn-clear.btn.btn-ghost(onclick={editorClear}) {t("pg.clear")}

      .pg-grid
        .pg-pane.pg-editor-pane
          .pane-head
            span.label {t("pg.editor_tab")}
          .editor(@&editor_container)
          .editor-bottom-anchor(@&editor_bottom_el)

        .pg-pane.pg-output-pane
          .pane-head
            span.label {t("pg.diag_output")}
            span#status.status(class="status-" + status_kind) {status_display}
          <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_noninteractive_element_interactions -->
          pre#output.output(onclick={outputClick} aria-live="polite")
            +if('output_state.kind === "loading"')
              .out-info {t("pg.loading_engine")}
            +if('output_state.kind === "ready"')
              .out-ready {t("pg.ready_prompt")}
            +if('output_state.kind === "wasm_failed"')
              span.out-err {t("out.wasm_failed") + (output_state.text ?? "")}
            +if('output_state.kind === "diag_ok"')
              .diag-report
                .diag-head.is-ok
                  span.diag-mark ✓
                  |  {t("out.no_errors")}
            +if('output_state.kind === "diag_err"')
              .diag-report
                .diag-head.is-err
                  span.diag-mark ✗
                  |  {output_state.diag_li.length} {output_state.diag_li.length === 1 ? t("status.error") : t("status.errors")}
                +each('output_state.diag_li as d')
                  .diag-row
                    +if('d.line > 0')
                      a.diag-ln(href={"#L" + d.line} data-line={d.line}) L{d.line}
                    +if('!d.line')
                      span.diag-ln.diag-ln-none —
                    .diag-msg {typeof d === "string" ? d : (d.message ?? JSON.stringify(d))}
            +if('output_state.kind === "output"')
              span(class={output_state.is_error ? "out-err" : "out-ok"}) {output_state.text || t("out.no_output")}

      p.pg-note {@html t("pg.note")}
</template>

<style lang="stylus">
.playground
  padding 20px 0 48px
  background #ffffff

.pg
  background #ffffff
  border 1px solid #d0d7de
  border-radius 12px
  box-shadow 0 4px 20px rgba(140, 149, 159, 0.1)
  overflow hidden

.pg-toolbar
  display flex
  align-items center
  justify-content space-between
  padding 10px 16px
  background #f6f8fa
  border-bottom 1px solid #d0d7de
  flex-wrap wrap
  gap 12px

.pg-examples
  display flex
  align-items center
  gap 8px
  font-size 13px
  font-weight 600
  color #57606a

.pg-examples select
  padding 5px 10px
  border 1px solid #d0d7de
  border-radius 6px
  background #ffffff
  color #1f2328
  font-size 13px
  font-weight 500
  cursor pointer

.pg-actions
  display flex
  align-items center
  gap 8px

.btn-glyph
  font-size 11px

.pg-grid
  display grid
  grid-template-columns 1fr 1fr
  min-height 420px
  background #ffffff

@media (max-width 768px)
  .pg-grid
    grid-template-columns 1fr

.pg-pane
  display flex
  flex-direction column
  min-height 380px

.pg-editor-pane
  border-right 1px solid #d0d7de

@media (max-width 768px)
  .pg-editor-pane
    border-right none
    border-bottom 1px solid #d0d7de

.pane-head .status
  margin-left auto


.editor
  flex 1
  height 100%
  overflow auto

.editor-bottom-anchor
  display block
  height 0
  scroll-margin-top 70px

.output
  flex 1
  margin 0
  padding 14px 16px
  font-family 'JetBrains Mono', monospace
  font-size 13px
  line-height 1.55
  background #fafbfc
  color #1f2328
  overflow auto
  white-space pre-wrap
  word-break break-all

:global(.out-ok)
  color #1a7f37
  font-weight 500

:global(.out-err)
  color #cf222e
  font-weight 500

:global(.out-ready)
  color #57606a
  font-style italic

:global(.out-info)
  color #0969da
  font-weight 500

:global(.diag-report)
  display flex
  flex-direction column
  gap 8px

:global(.diag-head)
  font-weight 700
  font-size 13px
  display flex
  align-items center
  gap 6px

:global(.diag-head.is-ok)
  color #1a7f37

:global(.diag-head.is-err)
  color #cf222e

:global(.diag-row)
  display flex
  align-items flex-start
  gap 8px
  font-size 12px
  padding 4px 0
  border-bottom 1px dashed #eaeef2

:global(.diag-ln)
  background #eaeef2
  color #0969da
  padding 1px 5px
  border-radius 4px
  font-weight 600
  text-decoration none
  cursor pointer
  &:hover
    background #0969da
    color #ffffff

:global(.diag-msg)
  color #cf222e
  flex 1

.pg-note
  padding 12px 16px
  margin 0
  font-size 12px
  color #656d76
  background #f6f8fa
  border-top 1px solid #d0d7de
  line-height 1.5
</style>
