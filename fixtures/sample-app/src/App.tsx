import { Link, Navigate, Route, Routes } from "react-router-dom";
import { ProjectsPage } from "@/pages/ProjectsPage";
import { TasksPage } from "@/pages/TasksPage";
import { UsersPage } from "@/pages/UsersPage";

export default function App() {
  return (
    <div className="min-h-screen">
      <header className="border-b border-border">
        <div className="mx-auto flex max-w-5xl items-center justify-between px-4 py-4">
          <div>
            <p className="text-xl font-semibold tracking-tight">Pulseboard</p>
            <p className="text-sm text-muted-foreground">
              Sample productivity app for repository intelligence demos
            </p>
          </div>
          <nav className="flex gap-3 text-sm">
            <Link className="hover:underline" to="/projects">
              Projects
            </Link>
            <Link className="hover:underline" to="/tasks">
              Tasks
            </Link>
            <Link className="hover:underline" to="/users">
              Users
            </Link>
          </nav>
        </div>
      </header>
      <main className="mx-auto max-w-5xl px-4 py-6">
        <Routes>
          <Route path="/" element={<Navigate to="/tasks" replace />} />
          <Route path="/projects" element={<ProjectsPage />} />
          <Route path="/tasks" element={<TasksPage />} />
          <Route path="/users" element={<UsersPage />} />
        </Routes>
      </main>
    </div>
  );
}
