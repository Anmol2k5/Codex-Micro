import { describe, expect, it } from "vitest";
import { pageCount, pageThreads, selectedPageIndex } from "./paging";
import type { ThreadSummary } from "../../types/codex";

const threads: ThreadSummary[] = Array.from({ length: 9 }, (_, index) => ({
  id: `t-${index + 1}`,
  title: `Thread ${index + 1}`,
  status: "idle",
  isActive: index === 0,
}));

describe("thread paging", () => {
  it("returns four threads per controller page", () => {
    expect(pageThreads(threads, 1, 4).map((thread) => thread.id)).toEqual([
      "t-5",
      "t-6",
      "t-7",
      "t-8",
    ]);
  });

  it("clamps an out-of-range page", () => {
    expect(pageThreads(threads, 99, 4).map((thread) => thread.id)).toEqual(["t-9"]);
  });

  it("computes page count and selected page", () => {
    expect(pageCount(threads.length, 4)).toBe(3);
    expect(selectedPageIndex(threads, "t-9", 4)).toBe(2);
  });
});
