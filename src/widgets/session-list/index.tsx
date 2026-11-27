import { useEffect, useState } from "react";
import type { Document } from "../../entities/document";

const relative = new Intl.RelativeTimeFormat("ko");
// how long a pushed-to session stays marked NEW when nobody clicks it
const NEW_FOR_MS = 10 * 60_000;

/** "20분 전". Under a minute is "방금 전". */
function ago(ms: number, now: number) {
  const s = (now - ms) / 1000;
  for (const [unit, size] of [["day", 86400], ["hour", 3600], ["minute", 60]] as const)
    if (s >= size) return relative.format(-Math.floor(s / size), unit);
  return "방금 전";
}

export function SessionList(props: { docs: Document[]; current: Document; onSelect: (session: string) => void }) {
  // re-render so "n분 전" keeps moving while no document changes
  const [now, setNow] = useState(Date.now);
  useEffect(() => {
    const timer = setInterval(() => setNow(Date.now()), 30_000);
    return () => clearInterval(timer);
  }, []);
  // when each session was last clicked; in memory only, so a restart re-marks whatever is still recent
  const [seen, setSeen] = useState<Record<string, number>>({});
  const isNew = (d: Document) =>
    d.updated_at > (seen[d.session] ?? 0) && Math.max(now, Date.now()) - d.updated_at < NEW_FOR_MS;
  // one group per agent conversation, latest activity first; reviews that named no conversation go last
  const groups = new Map<string, Document[]>();
  for (const d of [...props.docs].reverse()) {
    const id = d.origin.agent_session_id;
    groups.set(id, [...(groups.get(id) ?? []), d]);
  }
  const unnamed = groups.get("");
  if (groups.delete("")) groups.set("", unnamed!);
  return (
    <nav aria-label="세션">
      {[...groups].map(([id, docs]) => (
        <details key={id} open>
          <summary>{id ? [docs[0].origin.agent, id].filter(Boolean).join(" · ") : "세션 정보 없음"}</summary>
          {docs.map((d) => (
            <button
              key={d.session}
              aria-current={d === props.current}
              data-new={isNew(d) || undefined}
              onClick={() => {
                setSeen({ ...seen, [d.session]: Date.now() });
                props.onSelect(d.session);
              }}
            >
              {/* floated first so the name wraps around it; the panel shows on hover or keyboard focus, in CSS */}
              <span className="info">
                <i aria-hidden>ⓘ</i>
                <span role="tooltip">
                  <span>에이전트: {d.origin.agent || "기록 없음"}</span>
                  <span>작업 디렉터리: {d.origin.cwd || "기록 없음"}</span>
                  <span>세션 ID: {d.origin.agent_session_id || "기록 없음"}</span>
                </span>
              </span>
              <strong>{d.session}</strong>
              {isNew(d) && <mark>NEW</mark>}
              <span>{d.goal}</span>
              {/* the sv-SE locale formats as yyyy-mm-dd HH:MM:SS, in local time; 0: the session predates the timestamps */}
              <span>생성 {d.created_at ? new Date(d.created_at).toLocaleString("sv-SE") : "기록 없음"}</span>
              <span>업데이트 {d.updated_at ? ago(d.updated_at, Math.max(now, Date.now())) : "기록 없음"}</span>
            </button>
          ))}
        </details>
      ))}
    </nav>
  );
}
