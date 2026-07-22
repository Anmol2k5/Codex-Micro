import { describe, expect, it } from "vitest";
import { addSelectorMapping, buildSelectorCandidate, createSelectorProfile } from "./profile";
import type { FlatUiaNode } from "./snapshot";

const node: FlatUiaNode = {
  path: "0.2.1",
  depth: 2,
  name: "Review changes",
  automationId: "review-button",
  controlType: "ControlType.Button",
  className: "ButtonClass",
  isEnabled: true,
  isOffscreen: false,
  patterns: ["InvokePatternIdentifiers.Pattern"],
};

describe("buildSelectorCandidate", () => {
  it("prefers stable automation metadata without storing screen coordinates", () => {
    expect(buildSelectorCandidate(node)).toEqual({
      automationId: "review-button",
      names: ["Review changes"],
      controlType: "ControlType.Button",
      className: "ButtonClass",
      requiredPatterns: ["InvokePatternIdentifiers.Pattern"],
    });
  });
});

describe("selector profiles", () => {
  it("adds semantic mappings without duplicating the same selector", () => {
    const profile = createSelectorProfile({
      profileId: "local-chatgpt",
      processNames: ["ChatGPT.exe"],
      windowTitleHints: ["ChatGPT"],
      appVersions: [],
    });
    const once = addSelectorMapping(profile, "reviewChanges", buildSelectorCandidate(node));
    const twice = addSelectorMapping(once, "reviewChanges", buildSelectorCandidate(node));

    expect(twice.selectors.reviewChanges).toHaveLength(1);
    expect(twice.selectors.reviewChanges?.[0].automationId).toBe("review-button");
  });
});
