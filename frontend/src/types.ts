export type Confidence = "fact" | "assumption" | "open_question";

export type Evidence = {
  path: string;
  start_line: number;
  end_line: number;
  excerpt: string;
};

export type Finding = {
  id: string;
  category: string;
  summary: string;
  confidence: Confidence;
  evidence: Evidence[];
};

export type Requirement = {
  id: string;
  statement: string;
  acceptance_criteria: string[];
};

export type BacklogTicket = {
  key: string;
  type: string;
  title: string;
  description: string;
  acceptance_criteria: string[];
  requirement_ids: string[];
  depends_on: string[];
  definition_of_done: string[];
};

export type EngineeringProposal = {
  run_id: string;
  request: string;
  findings: Finding[];
  feasibility: {
    summary: string;
    approach_options: string[];
    risks: string[];
  };
  technical_debt: Array<{
    title: string;
    impact: string;
    recommendation: string;
    confidence: Confidence;
    evidence: Evidence[];
  }>;
  requirements: {
    functional: Requirement[];
    non_functional: Requirement[];
  };
  backlog: BacklogTicket[];
  open_questions: string[];
  design_md_path?: string | null;
  snapshot_path?: string | null;
};

export type RunRecord = {
  id: string;
  request: string;
  repo_path: string;
  state: string;
  proposal?: EngineeringProposal | null;
  approval?: {
    decision: string;
    actor: string;
    decided_at: string;
    comment?: string | null;
  } | null;
  error?: string | null;
};
