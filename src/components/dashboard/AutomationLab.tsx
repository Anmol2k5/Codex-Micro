import { useMemo, useState, type ChangeEvent } from "react";
import { Download, FileJson, FlaskConical, Plus, Search } from "lucide-react";
import {
  addSelectorMapping,
  buildSelectorCandidate,
  createSelectorProfile,
  SEMANTIC_SELECTOR_KEYS,
  type SelectorProfile,
  type SemanticSelectorKey,
} from "../../features/automation-lab/profile";
import {
  flattenUiaSnapshot,
  parseUiaSnapshot,
  searchUiaNodes,
  type FlatUiaNode,
  type UiaSnapshot,
} from "../../features/automation-lab/snapshot";

function downloadJson(filename: string, value: unknown): void {
  const blob = new Blob([JSON.stringify(value, null, 2)], { type: "application/json" });
  const url = URL.createObjectURL(blob);
  const anchor = document.createElement("a");
  anchor.href = url;
  anchor.download = filename;
  anchor.click();
  URL.revokeObjectURL(url);
}

function nodeLabel(node: FlatUiaNode): string {
  return node.name || node.automationId || node.controlType || node.path;
}

async function readTextFile(file: File): Promise<string> {
  if (typeof file.text === "function") return file.text();

  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = () => resolve(typeof reader.result === "string" ? reader.result : "");
    reader.onerror = () => reject(reader.error ?? new Error("Unable to read snapshot file."));
    reader.readAsText(file);
  });
}

