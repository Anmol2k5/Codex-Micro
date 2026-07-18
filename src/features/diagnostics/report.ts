interface DiagnosticActionEntry {
  action: string;
  outcome: string;
  diagnosticCode: string;
  elapsedMs: number;
  userMessage: string;
}

interface DiagnosticReportInput {
  version: string;
  connectionState: string;
  capabilities: object;
  actionHistory: DiagnosticActionEntry[];
}

export function redactDiagnosticText(text: string): string {
  return text
    .replace(/C:\\Users\\[^\\\s]+/gi, "C:\\Users\\<redacted>")
    .replace(/\bsk-[A-Za-z0-9._-]+\b/g, "<redacted-secret>")
    .replace(/\btoken\s*=\s*[^\s]+/gi, "token=<redacted>")
    .replace(/Authorization:\s*Bearer\s+[^\s]+/gi, "Authorization: Bearer <redacted>");
}

export function buildDiagnosticReport(input: DiagnosticReportInput): string {
  const safe = {
    generatedAt: new Date().toISOString(),
    version: input.version,
    connectionState: input.connectionState,
    capabilities: input.capabilities,
    actionHistory: input.actionHistory.slice(-50).map((entry) => ({
      ...entry,
      userMessage: redactDiagnosticText(entry.userMessage),
    })),
  };

  return redactDiagnosticText(JSON.stringify(safe, null, 2));
}
