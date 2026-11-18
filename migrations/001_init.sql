CREATE TABLE IF NOT EXISTS runs (
    id TEXT PRIMARY KEY,
    request TEXT NOT NULL,
    repo_path TEXT NOT NULL,
    state TEXT NOT NULL,
    proposal_json JSONB,
    approval_json JSONB,
    error TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_runs_updated_at ON runs (updated_at DESC);
