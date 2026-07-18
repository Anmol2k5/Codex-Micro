export interface UiaSnapshotTarget {
  processName: string;
  processId: number;
  windowTitle: string;
  processVersion?: string | null;
}

export interface UiaSnapshotNode {
  name: string;
  automationId: string;
  controlType: string;
  className: string;
  isEnabled: boolean;
  isOffscreen: boolean;
  patterns: string[];
  children: UiaSnapshotNode[];
}

export interface UiaSnapshot {
  schemaVersion: 1;
  capturedAt: string;
  target: UiaSnapshotTarget;
  root: UiaSnapshotNode;
}

export interface FlatUiaNode extends Omit<UiaSnapshotNode, "children"> {
  path: string;
  depth: number;
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function isNode(value: unknown): value is UiaSnapshotNode {
  if (!isRecord(value)) return false;
  return (
    typeof value.name === "string" &&
    typeof value.automationId === "string" &&
    typeof value.controlType === "string" &&
    typeof value.className === "string" &&
    typeof value.isEnabled === "boolean" &&
    typeof value.isOffscreen === "boolean" &&
    Array.isArray(value.patterns) &&
    value.patterns.every((pattern) => typeof pattern === "string") &&
    Array.isArray(value.children) &&
    value.children.every(isNode)
  );
}

export function parseUiaSnapshot(raw: string): UiaSnapshot {
  let value: unknown;
  try {
    value = JSON.parse(raw);
  } catch {
    throw new Error("Invalid UI Automation snapshot: file is not valid JSON.");
  }

  if (!isRecord(value) || value.schemaVersion !== 1 || !isRecord(value.target) || !isNode(value.root)) {
    throw new Error("Invalid UI Automation snapshot: unsupported or malformed structure.");
  }

  const target = value.target;
  if (
    typeof value.capturedAt !== "string" ||
    typeof target.processName !== "string" ||
    typeof target.processId !== "number" ||
    typeof target.windowTitle !== "string"
  ) {
    throw new Error("Invalid UI Automation snapshot: target metadata is incomplete.");
  }

  return value as unknown as UiaSnapshot;
}

export function flattenUiaSnapshot(snapshot: UiaSnapshot): FlatUiaNode[] {
  const output: FlatUiaNode[] = [];

  const visit = (node: UiaSnapshotNode, path: string, depth: number) => {
    const { children, ...flat } = node;
    output.push({ ...flat, path, depth });
    children.forEach((child, index) => visit(child, `${path}.${index}`, depth + 1));
  };

  visit(snapshot.root, "0", 0);
  return output;
}

export function searchUiaNodes(nodes: FlatUiaNode[], query: string): FlatUiaNode[] {
  const normalized = query.trim().toLowerCase();
  if (!normalized) return nodes;

  return nodes.filter((node) => {
    const haystack = [
      node.name,
      node.automationId,
      node.controlType,
      node.className,
      ...node.patterns,
    ]
      .join("\n")
      .toLowerCase();
    return haystack.includes(normalized);
  });
}
