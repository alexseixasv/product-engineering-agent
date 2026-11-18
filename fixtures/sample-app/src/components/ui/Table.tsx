import type { HTMLAttributes, TableHTMLAttributes } from "react";

export function Table({
  className = "",
  ...props
}: TableHTMLAttributes<HTMLTableElement>) {
  return (
    <table className={`w-full caption-bottom text-sm ${className}`} {...props} />
  );
}

export function THead(props: HTMLAttributes<HTMLTableSectionElement>) {
  return <thead className="border-b border-border text-left" {...props} />;
}

export function TBody(props: HTMLAttributes<HTMLTableSectionElement>) {
  return <tbody {...props} />;
}

export function TR(props: HTMLAttributes<HTMLTableRowElement>) {
  return <tr className="border-b border-border last:border-0" {...props} />;
}

export function TH({
  className = "",
  ...props
}: HTMLAttributes<HTMLTableCellElement>) {
  return (
    <th className={`px-3 py-2 font-medium text-muted-foreground ${className}`} {...props} />
  );
}

export function TD({
  className = "",
  ...props
}: HTMLAttributes<HTMLTableCellElement>) {
  return <td className={`px-3 py-2 ${className}`} {...props} />;
}
