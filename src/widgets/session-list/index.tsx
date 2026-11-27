import type { Document } from "../../entities/document";

export function SessionList(props: { docs: Document[]; current: Document; onSelect: (session: string) => void }) {
  return (
    <nav aria-label="세션">
      {[...props.docs].reverse().map((d) => (
        <button key={d.session} aria-current={d === props.current} onClick={() => props.onSelect(d.session)}>
          <strong>{d.session}</strong>
          <span>{d.goal}</span>
        </button>
      ))}
    </nav>
  );
}
