# Pulseboard — sample fixture app

Target application for the Product Engineering Agent (see ADR-007).

## Run

```bash
cd fixtures/sample-app
npm install
npm run dev
```

## What exists

| Area | Location |
|---|---|
| Domain models | `src/models/types.ts` |
| Mock data | `src/data/mock.ts` |
| API modules | `src/api/{users,projects,tasks}.ts` |
| Design system primitives | `src/components/ui/{Button,Card,Badge,Select,Table}.tsx` |
| Pages | Projects, Tasks (status filter), Users |

## Intentional gaps (analytics dashboard scenario)

- No analytics dashboard route or page
- No chart components / charting library
- No CSV export
- No aggregated productivity metrics API
- `getOverdueTasks` exists in `tasks.ts` but is unused by UI
- Technical debt signal: API modules are client-side mocks without a real backend contract layer

## Design system notes

Primitives follow a lightweight shadcn-style pattern (CVA-less) with Tailwind tokens in `tailwind.config.js`.
