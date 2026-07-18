import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";

const scriptPath = new URL("../scripts/windows-capture-uia.ps1", import.meta.url);

describe("windows UI Automation capture script", () => {
  it("captures semantic accessibility metadata without relying on screen coordinates", () => {
    const script = readFileSync(scriptPath, "utf8");
    expect(script).toContain("AutomationElement.FromHandle");
    expect(script).toContain("ControlViewWalker");
    expect(script).toContain("GetSupportedPatterns");
    expect(script).toContain("schemaVersion");
    expect(script).toContain("ConvertTo-Json");
    expect(script).not.toContain("BoundingRectangleProperty");
  });
});
