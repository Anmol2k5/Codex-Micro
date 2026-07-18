import type { ReactNode } from "react";

interface ActionKeyProps {
  label: string;
  icon: ReactNode;
  disabled?: boolean;
  busy?: boolean;
  title?: string;
  onClick: () => void;
  tone?: "default" | "positive" | "danger";
}

export function ActionKey({
  label,
  icon,
  disabled = false,
  busy = false,
  title,
  onClick,
  tone = "default",
}: ActionKeyProps) {
  return (
    <button
      className={`action-key action-key-${tone}`}
      type="button"
      disabled={disabled || busy}
      title={title}
      onClick={onClick}
      aria-busy={busy}
    >
      <span className="action-key-icon" aria-hidden="true">
        {icon}
      </span>
      <span>{busy ? "Working…" : label}</span>
    </button>
  );
}
