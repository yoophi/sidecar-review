import { BlockView, type Document } from "../../entities/document";

export function DocumentView({ doc }: { doc: Document }) {
  return (
    <main>
      <h1>{doc.goal}</h1>
      {doc.blocks.map((b, i) => (
        <section key={i}>
          <h2>
            {i + 1}. {b.title}
          </h2>
          {"file" in b && <code>{b.file}</code>}
          <BlockView block={b} />
        </section>
      ))}
    </main>
  );
}
