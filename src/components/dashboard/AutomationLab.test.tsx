// @vitest-environment jsdom
import "@testing-library/jest-dom/vitest";
import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { AutomationLab } from "./AutomationLab";

const fixture = JSON.stringify({
  schemaVersion: 1,
  capturedAt: "2026-07-19T00:00:00.000Z",
  target: { processName: "ChatGPT.exe", processId: 42, windowTitle: "ChatGPT" },
  root: {
    name: "ChatGPT",
    automationId: "root",
    controlType: "ControlType.Window",
    className: "Chrome_WidgetWin_1",
    isEnabled: true,
    isOffscreen: false,
    patterns: [],
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
});

describe("AutomationLab", () => {
  it("loads a snapshot, filters nodes, and maps a selected element", async () => {
    render(<AutomationLab />);

    const file = new File([fixture], "capture.json", { type: "application/json" });
    fireEvent.change(screen.getByLabelText(/import UI Automation snapshot/i), {
      target: { files: [file] },
    });

    expect(await screen.findByText(/2 elements captured/i)).toBeInTheDocument();

    fireEvent.change(screen.getByLabelText(/search captured elements/i), {
      target: { value: "review" },
    });
    fireEvent.click(screen.getByRole("listitem", { name: /Review changes/i }));
    fireEvent.change(screen.getByLabelText(/semantic control/i), {
      target: { value: "reviewChanges" },
    });
    fireEvent.click(screen.getByRole("button", { name: /add selector mapping/i }));

    expect(screen.getByText(/1 mapping/i)).toBeInTheDocument();
  });
});
