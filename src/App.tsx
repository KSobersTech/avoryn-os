import { useState } from "react";
import MainLayout from "./layouts/MainLayout";
import DashboardPage from "./pages/DashboardPage";
import ProjectsPage from "./pages/ProjectsPage";
import TasksPage from "./pages/TasksPage";
import MemoryPage from "./pages/MemoryPage";
import JournalPage from "./pages/JournalPage";
import KnowledgeVaultPage from "./pages/KnowledgeVaultPage";
import SettingsPage from "./pages/SettingsPage";
import type { PageId } from "./types/navigation";

function App() {
  const [activePage, setActivePage] =
    useState<PageId>("dashboard");

  function renderPage() {
    switch (activePage) {
      case "dashboard":
        return <DashboardPage />;

      case "projects":
        return <ProjectsPage />;

      case "tasks":
        return <TasksPage />;

      case "memory":
        return <MemoryPage />;

      case "journal":
        return <JournalPage />;

      case "vault":
        return <KnowledgeVaultPage />;

      case "settings":
        return <SettingsPage />;
    }
  }

  return (
    <MainLayout
      activePage={activePage}
      onNavigate={setActivePage}
    >
      {renderPage()}
    </MainLayout>
  );
}

export default App;