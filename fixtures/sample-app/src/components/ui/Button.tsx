import type { ButtonHTMLAttributes } from "react";

type Props = ButtonHTMLAttributes<HTMLButtonElement> & {
  variant?: "default" | "outline" | "ghost";
};

export function Button({
  variant = "default",
  className = "",
  ...props
}: Props) {
  const base =
    "inline-flex items-center justify-center rounded-md px-3 py-2 text-sm font-medium transition disabled:opacity-50";
  const variants = {
    default: "bg-primary text-primary-foreground hover:opacity-90",
    outline: "border border-border bg-background hover:bg-accent",
    ghost: "hover:bg-accent",
  };
  return (
    <button className={`${base} ${variants[variant]} ${className}`} {...props} />
  );
}
