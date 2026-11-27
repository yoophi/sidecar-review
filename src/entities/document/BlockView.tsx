import { CodeView, HtmlFrame, MermaidView } from "../../shared/ui";
import type { Block } from "./model";

export function BlockView({ block }: { block: Block }) {
  switch (block.type) {
    case "diff":
      return <CodeView file={block.file} code={block.after} before={block.before} />;
    case "code":
      return <CodeView file={block.file} code={block.code} />;
    case "mermaid":
      return <MermaidView source={block.source} />;
    case "html":
      return <HtmlFrame html={block.html} title={block.title} />;
  }
}
