import { useEffect, useRef } from "react";
import { EditorView, minimalSetup } from "codemirror";
import { EditorState, StateEffect } from "@codemirror/state";
import { LanguageDescription } from "@codemirror/language";
import { languages } from "@codemirror/language-data";
import { unifiedMergeView } from "@codemirror/merge";

/** Read-only code. With `before`, a unified diff with unchanged regions collapsed. */
export function CodeView({ file, code, before }: { file: string; code: string; before?: string }) {
  const ref = useRef<HTMLDivElement>(null);
  useEffect(() => {
    const view = new EditorView({
      parent: ref.current!,
      doc: code,
      extensions: [
        minimalSetup,
        EditorState.readOnly.of(true),
        before === undefined
          ? []
          : unifiedMergeView({ original: before, mergeControls: false, collapseUnchanged: { margin: 2, minSize: 4 } }),
      ],
    });
    let alive = true;
    LanguageDescription.matchFilename(languages, file)
      ?.load()
      .then((lang) => alive && view.dispatch({ effects: StateEffect.appendConfig.of(lang) }));
    return () => {
      alive = false;
      view.destroy();
    };
  }, [file, code, before]);
  return <div ref={ref} />;
}
