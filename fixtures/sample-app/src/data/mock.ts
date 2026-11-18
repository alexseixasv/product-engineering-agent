import type { Project, Task, User } from "@/models/types";

export const users: User[] = [
  { id: "u1", name: "Alex Manager", email: "alex@example.com", role: "manager" },
  { id: "u2", name: "Sam Dev", email: "sam@example.com", role: "member" },
  { id: "u3", name: "Riley Dev", email: "riley@example.com", role: "member" },
];

export const projects: Project[] = [
  { id: "p1", name: "Mobile Onboarding", status: "active", ownerId: "u1" },
  { id: "p2", name: "Billing Cleanup", status: "paused", ownerId: "u1" },
  { id: "p3", name: "Search Relevance", status: "active", ownerId: "u1" },
];

export const tasks: Task[] = [
  {
    id: "t1",
    projectId: "p1",
    title: "Wire welcome checklist",
    assigneeId: "u2",
    status: "in_progress",
    dueDate: "2026-11-10",
    estimateHours: 8,
  },
  {
    id: "t2",
    projectId: "p1",
    title: "QA onboarding emails",
    assigneeId: "u3",
    status: "blocked",
    dueDate: "2026-11-05",
    estimateHours: 4,
  },
  {
    id: "t3",
    projectId: "p2",
    title: "Archive unused plans",
    assigneeId: "u2",
    status: "todo",
    dueDate: "2026-11-20",
    estimateHours: 6,
  },
  {
    id: "t4",
    projectId: "p3",
    title: "Tune synonym dictionary",
    assigneeId: "u3",
    status: "done",
    dueDate: "2026-11-01",
    estimateHours: 5,
  },
];
