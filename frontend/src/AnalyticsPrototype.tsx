export function AnalyticsPrototype() {
  return (
    <div className="mx-auto max-w-5xl px-4 py-8" data-testid="analytics-prototype">
      <header className="mb-6 flex items-end justify-between gap-4 border-b border-line pb-4">
        <div>
          <h1 className="font-display text-3xl font-bold">Analytics dashboard</h1>
          <p className="text-sm text-ink/70">
            Proposed UI reusing Pulseboard Card, Table, Select, Badge, and Button patterns.
          </p>
        </div>
        <button type="button" className="rounded-md bg-accent px-3 py-2 text-sm font-medium text-white">
          Export CSV
        </button>
      </header>

      <div className="mb-4 flex gap-3">
        <select className="rounded-md border border-line bg-white px-2 py-2 text-sm" aria-label="Project filter">
          <option>All projects</option>
          <option>Mobile Onboarding</option>
        </select>
        <select className="rounded-md border border-line bg-white px-2 py-2 text-sm" aria-label="Status filter">
          <option>All statuses</option>
          <option>Overdue</option>
        </select>
      </div>

      <div className="grid gap-4 md:grid-cols-3">
        {[
          ["Team productivity", "78%", "Completed vs estimated hours"],
          ["Overdue tasks", "2", "Across active projects"],
          ["Project performance", "2 / 3", "Projects on track"],
        ].map(([title, value, hint]) => (
          <div key={title} className="rounded-md border border-line bg-white/80 p-4">
            <p className="text-sm text-ink/60">{title}</p>
            <p className="mt-2 font-display text-3xl font-semibold">{value}</p>
            <p className="mt-1 text-xs text-ink/50">{hint}</p>
            <div className="mt-4 h-16 rounded bg-gradient-to-r from-ink/10 via-accent/30 to-ink/10" />
          </div>
        ))}
      </div>

      <div className="mt-6 overflow-hidden rounded-md border border-line bg-white/80">
        <table className="w-full text-sm">
          <thead className="border-b border-line text-left text-ink/60">
            <tr>
              <th className="px-3 py-2">Task</th>
              <th className="px-3 py-2">Project</th>
              <th className="px-3 py-2">Due</th>
              <th className="px-3 py-2">Status</th>
            </tr>
          </thead>
          <tbody>
            <tr className="border-b border-line">
              <td className="px-3 py-2">QA onboarding emails</td>
              <td className="px-3 py-2">Mobile Onboarding</td>
              <td className="px-3 py-2">2026-11-05</td>
              <td className="px-3 py-2">
                <span className="rounded bg-rose-100 px-2 py-0.5 text-xs text-rose-800">overdue</span>
              </td>
            </tr>
            <tr>
              <td className="px-3 py-2">Wire welcome checklist</td>
              <td className="px-3 py-2">Mobile Onboarding</td>
              <td className="px-3 py-2">2026-11-10</td>
              <td className="px-3 py-2">
                <span className="rounded bg-amber-100 px-2 py-0.5 text-xs text-amber-900">in progress</span>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  );
}
