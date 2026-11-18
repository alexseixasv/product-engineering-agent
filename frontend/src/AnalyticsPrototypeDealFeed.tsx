/** Deal Feed Density (mydealz-inspired) visual proposal for the analytics feature. */
export function AnalyticsPrototypeDealFeed() {
  return (
    <div
      className="min-h-screen bg-deal-canvas font-deal text-deal-ink"
      data-testid="analytics-prototype-deal-feed"
    >
      <div className="border-b border-deal-hairline bg-deal-surface">
        <div className="mx-auto flex max-w-5xl flex-wrap items-center gap-3 px-4 py-3">
          <p className="text-[14px] font-bold tracking-tight">Pulseboard Analytics</p>
          <div className="flex min-w-[200px] flex-1 items-center gap-2 rounded-full border border-deal-hairline bg-deal-surface px-4 py-2 text-[16px] font-medium text-deal-body">
            <span aria-hidden>⌕</span>
            <span>Search metrics, projects…</span>
          </div>
          <button
            type="button"
            className="rounded-full bg-deal-primary px-3.5 py-2 text-[14px] font-bold text-white"
          >
            Export CSV
          </button>
        </div>
      </div>

      <div className="mx-auto grid max-w-5xl gap-3 px-4 py-4 lg:grid-cols-[minmax(0,1fr)_280px]">
        <main className="space-y-3">
          <header className="rounded-lg border border-deal-hairline bg-deal-surface p-4">
            <div className="flex flex-wrap items-start justify-between gap-3">
              <div>
                <h1 className="text-[24px] font-bold leading-tight">Analytics dashboard</h1>
                <p className="mt-1 text-[14px] font-medium text-deal-body">
                  Deal Feed Density — scannable cards, brand-green actions, hot/cold status.
                </p>
              </div>
              <span className="rounded-md bg-deal-canvas px-2 py-1 text-[12px] font-medium text-deal-body">
                Posted just now · proposal
              </span>
            </div>

            <div className="mt-4 flex flex-wrap gap-2">
              <select
                aria-label="Project filter"
                className="rounded-full border border-deal-hairline bg-deal-surface px-3 py-2 text-[14px] font-bold"
              >
                <option>All projects</option>
                <option>Mobile Onboarding</option>
              </select>
              <select
                aria-label="Status filter"
                className="rounded-full border border-deal-hairline bg-deal-surface px-3 py-2 text-[14px] font-bold"
              >
                <option>All statuses</option>
                <option>Overdue</option>
              </select>
              <span className="inline-flex items-center rounded-sm bg-deal-discount px-2 py-1 text-[14px] font-bold text-deal-green-strong">
                − filters applied
              </span>
            </div>
          </header>

          {[
            {
              title: "Team productivity",
              value: "78%",
              hint: "Completed vs estimated hours",
              temp: "+42°",
              rising: true,
            },
            {
              title: "Overdue tasks",
              value: "2",
              hint: "Across active projects",
              temp: "-8°",
              rising: false,
            },
            {
              title: "Project performance",
              value: "2 / 3",
              hint: "Projects on track",
              temp: "+19°",
              rising: true,
            },
          ].map((card) => (
            <article
              key={card.title}
              className="rounded-lg border border-deal-hairline bg-deal-surface p-4 shadow-[0_0_0_0_transparent] hover:shadow-[0_4px_12px_rgba(11,22,44,0.09)]"
            >
              <div className="flex flex-wrap items-start justify-between gap-3">
                <div>
                  <h2 className="text-[20px] font-bold leading-snug">{card.title}</h2>
                  <p className="mt-1 text-[12px] font-medium text-deal-body">{card.hint}</p>
                </div>
                <TemperatureControl reading={card.temp} hot={card.rising} />
              </div>
              <div className="mt-3 flex flex-wrap items-end gap-3">
                <p className="text-[24px] font-bold text-deal-primary">{card.value}</p>
                {card.rising ? (
                  <span className="rounded-sm bg-deal-discount px-2 py-1 text-[14px] font-bold text-deal-green-strong">
                    rising
                  </span>
                ) : (
                  <span className="rounded-sm bg-deal-warning-surface px-2 py-1 text-[14px] font-bold text-deal-warning-text">
                    needs attention
                  </span>
                )}
                <span className="text-[12px] font-medium text-deal-muted line-through">
                  prior baseline
                </span>
              </div>
              <div className="mt-4 h-14 rounded-lg bg-gradient-to-r from-deal-cold/20 via-deal-primary/25 to-deal-hot/20" />
              <div className="mt-4 flex flex-wrap items-center gap-3 border-t border-deal-hairline pt-3">
                <button
                  type="button"
                  className="rounded-full bg-deal-primary px-3.5 py-2 text-[14px] font-bold text-white"
                >
                  Open detail
                </button>
                <span className="text-[12px] font-medium text-deal-body">12 comments</span>
                <span className="text-[12px] font-medium text-deal-body">Share</span>
              </div>
            </article>
          ))}

          <section className="overflow-hidden rounded-lg border border-deal-hairline bg-deal-surface">
            <div className="flex items-center justify-between border-b border-deal-hairline px-4 py-3">
              <h2 className="text-[20px] font-bold">Task firehose</h2>
              <span className="rounded-full border border-deal-hairline px-3 py-1 text-[14px] font-bold text-deal-premium">
                premium view
              </span>
            </div>
            <table className="w-full text-[14px]">
              <thead className="border-b border-deal-hairline text-left text-deal-body">
                <tr>
                  <th className="px-4 py-2 font-bold">Task</th>
                  <th className="px-4 py-2 font-bold">Project</th>
                  <th className="px-4 py-2 font-bold">Due</th>
                  <th className="px-4 py-2 font-bold">Temp</th>
                </tr>
              </thead>
              <tbody>
                <tr className="border-b border-deal-hairline">
                  <td className="px-4 py-3 font-bold">QA onboarding emails</td>
                  <td className="px-4 py-3 text-deal-body">Mobile Onboarding</td>
                  <td className="px-4 py-3 text-deal-body">2026-11-05</td>
                  <td className="px-4 py-3">
                    <TemperatureControl reading="-18°" hot={false} />
                  </td>
                </tr>
                <tr>
                  <td className="px-4 py-3 font-bold">Wire welcome checklist</td>
                  <td className="px-4 py-3 text-deal-body">Mobile Onboarding</td>
                  <td className="px-4 py-3 text-deal-body">2026-11-10</td>
                  <td className="px-4 py-3">
                    <TemperatureControl reading="+11°" hot />
                  </td>
                </tr>
              </tbody>
            </table>
          </section>
        </main>

        <aside className="space-y-3">
          <div className="rounded-lg border border-deal-hairline bg-deal-surface p-4">
            <h2 className="text-[20px] font-bold">Deal Alarm</h2>
            <p className="mt-2 text-[14px] font-medium text-deal-body">
              Get notified when overdue heat rises above threshold.
            </p>
            <button
              type="button"
              className="mt-4 w-full rounded-full bg-deal-primary px-3.5 py-2 text-[14px] font-bold text-white"
            >
              Create alert
            </button>
          </div>
          <div className="rounded-lg border border-deal-hairline bg-deal-surface p-4">
            <h2 className="text-[20px] font-bold">Design notes</h2>
            <ul className="mt-2 list-disc space-y-1 pl-4 text-[12px] font-medium text-deal-body">
              <li>Primary green only on prices & CTAs</li>
              <li>Cold blue / hot red temperature pairing intact</li>
              <li>Full-round pills; 8px cards; hairline borders</li>
            </ul>
          </div>
        </aside>
      </div>
    </div>
  );
}

function TemperatureControl({ reading, hot }: { reading: string; hot: boolean }) {
  return (
    <div
      className="inline-flex items-center gap-2 rounded-full border border-deal-hairline bg-deal-surface px-3 py-1 text-[14px] font-bold"
      aria-label={`Temperature ${reading}`}
    >
      <span className="text-deal-cold" aria-hidden>
        ↓
      </span>
      <span className={hot ? "text-deal-hot" : "text-deal-cold"}>{reading}</span>
      <span className="text-deal-hot" aria-hidden>
        ↑
      </span>
    </div>
  );
}
