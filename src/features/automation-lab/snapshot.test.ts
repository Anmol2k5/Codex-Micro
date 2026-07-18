import { describe, expect, it } from "vitest";
import {
  flattenUiaSnapshot,
  parseUiaSnapshot,
  searchUiaNodes,
} from "./snapshot";

const fixture = {
  schemaVersion: 1,
  capturedAt: "2026-07-19T00:00:00.000Z",
  target: {
    processName: "ChatGPT.exe",
    processId: 42,
    windowTitle: "ChatGPT",
  },
  root: {
    name: "ChatGPT",
    automationId: "root",
    controlType: "ControlType.Window",
    className: "Chrome_WidgetWin_1",
    isEnabled: true,
    isOffscreen: false,
    patterns: ["WindowPatternIdentifiers.Pattern"],
    children: [
      {
        name: "Review changes",
        automationId: "review-button",
        controlType: "ControlType.Button",
        className: "",
        isEnabled: true,
        isOffscreen: false,
        patterns: ["InvokePatternIdentifiers.Pattern"],
        children: [],
      },
    ],
  },
};

describe("parseUiaSnapshot", () => {
  it("accepts a valid capture and rejects malformed shapes", () => {
    expect(parseUiaSnapshot(JSON.stringify(fixture)).target.processName).toBe("ChatGPT.exe");
    expect(() => parseUiaSnapshot('{"schemaVersion":1,"root":null}')).toThrow(
      /invalid UI Automation snapshot/i,
    );
  });
});

describe("flattenUiaSnapshot", () => {
  it("assigns stable structural paths and depth", () => {
    const nodes = flattenUiaSnapshot(parseUiaSnapshot(JSON.stringify(fixture)));
    expect(nodes.map((node) => [node.path, node.depth, node.name])).toEqual([
      ["0", 0, "ChatGPT"],
      ["0.0", 1, "Review changes"],
    ]);
  });
});

describe("searchUiaNodes", () => {
  it("searches name, automation id, class, control type, and patterns case-insensitively", () => {
    const nodes = flattenUiaSnapshot(parseUiaSnapshot(JSON.stringify(fixture)));
    expect(searchUiaNodes(nodes, "review").map((node) => node.path)).toEqual(["0.0"]);
    expect(searchUiaNodes(nodes, "invokePattern").map((node) => node.path)).toEqual(["0.0"]);
    expect(searchUiaNodes(nodes, "chrome_widget").map((node) => node.path)).toEqual(["0"]);
  });
});