export function AutomationLab() {
  const [snapshot, setSnapshot] = useState<UiaSnapshot | null>(null);
  const [nodes, setNodes] = useState<FlatUiaNode[]>([]);
  const [query, setQuery] = useState("");
  const [selectedPath, setSelectedPath] = useState<string | null>(null);
  const [semanticKey, setSemanticKey] = useState<SemanticSelectorKey>("reviewChanges");
  const [profile, setProfile] = useState<SelectorProfile | null>(null);
  const [error, setError] = useState<string | null>(null);

  const visibleNodes = useMemo(() => searchUiaNodes(nodes, query).slice(0, 100), [nodes, query]);
  const selectedNode = nodes.find((node) => node.path === selectedPath) ?? null;
  const mappingCount = profile
    ? Object.values(profile.selectors).reduce((sum, entries) => sum + (entries?.length ?? 0), 0)
    : 0;

  const importSnapshot = async (event: ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (!file) return;

    try {
      const parsed = parseUiaSnapshot(await readTextFile(file));
      const flattened = flattenUiaSnapshot(parsed);
      setSnapshot(parsed);
      setNodes(flattened);
      setSelectedPath(null);
      setQuery("");
      setProfile(
        createSelectorProfile({
          profileId: `local-${parsed.target.processName.replace(/\.exe$/i, "").toLowerCase()}`,
          processName: parsed.target.processName,
          windowTitle: parsed.target.windowTitle,
        }),
      );
      setError(null);
    } catch (importError) {
      setSnapshot(null);
      setNodes([]);
      setProfile(null);
      setSelectedPath(null);
      setError(importError instanceof Error ? importError.message : String(importError));
    } finally {
      event.target.value = "";
    }
  };

  const addMapping = () => {
    if (!profile || !selectedNode) return;
    setProfile(addSelectorMapping(profile, semanticKey, buildSelectorCandidate(selectedNode)));
  };

  return (
    <article className="panel automation-lab-panel">
      <div className="panel-heading automation-lab-heading">
        <div>
          <p className="eyebrow">AUTOMATION LAB</p>
          <h2>Inspect real Codex accessibility data</h2>
        </div>
        <label className="small-button file-button">
          <FileJson size={15} /> Import snapshot
          <input
            type="file"
            accept="application/json,.json"
            aria-label="Import UI Automation snapshot"
            onChange={(event) => void importSnapshot(event)}
          />
        </label>
      </div>

      <p className="automation-lab-copy">
        Run <code>scripts/windows-capture-uia.ps1</code> on your Windows PC while Codex is open,
        then import the generated JSON here. MicroDeck never guesses selectors from screenshots or
        hard-coded coordinates.
      </p>

      {error ? <div className="error-banner">{error}</div> : null}

      {!snapshot ? (
        <div className="automation-lab-empty">
          <FlaskConical size={22} />
          <div>
            <strong>No accessibility snapshot loaded</strong>
            <span>Capture the live ChatGPT/Codex UI tree on Windows, then import it here.</span>
          </div>
        </div>
      ) : (
        <div className="automation-lab-workspace">
          <div className="snapshot-summary">
            <strong>{nodes.length} elements captured</strong>
            <span>{snapshot.target.processName} · {snapshot.target.windowTitle}</span>
            <span>{new Date(snapshot.capturedAt).toLocaleString()}</span>
          </div>

          <label className="automation-search">
            <Search size={15} aria-hidden="true" />
            <input
              aria-label="Search captured elements"
              value={query}
              onChange={(event) => setQuery(event.target.value)}
              placeholder="Search name, AutomationId, control type, class, pattern…"
            />
          </label>

          <div className="automation-lab-columns">
            <div className="uia-node-list" role="list" aria-label="Captured UI Automation elements">
              {visibleNodes.length ? visibleNodes.map((node) => (
                <button
                  type="button"
                  role="listitem"
                  key={node.path}
                  aria-label={`Select ${nodeLabel(node)}`}
                  className={`uia-node-row ${selectedPath === node.path ? "uia-node-row-selected" : ""}`}
                  onClick={() => setSelectedPath(node.path)}
                >
                  <span className="uia-node-depth" style={{ paddingLeft: `${Math.min(node.depth, 8) * 10}px` }}>
                    {nodeLabel(node)}
                  </span>
                  <code>{node.automationId || "no AutomationId"}</code>
                  <small>{node.controlType}</small>
                </button>
              )) : <p className="empty-copy">No captured elements match that search.</p>}
            </div>

            <div className="selector-builder">
              <div>
                <p className="eyebrow">SELECTED ELEMENT</p>
                {selectedNode ? (
                  <dl className="selector-details">
                    <div><dt>Name</dt><dd>{selectedNode.name || "—"}</dd></div>
                    <div><dt>AutomationId</dt><dd>{selectedNode.automationId || "—"}</dd></div>
                    <div><dt>Control type</dt><dd>{selectedNode.controlType || "—"}</dd></div>
                    <div><dt>Class</dt><dd>{selectedNode.className || "—"}</dd></div>
                    <div><dt>Patterns</dt><dd>{selectedNode.patterns.join(", ") || "—"}</dd></div>
                    <div><dt>Path</dt><dd>{selectedNode.path}</dd></div>
                  </dl>
                ) : <p className="empty-copy">Select an element to inspect its stable accessibility metadata.</p>}
              </div>

              <label className="selector-control">
                <span>Semantic control</span>
                <select
                  aria-label="Semantic control"
                  value={semanticKey}
                  onChange={(event) => setSemanticKey(event.target.value as SemanticSelectorKey)}
                >
                  {SEMANTIC_SELECTOR_KEYS.map((key) => <option key={key} value={key}>{key}</option>)}
                </select>
              </label>

              <button
                type="button"
                className="primary-button"
                disabled={!selectedNode || !profile}
                onClick={addMapping}
              >
                <Plus size={16} /> Add selector mapping
              </button>

              <div className="profile-summary">
                <strong>{mappingCount} {mappingCount === 1 ? "mapping" : "mappings"}</strong>
                <span>Mappings are only selector candidates until verified by a real action and observable confirmation.</span>
              </div>

              <button
                type="button"
                className="secondary-button"
                disabled={!profile || mappingCount === 0}
                onClick={() => profile && downloadJson(`${profile.profileId}.json`, profile)}
              >
                <Download size={15} /> Export selector profile
              </button>
            </div>
          </div>
        </div>
      )}
    </article>
  );
}
