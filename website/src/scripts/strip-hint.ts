/**
 * A year strip is drawn to scale, so on a narrow screen it is wider than its
 * box and scrolls sideways. The hint under it is shown only while that is so.
 * Its box already has an accessible label that says it scrolls, so the hint is
 * hidden from assistive technology.
 */
const observer = new ResizeObserver((entries) => {
  for (const { target } of entries) {
    const hint = target.nextElementSibling;
    if (hint instanceof HTMLElement && hint.matches(".strip-hint")) hint.hidden = target.scrollWidth <= target.clientWidth;
  }
});

export function addScrollHint(scroller: HTMLElement): void {
  const hint = document.createElement("p");
  hint.className = "strip-hint";
  hint.setAttribute("aria-hidden", "true");
  hint.hidden = true;
  hint.textContent = "Scroll sideways to see the whole year.";
  scroller.after(hint);
  observer.observe(scroller);
}
