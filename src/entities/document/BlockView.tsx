import { CodeView, HtmlFrame, MermaidView } from "../../shared/ui";
import type { Block } from "./model";

export function BlockView({ block, split }: { block: Block; split?: boolean }) {
  switch (block.type) {
    case "diff":
      return <CodeView file={block.file} code={block.after} before={block.before} split={split} />;
    case "code":
      return <CodeView file={block.file} code={block.code} />;
    case "mermaid":
      return <MermaidView source={block.source} />;
    case "html":
      return <HtmlFrame html={block.html} title={block.title} />;
  }
}
