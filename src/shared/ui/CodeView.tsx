import { useEffect, useRef } from "react";
import { EditorView, minimalSetup } from "codemirror";
import { EditorState, StateEffect } from "@codemirror/state";
import { LanguageDescription } from "@codemirror/language";
import { languages } from "@codemirror/language-data";
import { MergeView, unifiedMergeView } from "@codemirror/merge";

const base = [minimalSetup, EditorState.readOnly.of(true)];
// highlightChanges off: its per-character marks draw as underlines all over the code; changed lines keep their background
const diff = { collapseUnchanged: { margin: 2, minSize: 4 }, highlightChanges: false };

/** Read-only code. With `before`, a diff with unchanged regions collapsed: inline, or side by side when `split`. */
export function CodeView(props: { file: string; code: string; before?: string; split?: boolean }) {
  const { file, code, before } = props;
  const split = before !== undefined && props.split;
  const ref = useRef<HTMLDivElement>(null);
  useEffect(() => {
    const parent = ref.current!;
    const merge = split
      ? new MergeView({ parent, a: { doc: before, extensions: base }, b: { doc: code, extensions: base }, ...diff })
      : undefined;
    const views = merge
      ? [merge.a, merge.b]
      : [
          new EditorView({
            parent,
            doc: code,
            extensions: [
              base,
              before === undefined ? [] : unifiedMergeView({ original: before, mergeControls: false, ...diff }),
            ],
          }),
        ];
    let alive = true;
    LanguageDescription.matchFilename(languages, file)
      ?.load()
      .then((lang) => alive && views.forEach((v) => v.dispatch({ effects: StateEffect.appendConfig.of(lang) })));
    return () => {
      alive = false;
      merge ? merge.destroy() : views[0].destroy();
    };
  }, [file, code, before, split]);
  return <div ref={ref} />;
}
