function Sidebar() {
    return (
        <aside
            style={{
                width: "260px",
                minHeight: "100vh",
                background: "#0f172a",
                color: "white",
                padding: "24px",
                boxSizing: "border-box",
            }}
        >
            <h2>AVORYN</h2>

            <hr />

            <p>Dashboard</p>
            <p>Projects</p>
            <p>Tasks</p>
            <p>Memory</p>
            <p>Journal</p>
            <p>Knowledge Vault</p>
            <p>Settings</p>
        </aside>
    );
}

export default Sidebar;