import { useEffect, useState } from "react";
import mermaid from "mermaid";

mermaid.initialize({ startOnLoad: false });
let count = 0;

export function MermaidView({ source }: { source: string }) {
  const [out, setOut] = useState<{ svg?: string; error?: string }>({});
  useEffect(() => {
    if (!source.trim()) return setOut({});
    // a new id for every render: mermaid first deletes the element that has the id, which would be our drawn chart
    mermaid
      .render(`mermaid-${count++}`, source)
      .then(({ svg }) => setOut({ svg }))
      .catch((e) => setOut({ error: String(e) }));
  }, [source]);
  if (out.error) return <pre role="alert">{out.error}</pre>;
  // nothing to draw (blank source, or not rendered yet): no empty box
  if (!out.svg) return null;
  // mermaid sanitizes its SVG (securityLevel "strict" is the default)
  return <div className="mermaid" dangerouslySetInnerHTML={{ __html: out.svg }} />;
}
