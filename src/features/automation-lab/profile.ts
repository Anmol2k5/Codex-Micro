import type { FlatUiaNode } from "./snapshot";

export type SemanticSelectorKey =
  | "codexModeIndicator"
  | "threadList"
  | "threadRow"
  | "activeThread"
  | "newThread"
  | "reviewChanges"
  | "approve"
  | "reject"
  | "discardChanges"
  | "promptComposer"
  | "sendButton"
  | "reasoningControl"
  | "reasoningOption"
  | "approvalPanel";

export interface ElementSelectorCandidate {
  automationId?: string;
  names?: string[];
  controlType?: string;
  className?: string;
  requiredPatterns?: string[];
  ancestorHints?: string[];
  descendantHints?: string[];
}

export interface SelectorProfile {
  schemaVersion: 1;
  profileId: string;
  description: string;
  target: {
    processNames: string[];
    windowTitleHints: string[];
    appVersions: string[];
  };
  selectors: Partial<Record<SemanticSelectorKey, ElementSelectorCandidate[]>>;
}

export const SEMANTIC_SELECTOR_KEYS: SemanticSelectorKey[] = [
  "codexModeIndicator",
  "threadList",
  "threadRow",
  "activeThread",
  "newThread",
  "reviewChanges",
  "approve",
  "reject",
  "discardChanges",
  "promptComposer",
  "sendButton",
  "reasoningControl",
  "reasoningOption",
  "approvalPanel",
];

export function buildSelectorCandidate(node: FlatUiaNode): ElementSelectorCandidate {
  return {
    ...(node.automationId ? { automationId: node.automationId } : {}),
    ...(node.name ? { names: [node.name] } : {}),
    ...(node.controlType ? { controlType: node.controlType } : {}),
    ...(node.className ? { className: node.className } : {}),
    ...(node.patterns.length ? { requiredPatterns: [...node.patterns] } : {}),
  };
}

export function createSelectorProfile(input: {
  profileId: string;
  processNames: string[];
  windowTitleHints: string[];
  appVersions: string[];
}): SelectorProfile {
  return {
    schemaVersion: 1,
    profileId: input.profileId,
    description: "Locally captured MicroDeck selector profile. Review before committing or sharing.",
    target: {
      processNames: input.processNames,
      windowTitleHints: input.windowTitleHints,
      appVersions: input.appVersions,
    },
    selectors: {},
  };
}

export function addSelectorMapping(
  profile: SelectorProfile,
  key: SemanticSelectorKey,
  candidate: ElementSelectorCandidate,
): SelectorProfile {
  const existing = profile.selectors[key] ?? [];
  const encoded = JSON.stringify(candidate);
  if (existing.some((item) => JSON.stringify(item) === encoded)) return profile;

  return {
    ...profile,
    selectors: {
      ...profile.selectors,
      [key]: [...existing, candidate],
    },
  };
}
