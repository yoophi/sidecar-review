import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export type Block =
  | { type: "diff"; title: string; file: string; before: string; after: string }
  | { type: "code"; title: string; file: string; code: string }
  | { type: "mermaid"; title: string; source: string }
  | { type: "html"; title: string; html: string };

/** One coding-agent session. The most recently updated document comes last. */
export type Document = { session: string; goal: string; blocks: Block[] };

export function useDocuments() {
  const [docs, setDocs] = useState<Document[]>([]);
  useEffect(() => {
    const unlisten = listen<Document[]>("documents", (e) => setDocs(e.payload));
    invoke<Document[]>("documents").then(setDocs);
    return () => {
      unlisten.then((f) => f());
    };
  }, []);
  return docs;
}
