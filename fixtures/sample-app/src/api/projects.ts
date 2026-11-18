import { projects } from "@/data/mock";
import type { Project } from "@/models/types";

/** Client API module mimicking GET /api/projects */
export async function listProjects(): Promise<Project[]> {
  return [...projects];
}

export async function getProject(id: string): Promise<Project | undefined> {
  return projects.find((p) => p.id === id);
}
