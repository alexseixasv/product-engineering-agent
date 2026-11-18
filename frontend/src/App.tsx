import { useEffect, useState, type FormEvent } from "react";
import { createRun, decide, finalize, listRuns } from "./api";
import type { RunRecord } from "./types";
import { ProposalReview } from "./ProposalReview";
import { AnalyticsPrototype } from "./AnalyticsPrototype";

const DEMO_REQUEST =
  "We need an analytics dashboard where managers can monitor team productivity, overdue tasks, and project performance. It should support filters, charts, and CSV export. Can we deliver this next sprint?";

export default function App() {
  const isPrototype = window.location.pathname.startsWith("/prototype/analytics");
  const [request, setRequest] = useState(DEMO_REQUEST);
  const [actor, setActor] = useState("pm@example.com");
  const [runs, setRuns] = useState<RunRecord[]>([]);
  const [active, setActive] = useState<RunRecord | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  if (isPrototype) {
    return <AnalyticsPrototype />;
  }

  async function refresh() {
    try {
      const data = await listRuns();
      setRuns(data);
    } catch {
      // API may be offline during static review
    }
  }

  useEffect(() => {
    void refresh();
  }, []);

  async function onSubmit(e: FormEvent) {
    e.preventDefault();
    setBusy(true);
    setError(null);
    try {
      const run = await createRun(request);
      setActive(run);
      await refresh();
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setBusy(false);
    }
  }

  async function onDecision(decision: "approve" | "reject") {
    if (!active) return;
    setBusy(true);
    setError(null);
    try {
      const run = await decide(active.id, decision, actor, undefined);
      setActive(run);
      await refresh();
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setBusy(false);
    }
  }

  async function onFinalize() {
    if (!active) return;
    setBusy(true);
    setError(null);
    try {
      const run = await finalize(active.id);
      setActive(run);
      await refresh();
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setBusy(false);
    }
  }

  return (
    <div className="mx-auto max-w-6xl px-4 py-8">
      <header className="mb-8 border-b border-line pb-6">
        <p className="font-display text-4xl font-bold tracking-tight text-ink">
          Product Engineering Agent
        </p>
        <p className="mt-2 max-w-2xl text-base text-ink/70">
          Submit a product request, inspect evidenced findings, then approve or
          reject before design artifacts are generated.
        </p>
      </header>

      <div className="grid gap-8 lg:grid-cols-[minmax(0,1fr)_280px]">
        <section>
          <form onSubmit={onSubmit} className="space-y-3">
            <label className="block text-sm font-medium" htmlFor="request">
              Product manager request
            </label>
            <textarea
              id="request"
              className="min-h-36 w-full rounded-md border border-line bg-white/80 p-3 text-sm shadow-sm outline-none ring-accent focus:ring-2"
              value={request}
              onChange={(e) => setRequest(e.target.value)}
            />
            <div className="flex flex-wrap items-end gap-3">
              <div>
                <label className="block text-sm font-medium" htmlFor="actor">
                  Approver actor
                </label>
                <input
                  id="actor"
                  className="mt-1 rounded-md border border-line bg-white/80 px-3 py-2 text-sm"
                  value={actor}
                  onChange={(e) => setActor(e.target.value)}
                />
              </div>
              <button
                type="submit"
                disabled={busy}
                className="rounded-md bg-ink px-4 py-2 text-sm font-medium text-paper disabled:opacity-50"
              >
                {busy ? "Working…" : "Analyze codebase"}
              </button>
            </div>
          </form>

          {error && (
            <p className="mt-4 rounded-md border border-rose-300 bg-rose-50 px-3 py-2 text-sm text-rose-900">
              {error}
            </p>
          )}

          {active && (
            <div className="mt-8">
              <div className="mb-4 flex flex-wrap items-center gap-3">
                <span className="rounded-md bg-ink/5 px-2 py-1 text-xs font-medium uppercase tracking-wide">
                  State: {active.state}
                </span>
                {active.state === "awaiting_human_approval" && (
                  <>
                    <button
                      type="button"
                      disabled={busy}
                      onClick={() => void onDecision("approve")}
                      className="rounded-md bg-accent px-3 py-2 text-sm font-medium text-white"
                    >
                      Approve proposal
                    </button>
                    <button
                      type="button"
                      disabled={busy}
                      onClick={() => void onDecision("reject")}
                      className="rounded-md border border-line bg-white px-3 py-2 text-sm"
                    >
                      Reject
                    </button>
                  </>
                )}
                {active.state === "approved" && (
                  <button
                    type="button"
                    disabled={busy}
                    onClick={() => void onFinalize()}
                    className="rounded-md bg-ink px-3 py-2 text-sm font-medium text-paper"
                  >
                    Generate design.md + snapshot
                  </button>
                )}
              </div>
              {active.proposal ? (
                <ProposalReview proposal={active.proposal} approval={active.approval} />
              ) : (
                <p className="text-sm text-ink/70">No proposal payload yet.</p>
              )}
            </div>
          )}
        </section>

        <aside className="h-fit rounded-md border border-line bg-white/70 p-4">
          <h2 className="font-display text-lg font-semibold">Recent runs</h2>
          <ul className="mt-3 space-y-2 text-sm">
            {runs.length === 0 && (
              <li className="text-ink/60">No runs yet (start API to populate).</li>
            )}
            {runs.map((r) => (
              <li key={r.id}>
                <button
                  type="button"
                  className="w-full rounded-md border border-transparent px-2 py-1 text-left hover:border-line hover:bg-paper"
                  onClick={() => setActive(r)}
                >
                  <span className="block truncate font-medium">{r.request}</span>
                  <span className="text-xs text-ink/60">{r.state}</span>
                </button>
              </li>
            ))}
          </ul>
        </aside>
      </div>
    </div>
  );
}
