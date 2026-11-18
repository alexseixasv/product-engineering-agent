import { useEffect, useMemo, useState } from "react";
import { listTasks } from "@/api/tasks";
import type { Task } from "@/models/types";
import { Badge } from "@/components/ui/Badge";
import { Card, CardContent, CardHeader } from "@/components/ui/Card";
import { Select } from "@/components/ui/Select";
import { Table, TBody, TD, TH, THead, TR } from "@/components/ui/Table";

function statusTone(status: Task["status"]) {
  if (status === "done") return "success" as const;
  if (status === "blocked") return "danger" as const;
  if (status === "in_progress") return "warning" as const;
  return "neutral" as const;
}

export function TasksPage() {
  const [tasks, setTasks] = useState<Task[]>([]);
  const [statusFilter, setStatusFilter] = useState<string>("all");

  useEffect(() => {
    void listTasks().then(setTasks);
  }, []);

  const filtered = useMemo(() => {
    if (statusFilter === "all") return tasks;
    return tasks.filter((t) => t.status === statusFilter);
  }, [tasks, statusFilter]);

  return (
    <Card>
      <CardHeader className="flex flex-row items-center justify-between gap-4">
        <div>
          <h2 className="text-lg font-semibold">Tasks</h2>
          <p className="text-sm text-muted-foreground">
            Filter by status. Charts and CSV export are not implemented.
          </p>
        </div>
        <Select
          aria-label="Filter by status"
          value={statusFilter}
          onChange={(e) => setStatusFilter(e.target.value)}
        >
          <option value="all">All statuses</option>
          <option value="todo">Todo</option>
          <option value="in_progress">In progress</option>
          <option value="blocked">Blocked</option>
          <option value="done">Done</option>
        </Select>
      </CardHeader>
      <CardContent>
        <Table>
          <THead>
            <TR>
              <TH>Title</TH>
              <TH>Project</TH>
              <TH>Status</TH>
              <TH>Due</TH>
            </TR>
          </THead>
          <TBody>
            {filtered.map((t) => (
              <TR key={t.id}>
                <TD>{t.title}</TD>
                <TD>{t.projectId}</TD>
                <TD>
                  <Badge tone={statusTone(t.status)}>{t.status}</Badge>
                </TD>
                <TD>{t.dueDate}</TD>
              </TR>
            ))}
          </TBody>
        </Table>
      </CardContent>
    </Card>
  );
}
