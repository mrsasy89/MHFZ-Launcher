import { store } from "./store";

// Map logical names → files in /public/audio/
const FILES = {
  hover:   "/audio/hover.mp3",
  select:  "/audio/select.mp3",
  confirm: "/audio/confirm.mp3",
  start:   "/audio/start.mp3",
};

// Quick helper to get current volume [0..1]
function currentVolume() {
  return ((store.settings?.sfxVolume ?? 70) / 100);
}

// Play by logical name
export function play(name) {
  if (!store.settings?.sfxEnabled) return;
  const src = FILES[name];
  if (!src) return;
  // New Audio each time so overlapping clicks don't cut off
  const a = new Audio(src);
  a.volume = currentVolume();
  a.play().catch(() => {});
}

// Convenience wrappers
export const playHover   = () => throttledHover();
export const playSelect  = () => play("select");
export const playConfirm = () => play("confirm");
export const playStart   = () => play("start");

// ── Hover throttling to avoid machine‑gun sounds ─────────────
let lastHoverTs = 0;
function throttledHover() {
  const now = performance.now();
  if (now - lastHoverTs < 60) return; // ~16fps min
  lastHoverTs = now;
  play("hover");
}

// ── Attach to elements easily (returns unbind fn) ────────────
export function bindSfx(el, opts = { hover: true, click: "select" }) {
  const onHover = opts.hover ? () => playHover() : null;
  const onClick = opts.click ? () => play(opts.click) : null;

  if (onHover) el.addEventListener("mouseenter", onHover);
  if (onClick) el.addEventListener("click", onClick);

  return () => {
    if (onHover) el.removeEventListener("mouseenter", onHover);
    if (onClick) el.removeEventListener("click", onClick);
  };
}