/** Agent-supplied HTML. The sandbox (scripts, but no same-origin) keeps it away from the app and Tauri IPC. */
// shortcut: fixed height in CSS; have the frame postMessage its height when charts get clipped
export function HtmlFrame({ html, title }: { html: string; title: string }) {
  return <iframe sandbox="allow-scripts" srcDoc={html} title={title || "HTML"} />;
}
