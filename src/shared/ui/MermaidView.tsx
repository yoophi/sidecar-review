import { useEffect, useRef, useState } from "react";
import mermaid from "mermaid";

mermaid.initialize({ startOnLoad: false });
let count = 0;

export function MermaidView({ source }: { source: string }) {
  const id = useRef(`mermaid-${count++}`).current;
  const [out, setOut] = useState<{ svg?: string; error?: string }>({});
  useEffect(() => {
    mermaid
      .render(id, source)
      .then(({ svg }) => setOut({ svg }))
      .catch((e) => setOut({ error: String(e) }));
  }, [id, source]);
  if (out.error) return <pre role="alert">{out.error}</pre>;
  // mermaid sanitizes its SVG (securityLevel "strict" is the default)
  return <div className="mermaid" dangerouslySetInnerHTML={{ __html: out.svg ?? "" }} />;
}
