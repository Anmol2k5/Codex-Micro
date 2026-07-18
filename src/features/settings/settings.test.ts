import { describe, expect, it } from "vitest";
import { DEFAULT_SETTINGS, parseSettings, serializeSettings } from "./settings";

describe("parseSettings", () => {
  it("returns safe defaults when storage is empty", () => {
    expect(parseSettings(null)).toEqual(DEFAULT_SETTINGS);
  });

  it("migrates partial legacy settings and clamps unsafe visual values", () => {
    const parsed = parseSettings(
      JSON.stringify({
        schemaVersion: 0,
        followActiveThread: false,
        controllerScale: 4,
        controllerOpacity: 0.1,
      }),
    );

    expect(parsed.schemaVersion).toBe(1);
    expect(parsed.followActiveThread).toBe(false);
    expect(parsed.controllerScale).toBe(1.4);
    expect(parsed.controllerOpacity).toBe(0.55);
    expect(parsed.confirmDiscardChanges).toBe(true);
  });

  it("recovers from malformed JSON", () => {
    expect(parseSettings("not-json")).toEqual(DEFAULT_SETTINGS);
  });
});

describe("serializeSettings", () => {
  it("round-trips validated settings", () => {
    const encoded = serializeSettings({ ...DEFAULT_SETTINGS, alwaysOnTop: false });
    expect(parseSettings(encoded).alwaysOnTop).toBe(false);
  });
});
