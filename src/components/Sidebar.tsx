import type { PageId } from "../types/navigation";

type SidebarProps = {
    activePage: PageId;
    onNavigate: (page: PageId) => void;
};

const navigationItems: { id: PageId; label: string }[] = [
    { id: "dashboard", label: "Dashboard" },
    { id: "projects", label: "Projects" },
    { id: "tasks", label: "Tasks" },
    { id: "memory", label: "Memory" },
    { id: "journal", label: "Journal" },
    { id: "vault", label: "Knowledge Vault" },
    { id: "settings", label: "Settings" },
];

function Sidebar({ activePage, onNavigate }: SidebarProps) {
    return (
        <aside
            style={{
                width: "260px",
                minHeight: "100vh",
                backgroundColor: "#0f172a",
                color: "#ffffff",
                padding: "24px",
                boxSizing: "border-box",
            }}
        >
            <h2
                style={{
                    marginTop: 0,
                    marginBottom: "24px",
                    fontSize: "28px",
                }}
            >
                AVORYN
            </h2>

            <nav
                aria-label="Primary navigation"
                style={{
                    display: "flex",
                    flexDirection: "column",
                    gap: "10px",
                }}
            >
                {navigationItems.map((item) => {
                    const isActive = activePage === item.id;

                    return (
                        <button
                            key={item.id}
                            type="button"
                            onClick={() => onNavigate(item.id)}
                            style={{
                                width: "100%",
                                padding: "12px 14px",
                                border: "none",
                                borderRadius: "8px",
                                backgroundColor: isActive ? "#1e293b" : "transparent",
                                color: "#ffffff",
                                font: "inherit",
                                textAlign: "left",
                                cursor: "pointer",
                            }}
                        >
                            {item.label}
                        </button>
                    );
                })}
            </nav>
        </aside>
    );
}

export default Sidebar;