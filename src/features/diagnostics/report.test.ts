import { describe, expect, it } from "vitest";
import { buildDiagnosticReport, redactDiagnosticText } from "./report";

describe("redactDiagnosticText", () => {
  it("redacts Windows user paths and bearer-like secrets", () => {
    const text = "C:\\Users\\Anmol\\project token=sk-secret Authorization: Bearer abc.def";
    const redacted = redactDiagnosticText(text);
    expect(redacted).not.toContain("Anmol");
    expect(redacted).not.toContain("sk-secret");
    expect(redacted).not.toContain("abc.def");
    expect(redacted).toContain("C:\\Users\\<redacted>");
  });
});

describe("buildDiagnosticReport", () => {
  it("excludes conversation content and keeps structured action metadata", () => {
    const report = buildDiagnosticReport({
      version: "0.1.0",
      connectionState: "degraded",
      capabilities: { canFocusApp: true },
      actionHistory: [
        {
          action: "focusApp",
          outcome: "succeeded",
          diagnosticCode: "WINDOW_FOCUSED",
          elapsedMs: 12,
          userMessage: "Focused C:\\Users\\Anmol\\ChatGPT",
        },
      ],
    });

    expect(report).toContain("WINDOW_FOCUSED");
    expect(report).not.toContain("Anmol");
  });
});
