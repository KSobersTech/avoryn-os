import Sidebar from "../components/Sidebar";

type Props = {
    children?: React.ReactNode;
};

function MainLayout({ children }: Props) {
    return (
        <div
            style={{
                display: "flex",
                minHeight: "100vh",
                background: "#f4f7fb",
            }}
        >
            <Sidebar />

            <main
                style={{
                    flex: 1,
                    padding: "40px",
                }}
            >
                {children}
            </main>
        </div>
    );
}

export default MainLayout;