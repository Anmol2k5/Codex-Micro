import type { ThreadSummary } from "../../types/codex";

export function pageCount(total: number, pageSize: number): number {
  if (pageSize <= 0) return 0;
  return Math.max(1, Math.ceil(total / pageSize));
}

export function clampPageIndex(total: number, pageIndex: number, pageSize: number): number {
  return Math.min(Math.max(0, pageIndex), pageCount(total, pageSize) - 1);
}

export function pageThreads(
  threads: ThreadSummary[],
  pageIndex: number,
  pageSize: number,
): ThreadSummary[] {
  if (threads.length === 0 || pageSize <= 0) return [];
  const safePage = clampPageIndex(threads.length, pageIndex, pageSize);
  const start = safePage * pageSize;
  return threads.slice(start, start + pageSize);
}

export function selectedPageIndex(
  threads: ThreadSummary[],
  selectedThreadId: string | null,
  pageSize: number,
): number {
  if (!selectedThreadId || pageSize <= 0) return 0;
  const index = threads.findIndex((thread) => thread.id === selectedThreadId);
  return index < 0 ? 0 : Math.floor(index / pageSize);
}
