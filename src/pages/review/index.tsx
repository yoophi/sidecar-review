import { useState } from "react";
import { useDocuments } from "../../entities/document";
import { Divider } from "../../shared/ui";
import { DocumentView } from "../../widgets/document-view";
import { FileList } from "../../widgets/file-list";
import { SessionList } from "../../widgets/session-list";

export function ReviewPage() {
  const docs = useDocuments();
  const [selected, setSelected] = useState<string>();
  const [picked, setPicked] = useState<{ session: string; file?: string }>();
  // nothing picked: follow whichever session the agent touched last
  const current = docs.find((d) => d.session === selected) ?? docs[docs.length - 1];

  if (!current)
    return (
      <main className="empty">
        <p>코딩 에이전트의 리뷰를 기다리는 중입니다.</p>
        <code>claude mcp add --transport http sidecar http://127.0.0.1:7437/mcp</code>
      </main>
    );

  // a file filter belongs to the session it was picked in
  const file = picked?.session === current.session ? picked.file : undefined;

  return (
    <>
      <SessionList docs={docs} current={current} onSelect={setSelected} />
      <Divider />
      <FileList doc={current} current={file} onSelect={(file) => setPicked({ session: current.session, file })} />
      <Divider />
      <DocumentView doc={current} file={file} />
    </>
  );
}
