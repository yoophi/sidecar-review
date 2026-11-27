/** A draggable border that resizes the element just before it. Its min/max width in CSS bound the drag. */
export function Divider() {
  return (
    <div
      role="separator"
      aria-orientation="vertical"
      aria-label="너비 조절"
      tabIndex={0}
      onPointerDown={(e) => {
        e.preventDefault(); // no text selection while dragging
        e.currentTarget.focus();
        e.currentTarget.setPointerCapture(e.pointerId);
      }}
      onPointerMove={(e) => {
        if (!e.currentTarget.hasPointerCapture(e.pointerId)) return;
        const prev = e.currentTarget.previousElementSibling as HTMLElement;
        prev.style.width = `${e.clientX - prev.getBoundingClientRect().left}px`;
      }}
      onKeyDown={(e) => {
        const prev = e.currentTarget.previousElementSibling as HTMLElement;
        const step = e.key === "ArrowLeft" ? -16 : e.key === "ArrowRight" ? 16 : 0;
        if (step) prev.style.width = `${prev.offsetWidth + step}px`;
      }}
    />
  );
}
