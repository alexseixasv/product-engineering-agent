import { useEffect, useState } from "react";
import { listProjects } from "@/api/projects";
import type { Project } from "@/models/types";
import { Badge } from "@/components/ui/Badge";
import { Card, CardContent, CardHeader } from "@/components/ui/Card";
import { Table, TBody, TD, TH, THead, TR } from "@/components/ui/Table";

export function ProjectsPage() {
  const [projects, setProjects] = useState<Project[]>([]);

  useEffect(() => {
    void listProjects().then(setProjects);
  }, []);

  return (
    <Card>
      <CardHeader>
        <h2 className="text-lg font-semibold">Projects</h2>
        <p className="text-sm text-muted-foreground">
          Active delivery workstreams. No analytics rollup exists yet.
        </p>
      </CardHeader>
      <CardContent>
        <Table>
          <THead>
            <TR>
              <TH>Name</TH>
              <TH>Status</TH>
              <TH>Owner</TH>
            </TR>
          </THead>
          <TBody>
            {projects.map((p) => (
              <TR key={p.id}>
                <TD>{p.name}</TD>
                <TD>
                  <Badge tone={p.status === "active" ? "success" : "neutral"}>
                    {p.status}
                  </Badge>
                </TD>
                <TD>{p.ownerId}</TD>
              </TR>
            ))}
          </TBody>
        </Table>
      </CardContent>
    </Card>
  );
}
