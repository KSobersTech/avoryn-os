import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

type Project = {
    id: string;
    name: string;
    description: string | null;
    workspace: string;
    projectKind: string;
    status: string;
    priority: string;
    startDate: string | null;
    dueDate: string | null;
    completedAt: string | null;
    createdAt: string;
    updatedAt: string;
    deletedAt: string | null;
};

function ProjectsPage() {
    const [projects, setProjects] = useState<Project[]>([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        async function loadProjects() {
            try {
                const result = await invoke<Project[]>("list_projects");
                setProjects(result);
            } catch (error) {
                setError(
                    error instanceof Error
                        ? error.message
                        : String(error)
                );
            } finally {
                setLoading(false);
            }
        }

        void loadProjects();
    }, []);

    return (
        <>
            <h1>Projects</h1>

            {loading && <p>Loading projects...</p>}

            {error && (
                <p role="alert">
                    Unable to load projects: {error}
                </p>
            )}

            {!loading && !error && projects.length === 0 && (
                <p>No projects yet.</p>
            )}

            {!loading && !error && projects.length > 0 && (
                <div>
                    {projects.map((project) => (
                        <article key={project.id}>
                            <h2>{project.name}</h2>

                            {project.description && (
                                <p>{project.description}</p>
                            )}

                            <p>
                                <strong>Workspace:</strong>{" "}
                                {project.workspace}
                            </p>

                            <p>
                                <strong>Status:</strong>{" "}
                                {project.status}
                            </p>

                            <p>
                                <strong>Priority:</strong>{" "}
                                {project.priority}
                            </p>
                        </article>
                    ))}
                </div>
            )}
        </>
    );
}

export default ProjectsPage;