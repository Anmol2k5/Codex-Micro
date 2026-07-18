export interface AppSettings {
  schemaVersion: 1;
  alwaysOnTop: boolean;
  followActiveThread: boolean;
  confirmDiscardChanges: boolean;
  sendAfterDictation: boolean;
  controllerScale: number;
  controllerOpacity: number;
  reducedMotion: boolean;
  onboardingComplete: boolean;
}

export const SETTINGS_STORAGE_KEY = "microdeck.settings.v1";

export const DEFAULT_SETTINGS: AppSettings = Object.freeze({
  schemaVersion: 1,
  alwaysOnTop: true,
  followActiveThread: true,
  confirmDiscardChanges: true,
  sendAfterDictation: false,
  controllerScale: 1,
  controllerOpacity: 1,
  reducedMotion: false,
  onboardingComplete: false,
});

function booleanOr(value: unknown, fallback: boolean): boolean {
  return typeof value === "boolean" ? value : fallback;
}

function numberOr(value: unknown, fallback: number): number {
  return typeof value === "number" && Number.isFinite(value) ? value : fallback;
}

function clamp(value: number, min: number, max: number): number {
  return Math.min(max, Math.max(min, value));
}

export function parseSettings(raw: string | null): AppSettings {
  if (!raw) return { ...DEFAULT_SETTINGS };

  try {
    const value = JSON.parse(raw) as Record<string, unknown>;
    return {
      schemaVersion: 1,
      alwaysOnTop: booleanOr(value.alwaysOnTop, DEFAULT_SETTINGS.alwaysOnTop),
      followActiveThread: booleanOr(
        value.followActiveThread,
        DEFAULT_SETTINGS.followActiveThread,
      ),
      confirmDiscardChanges: booleanOr(
        value.confirmDiscardChanges,
        DEFAULT_SETTINGS.confirmDiscardChanges,
      ),
      sendAfterDictation: booleanOr(
        value.sendAfterDictation,
        DEFAULT_SETTINGS.sendAfterDictation,
      ),
      controllerScale: clamp(
        numberOr(value.controllerScale, DEFAULT_SETTINGS.controllerScale),
        0.8,
        1.4,
      ),
      controllerOpacity: clamp(
        numberOr(value.controllerOpacity, DEFAULT_SETTINGS.controllerOpacity),
        0.55,
        1,
      ),
      reducedMotion: booleanOr(value.reducedMotion, DEFAULT_SETTINGS.reducedMotion),
      onboardingComplete: booleanOr(
        value.onboardingComplete,
        DEFAULT_SETTINGS.onboardingComplete,
      ),
    };
  } catch {
    return { ...DEFAULT_SETTINGS };
  }
}

export function serializeSettings(settings: AppSettings): string {
  return JSON.stringify(parseSettings(JSON.stringify(settings)));
}

export function loadSettings(): AppSettings {
  if (typeof window === "undefined") return { ...DEFAULT_SETTINGS };
  return parseSettings(window.localStorage.getItem(SETTINGS_STORAGE_KEY));
}

export function saveSettings(settings: AppSettings): void {
  if (typeof window === "undefined") return;
  window.localStorage.setItem(SETTINGS_STORAGE_KEY, serializeSettings(settings));
}

export function clearSettings(): void {
  if (typeof window === "undefined") return;
  window.localStorage.removeItem(SETTINGS_STORAGE_KEY);
}
