import type { ReactNode } from "react";

interface HardwareKeyProps {
  label?: string;
  icon?: ReactNode;
  variant?: "translucent" | "dashed" | "solid" | "pill" | "thought";
  tone?: "normal" | "positive" | "danger";
  active?: boolean;
  disabled?: boolean;
  busy?: boolean;
  onClick?: () => void;
  onDoubleClick?: () => void;
  title?: string;
  badge?: string | number;
}

export function HardwareKey({
  label,
  icon,
  variant = "solid",
  tone = "normal",
  active = false,
  disabled = false,
  busy = false,
  onClick,
  onDoubleClick,
  title,
  badge,
}: HardwareKeyProps) {
  return (
    <button
      type="button"
      className={`hardware-key key-variant-${variant} key-tone-${tone} ${active ? "key-active" : ""} ${busy ? "key-busy" : ""}`}
      disabled={disabled}
      onClick={onClick}
      onDoubleClick={onDoubleClick}
      title={title || label}
      aria-label={label || title || "Control deck key"}
    >
      <div className="key-cap">
        <div className="key-surface">
          {badge !== undefined && badge !== null ? (
            <span className="key-badge">{badge}</span>
          ) : null}
          {icon ? <span className="key-icon">{icon}</span> : null}
          {label && variant === "pill" ? <span className="key-label">{label}</span> : null}
        </div>
      </div>
    </button>
  );
}
