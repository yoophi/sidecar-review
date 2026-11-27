import type { Document } from "../../entities/document";

type Entry = { rest: string[]; file: string; blocks: number };
type Pick = { current?: string; onSelect: (file?: string) => void };

/** One directory level: collapsible subdirectories first, then the files directly in it. */
function Tree({ entries, ...pick }: { entries: Entry[] } & Pick) {
  const dirs = new Map<string, Entry[]>();
  const files: Entry[] = [];
  for (const e of entries) {
    if (e.rest.length <= 1) files.push(e);
    else dirs.set(e.rest[0], [...(dirs.get(e.rest[0]) ?? []), { ...e, rest: e.rest.slice(1) }]);
  }
  return (
    <>
      {[...dirs].map(([name, children]) => (
        <details key={name} open>
          <summary>{name}</summary>
          <div>
            <Tree entries={children} {...pick} />
          </div>
        </details>
      ))}
      {files.map((e) => (
        <button key={e.file} aria-current={e.file === pick.current} onClick={() => pick.onSelect(e.file)}>
          {e.rest[0] ?? e.file} <span title="블록 수">{e.blocks}</span>
        </button>
      ))}
    </>
  );
}

/** The second sidebar column: the selected session's goal, then a directory tree of the files its diff and code
 * blocks touch. No `current` means every block is shown. */
export function FileList(props: { doc: Document } & Pick) {
  const counts = new Map<string, number>();
  for (const b of props.doc.blocks) if ("file" in b) counts.set(b.file, (counts.get(b.file) ?? 0) + 1);
  return (
    <nav aria-label={`${props.doc.session} 파일`}>
      <p>{props.doc.goal}</p>
      <div className="tree">
        <button aria-current={!props.current} onClick={() => props.onSelect()}>
          전체 <span title="블록 수">{props.doc.blocks.length}</span>
        </button>
        <Tree
          entries={[...counts].map(([file, blocks]) => ({ rest: file.split("/").filter(Boolean), file, blocks }))}
          current={props.current}
          onSelect={props.onSelect}
        />
      </div>
    </nav>
  );
}
