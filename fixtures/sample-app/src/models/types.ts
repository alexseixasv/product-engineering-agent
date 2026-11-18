export type User = {
  id: string;
  name: string;
  email: string;
  role: "manager" | "member";
};

export type Project = {
  id: string;
  name: string;
  status: "active" | "paused" | "done";
  ownerId: string;
};

export type TaskStatus = "todo" | "in_progress" | "done" | "blocked";

export type Task = {
  id: string;
  projectId: string;
  title: string;
  assigneeId: string;
  status: TaskStatus;
  dueDate: string;
  estimateHours: number;
};
