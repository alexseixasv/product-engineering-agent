import type { RunRecord } from "./types";

const base = "";

export async function createRun(request: string): Promise<RunRecord> {
  const res = await fetch(`${base}/api/runs`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ request }),
  });
  if (!res.ok) {
    const body = await res.json().catch(() => ({}));
    throw new Error(body.error ?? `Failed to create run (${res.status})`);
  }
  return res.json();
}

export async function getRun(id: string): Promise<RunRecord> {
  const res = await fetch(`${base}/api/runs/${id}`);
  if (!res.ok) throw new Error(`Run not found`);
  return res.json();
}

export async function listRuns(): Promise<RunRecord[]> {
  const res = await fetch(`${base}/api/runs`);
  if (!res.ok) throw new Error(`Failed to list runs`);
  return res.json();
}

export async function decide(
  id: string,
  decision: "approve" | "reject",
  actor: string,
  comment?: string,
): Promise<RunRecord> {
  const res = await fetch(`${base}/api/runs/${id}/decision`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ decision, actor, comment }),
  });
  if (!res.ok) {
    const body = await res.json().catch(() => ({}));
    throw new Error(body.error ?? `Decision failed`);
  }
  return res.json();
}

export async function finalize(id: string): Promise<RunRecord> {
  const res = await fetch(`${base}/api/runs/${id}/finalize`, {
    method: "POST",
  });
  if (!res.ok) {
    const body = await res.json().catch(() => ({}));
    throw new Error(body.error ?? `Finalize failed`);
  }
  return res.json();
}
