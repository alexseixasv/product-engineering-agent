import { users } from "@/data/mock";
import type { User } from "@/models/types";

/** Client API module mimicking GET /api/users */
export async function listUsers(): Promise<User[]> {
  return [...users];
}

export async function getUser(id: string): Promise<User | undefined> {
  return users.find((u) => u.id === id);
}
