import { useState } from "react";
import { BlockView, type Document } from "../../entities/document";

/** With `file`, only that file's diff and code blocks. Block numbers stay those of the whole document. */
export function DocumentView({ doc, file }: { doc: Document; file?: string }) {
  const [split, setSplit] = useState(false);
  return (
    <main>
      <header>
        <h1>{doc.goal}</h1>
        <label>
          <input type="checkbox" checked={split} onChange={(e) => setSplit(e.target.checked)} /> diff 나란히 보기
        </label>
      </header>
      {doc.blocks.map(
        (b, i) =>
          (!file || ("file" in b && b.file === file)) && (
            <section key={i}>
              <h2>
                {i + 1}. {b.title}
              </h2>
              {"file" in b && <code>{b.file}</code>}
              <BlockView block={b} split={split} />
            </section>
          ),
      )}
    </main>
  );
}
