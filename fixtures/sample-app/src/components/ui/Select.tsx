import type { SelectHTMLAttributes } from "react";

export function Select({
  className = "",
  children,
  ...props
}: SelectHTMLAttributes<HTMLSelectElement>) {
  return (
    <select
      className={`h-9 rounded-md border border-border bg-background px-2 text-sm ${className}`}
      {...props}
    >
      {children}
    </select>
  );
}
