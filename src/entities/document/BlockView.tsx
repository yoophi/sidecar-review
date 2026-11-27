import { CodeView, HtmlFrame, MermaidView } from "../../shared/ui";
import type { Block } from "./model";

/** The diagram of a diff or code block is drawn above its code, so the two stay together under the file filter. */
export function BlockView({ block, split }: { block: Block; split?: boolean }) {
  switch (block.type) {
    case "diff":
      return (
        <>
          {block.mermaid && <MermaidView source={block.mermaid} />}
          <CodeView file={block.file} code={block.after} before={block.before} split={split} />
        </>
      );
    case "code":
      return (
        <>
          {block.mermaid && <MermaidView source={block.mermaid} />}
          <CodeView file={block.file} code={block.code} />
        </>
      );
    case "mermaid":
      return <MermaidView source={block.source} />;
    case "html":
      return <HtmlFrame html={block.html} title={block.title} />;
  }
}
