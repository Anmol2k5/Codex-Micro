import { useState, useRef } from "react";
import type { MouseEvent as ReactMouseEvent, TouchEvent as ReactTouchEvent } from "react";

interface HardwareKnobProps {
  level: string;
  options: string[];
  onChange: (level: string) => void;
  disabled?: boolean;
}

export function HardwareKnob({ level, options, onChange, disabled }: HardwareKnobProps) {
  const knobRef = useRef<HTMLDivElement>(null);
  const [isDragging, setIsDragging] = useState(false);

  const currentIndex = Math.max(0, options.indexOf(level));
  const totalOptions = Math.max(1, options.length);
  // Calculate angle between -120 deg and +120 deg
  const startAngle = -120;
  const endAngle = 120;
  const stepAngle = totalOptions > 1 ? (endAngle - startAngle) / (totalOptions - 1) : 0;
  const rotationDegrees = startAngle + currentIndex * stepAngle;

  const handleClick = () => {
    if (disabled || options.length === 0) return;
    const nextIndex = (currentIndex + 1) % options.length;
    onChange(options[nextIndex]);
  };

  const handlePointerDown = (e: ReactMouseEvent | ReactTouchEvent) => {
    if (disabled) return;
    setIsDragging(true);
    const startY = "touches" in e ? e.touches[0].clientY : e.clientY;

    const handlePointerMove = (moveEvt: MouseEvent | TouchEvent) => {
      const currentY = "touches" in moveEvt ? moveEvt.touches[0].clientY : moveEvt.clientY;
      const deltaY = startY - currentY;
      if (Math.abs(deltaY) > 15) {
        const direction = deltaY > 0 ? 1 : -1;
        const targetIndex = Math.min(
          options.length - 1,
          Math.max(0, currentIndex + direction),
        );
        if (targetIndex !== currentIndex && options[targetIndex]) {
          onChange(options[targetIndex]);
        }
      }
    };

    const handlePointerUp = () => {
      setIsDragging(false);
      window.removeEventListener("mousemove", handlePointerMove);
      window.removeEventListener("mouseup", handlePointerUp);
      window.removeEventListener("touchmove", handlePointerMove);
      window.removeEventListener("touchend", handlePointerUp);
    };

    window.addEventListener("mousemove", handlePointerMove);
    window.addEventListener("mouseup", handlePointerUp);
    window.addEventListener("touchmove", handlePointerMove);
    window.addEventListener("touchend", handlePointerUp);
  };

  return (
    <div
      ref={knobRef}
      className={`hardware-knob-container ${disabled ? "knob-disabled" : ""} ${isDragging ? "knob-dragging" : ""}`}
      onClick={handleClick}
      onMouseDown={handlePointerDown}
      onTouchStart={handlePointerDown}
      role="slider"
      aria-label="Reasoning level rotary dial"
      aria-valuenow={currentIndex}
      aria-valuemin={0}
      aria-valuemax={options.length - 1}
      aria-valuetext={level}
      title={`Reasoning dial: ${level} (Click or drag to change)`}
    >
      <div className="knob-outer-ring">
        <div
          className="knob-cap"
          style={{ transform: `rotate(${rotationDegrees}deg)` }}
        >
          <div className="knob-indicator-line" />
        </div>
      </div>
      <span className="knob-value-badge">{level || "N/A"}</span>
    </div>
  );
}
