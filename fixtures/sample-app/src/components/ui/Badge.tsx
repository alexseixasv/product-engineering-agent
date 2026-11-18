import type { HTMLAttributes } from "react";

type Props = HTMLAttributes<HTMLSpanElement> & {
  tone?: "neutral" | "success" | "warning" | "danger";
};

export function Badge({ tone = "neutral", className = "", ...props }: Props) {
  const tones = {
    neutral: "bg-muted text-muted-foreground",
    success: "bg-emerald-100 text-emerald-800",
    warning: "bg-amber-100 text-amber-900",
    danger: "bg-rose-100 text-rose-800",
  };
  return (
    <span
      className={`inline-flex rounded-md px-2 py-0.5 text-xs font-medium ${tones[tone]} ${className}`}
      {...props}
    />
  );
}
