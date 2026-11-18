import type { EngineeringProposal, RunRecord } from "./types";

type Props = {
  proposal: EngineeringProposal;
  approval?: RunRecord["approval"];
};

export function ProposalReview({ proposal, approval }: Props) {
  return (
    <div className="space-y-6" data-testid="proposal-review">
      {approval && (
        <section className="rounded-md border border-line bg-white/80 p-4">
          <h3 className="font-display text-xl font-semibold">Approval record</h3>
          <p className="mt-2 text-sm">
            <strong>{approval.decision}</strong> by {approval.actor} at{" "}
            {approval.decided_at}
          </p>
        </section>
      )}

      <section className="rounded-md border border-line bg-white/80 p-4">
        <h3 className="font-display text-xl font-semibold">Feasibility</h3>
        <p className="mt-2 text-sm leading-relaxed">{proposal.feasibility.summary}</p>
        <ul className="mt-3 list-disc space-y-1 pl-5 text-sm">
          {proposal.feasibility.risks.map((r) => (
            <li key={r}>{r}</li>
          ))}
        </ul>
      </section>

      <section className="rounded-md border border-line bg-white/80 p-4">
        <h3 className="font-display text-xl font-semibold">Findings</h3>
        <ul className="mt-3 space-y-3">
          {proposal.findings.map((f) => (
            <li key={f.id} className="border-b border-line/70 pb-3 last:border-0">
              <div className="flex flex-wrap items-center gap-2">
                <span className="text-sm font-medium">{f.id}</span>
                <span className="rounded bg-ink/5 px-1.5 py-0.5 text-xs uppercase">
                  {f.confidence}
                </span>
                <span className="text-xs text-ink/60">{f.category}</span>
              </div>
              <p className="mt-1 text-sm">{f.summary}</p>
              {f.evidence?.map((e, idx) => (
                <pre
                  key={`${f.id}-${idx}`}
                  className="mt-2 overflow-x-auto rounded bg-ink/[0.04] p-2 text-xs"
                >
                  {e.path}:{e.start_line}-{e.end_line}
                  {"\n"}
                  {e.excerpt}
                </pre>
              ))}
            </li>
          ))}
        </ul>
      </section>

      <section className="rounded-md border border-line bg-white/80 p-4">
        <h3 className="font-display text-xl font-semibold">Requirements</h3>
        <div className="mt-3 grid gap-4 md:grid-cols-2">
          <div>
            <h4 className="text-sm font-semibold">Functional</h4>
            <ul className="mt-2 space-y-2 text-sm">
              {proposal.requirements.functional.map((r) => (
                <li key={r.id}>
                  <p>
                    <span className="font-medium">{r.id}</span> — {r.statement}
                  </p>
                  <ul className="ml-4 list-disc text-ink/70">
                    {r.acceptance_criteria.map((c) => (
                      <li key={c}>{c}</li>
                    ))}
                  </ul>
                </li>
              ))}
            </ul>
          </div>
          <div>
            <h4 className="text-sm font-semibold">Non-functional</h4>
            <ul className="mt-2 space-y-2 text-sm">
              {proposal.requirements.non_functional.map((r) => (
                <li key={r.id}>
                  <p>
                    <span className="font-medium">{r.id}</span> — {r.statement}
                  </p>
                </li>
              ))}
            </ul>
          </div>
        </div>
      </section>

      <section className="rounded-md border border-line bg-white/80 p-4">
        <h3 className="font-display text-xl font-semibold">Jira-ready backlog</h3>
        <ul className="mt-3 space-y-2 text-sm">
          {proposal.backlog.map((t) => (
            <li key={t.key} className="rounded border border-line/80 p-3">
              <p className="font-medium">
                {t.key} · {t.type} · {t.title}
              </p>
              <p className="text-ink/70">Traces to: {t.requirement_ids.join(", ")}</p>
            </li>
          ))}
        </ul>
      </section>

      <section className="rounded-md border border-line bg-white/80 p-4">
        <h3 className="font-display text-xl font-semibold">Open questions</h3>
        <ul className="mt-2 list-disc space-y-1 pl-5 text-sm">
          {proposal.open_questions.map((q) => (
            <li key={q}>{q}</li>
          ))}
        </ul>
        {(proposal.design_md_path || proposal.snapshot_path) && (
          <div className="mt-4 text-sm">
            <p>design.md: {proposal.design_md_path}</p>
            <p>snapshot: {proposal.snapshot_path}</p>
          </div>
        )}
      </section>
    </div>
  );
}
