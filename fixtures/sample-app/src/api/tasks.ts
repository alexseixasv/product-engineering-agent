import { tasks } from "@/data/mock";
import type { Task } from "@/models/types";

/** Client API module mimicking GET /api/tasks */
export async function listTasks(projectId?: string): Promise<Task[]> {
  if (!projectId) return [...tasks];
  return tasks.filter((t) => t.projectId === projectId);
}

export async function getOverdueTasks(asOfIsoDate: string): Promise<Task[]> {
  return tasks.filter(
    (t) => t.status !== "done" && t.dueDate < asOfIsoDate,
  );
}
