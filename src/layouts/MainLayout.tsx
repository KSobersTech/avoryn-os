import type { ReactNode } from "react";
import Sidebar from "../components/Sidebar";
import type { PageId } from "../types/navigation";

type MainLayoutProps = {
    activePage: PageId;
    onNavigate: (page: PageId) => void;
    children: ReactNode;
};

function MainLayout({
    activePage,
    onNavigate,
    children,
}: MainLayoutProps) {
    return (
        <div
            style={{
                display: "flex",
                minHeight: "100vh",
                backgroundColor: "#f4f7fb",
            }}
        >
            <Sidebar
                activePage={activePage}
                onNavigate={onNavigate}
            />

            <main
                style={{
                    flex: 1,
                    padding: "40px",
                    color: "#0f172a",
                }}
            >
                {children}
            </main>
        </div>
    );
}

export default MainLayout;