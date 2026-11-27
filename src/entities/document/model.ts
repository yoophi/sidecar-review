import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export type Block =
  | { type: "diff"; title: string; file: string; before: string; after: string; mermaid: string }
  | { type: "code"; title: string; file: string; code: string; mermaid: string }
  | { type: "mermaid"; title: string; source: string }
  | { type: "html"; title: string; html: string };

/** One coding-agent session. The most recently updated document comes last.
 * `created_at` and `updated_at` are Unix ms; 0 in documents saved before the app recorded them.
 * `origin` is what the agent said about itself; a field it left out is "". */
export type Document = {
  session: string;
  goal: string;
  blocks: Block[];
  created_at: number;
  updated_at: number;
  origin: { agent: string; cwd: string; agent_session_id: string };
};

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
