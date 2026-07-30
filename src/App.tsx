import { useState } from "react";
import MainLayout from "./layouts/MainLayout";
import DashboardPage from "./pages/DashboardPage";
import type { PageId } from "./types/navigation";

function App() {
  const [activePage, setActivePage] =
    useState<PageId>("dashboard");

  function renderPage() {
    switch (activePage) {
      case "dashboard":
        return <DashboardPage />;

      case "projects":
        return (
          <>
            <h1>Projects</h1>
            <p>Your AVORYN projects will appear here.</p>
          </>
        );

      case "tasks":
        return (
          <>
            <h1>Tasks</h1>
            <p>Your tasks will appear here.</p>
          </>
        );

      case "memory":
        return (
          <>
            <h1>Memory</h1>
            <p>AVORYN memory records will appear here.</p>
          </>
        );

      case "journal":
        return (
          <>
            <h1>Journal</h1>
            <p>Your journal entries will appear here.</p>
          </>
        );

      case "vault":
        return (
          <>
            <h1>Knowledge Vault</h1>
            <p>Your stored knowledge will appear here.</p>
          </>
        );

      case "settings":
        return (
          <>
            <h1>Settings</h1>
            <p>AVORYN configuration options will appear here.</p>
          </>
        );
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