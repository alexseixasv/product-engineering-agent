import { useEffect, useState } from "react";
import { listUsers } from "@/api/users";
import type { User } from "@/models/types";
import { Badge } from "@/components/ui/Badge";
import { Card, CardContent, CardHeader } from "@/components/ui/Card";
import { Table, TBody, TD, TH, THead, TR } from "@/components/ui/Table";

export function UsersPage() {
  const [users, setUsers] = useState<User[]>([]);

  useEffect(() => {
    void listUsers().then(setUsers);
  }, []);

  return (
    <Card>
      <CardHeader>
        <h2 className="text-lg font-semibold">Users</h2>
        <p className="text-sm text-muted-foreground">
          Team roster used for assignment. No productivity metrics API.
        </p>
      </CardHeader>
      <CardContent>
        <Table>
          <THead>
            <TR>
              <TH>Name</TH>
              <TH>Email</TH>
              <TH>Role</TH>
            </TR>
          </THead>
          <TBody>
            {users.map((u) => (
              <TR key={u.id}>
                <TD>{u.name}</TD>
                <TD>{u.email}</TD>
                <TD>
                  <Badge>{u.role}</Badge>
                </TD>
              </TR>
            ))}
          </TBody>
        </Table>
      </CardContent>
    </Card>
  );
}
