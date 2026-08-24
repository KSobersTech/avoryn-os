import { useEffect, useState } from "react";
import type { FormEvent } from "react";
import { invoke } from "@tauri-apps/api/core";

type Workspace =
    | "engineering"
    | "school"
    | "career"
    | "life"
    | "system";

type ProjectKind =
    | "personal"
    | "client"
    | "portfolio"
    | "academic";

type ProjectPriority =
    | "low"
    | "medium"
    | "high"
    | "critical";

type Project = {
    id: string;
    name: string;
    description: string | null;
    workspace: Workspace;
    projectKind: ProjectKind;
    status: string;
    priority: ProjectPriority;
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
    const [creating, setCreating] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [success, setSuccess] = useState<string | null>(null);

    const [name, setName] = useState("");
    const [description, setDescription] = useState("");
    const [workspace, setWorkspace] =
        useState<Workspace>("engineering");
    const [projectKind, setProjectKind] =
        useState<ProjectKind>("portfolio");
    const [priority, setPriority] =
        useState<ProjectPriority>("medium");

    const [editingProjectId, setEditingProjectId] =
        useState<string | null>(null);

    const [editName, setEditName] = useState("");
    const [editDescription, setEditDescription] = useState("");
    const [editWorkspace, setEditWorkspace] =
        useState<Workspace>("engineering");
    const [editProjectKind, setEditProjectKind] =
        useState<ProjectKind>("portfolio");
    const [editStatus, setEditStatus] = useState("planned");
    const [editPriority, setEditPriority] =
        useState<ProjectPriority>("medium");
    const [editStartDate, setEditStartDate] = useState("");
    const [editDueDate, setEditDueDate] = useState("");
    function beginEditingProject(project: Project) {
        setEditingProjectId(project.id);
        setEditName(project.name);
        setEditDescription(project.description ?? "");
        setEditWorkspace(project.workspace);
        setEditProjectKind(project.projectKind);
        setEditStatus(project.status);
        setEditPriority(project.priority);
        setEditStartDate(project.startDate ?? "");
        setEditDueDate(project.dueDate ?? "");
    }
    useEffect(() => {
        async function loadProjects() {
            try {
                const result =
                    await invoke<Project[]>("list_projects");

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

    async function handleCreateProject(
        event: FormEvent<HTMLFormElement>
    ) {
        event.preventDefault();

        const trimmedName = name.trim();

        if (!trimmedName) {
            setError("A project name is required.");
            return;
        }

        setCreating(true);
        setError(null);
        setSuccess(null);

        try {
            const newProject = await invoke<Project>(
                "create_project",
                {
                    input: {
                        name: trimmedName,
                        description:
                            description.trim() || null,
                        workspace,
                        projectKind,
                        priority,
                        startDate: null,
                        dueDate: null,
                    },
                }
            );

            setProjects((currentProjects) => [
                newProject,
                ...currentProjects,
            ]);

            setName("");
            setDescription("");

            setSuccess(
                `Project "${newProject.name}" was created.`
            );
        } catch (error) {
            setError(
                error instanceof Error
                    ? error.message
                    : String(error)
            );
        } finally {
            setCreating(false);
        }
    }
    async function handleUpdateProject(
        event: FormEvent<HTMLFormElement>
    ) {
        event.preventDefault();

        if (!editingProjectId) {
            return;
        }

        const trimmedName = editName.trim();

        if (!trimmedName) {
            setError("A project name is required.");
            return;
        }

        setError(null);
        setSuccess(null);

        try {
            const updatedProject = await invoke<Project>(
                "update_project",
                {
                    projectId: editingProjectId,
                    input: {
                        name: trimmedName,
                        description: editDescription.trim() || null,
                        workspace: editWorkspace,
                        projectKind: editProjectKind,
                        status: editStatus,
                        priority: editPriority,
                        startDate: editStartDate || null,
                        dueDate: editDueDate || null,
                    },
                }
            );

            setProjects((currentProjects) =>
                currentProjects.map((project) =>
                    project.id === updatedProject.id
                        ? updatedProject
                        : project
                )
            );

            setEditingProjectId(null);
            setSuccess(
                `Project "${updatedProject.name}" was updated.`
            );
        } catch (error) {
            setError(
                error instanceof Error
                    ? error.message
                    : String(error)
            );
        }
    }
    return (
        <>
            <h1>Projects</h1>

            <section>
                <h2>Create Project</h2>

                <form onSubmit={handleCreateProject}>
                    <p>
                        <label>
                            Project Name
                            <br />
                            <input
                                type="text"
                                value={name}
                                onChange={(event) =>
                                    setName(event.target.value)
                                }
                                required
                            />
                        </label>
                    </p>

                    <p>
                        <label>
                            Description
                            <br />
                            <textarea
                                value={description}
                                onChange={(event) =>
                                    setDescription(
                                        event.target.value
                                    )
                                }
                            />
                        </label>
                    </p>

                    <p>
                        <label>
                            Workspace
                            <br />
                            <select
                                value={workspace}
                                onChange={(event) =>
                                    setWorkspace(
                                        event.target
                                            .value as Workspace
                                    )
                                }
                            >
                                <option value="engineering">
                                    Engineering
                                </option>
                                <option value="school">
                                    School
                                </option>
                                <option value="career">
                                    Career
                                </option>
                                <option value="life">
                                    Life
                                </option>
                                <option value="system">
                                    System
                                </option>
                            </select>
                        </label>
                    </p>

                    <p>
                        <label>
                            Project Type
                            <br />
                            <select
                                value={projectKind}
                                onChange={(event) =>
                                    setProjectKind(
                                        event.target
                                            .value as ProjectKind
                                    )
                                }
                            >
                                <option value="personal">
                                    Personal
                                </option>
                                <option value="client">
                                    Client
                                </option>
                                <option value="portfolio">
                                    Portfolio
                                </option>
                                <option value="academic">
                                    Academic
                                </option>
                            </select>
                        </label>
                    </p>

                    <p>
                        <label>
                            Priority
                            <br />
                            <select
                                value={priority}
                                onChange={(event) =>
                                    setPriority(
                                        event.target
                                            .value as ProjectPriority
                                    )
                                }
                            >
                                <option value="low">
                                    Low
                                </option>
                                <option value="medium">
                                    Medium
                                </option>
                                <option value="high">
                                    High
                                </option>
                                <option value="critical">
                                    Critical
                                </option>
                            </select>
                        </label>
                    </p>

                    <button
                        type="submit"
                        disabled={creating}
                    >
                        {creating
                            ? "Creating..."
                            : "Create Project"}
                    </button>
                </form>
            </section>

            {success && (
                <p role="status">{success}</p>
            )}

            {error && (
                <p role="alert">
                    AVORYN error: {error}
                </p>
            )}

            <hr />
            {editingProjectId && (
                <section>
                    <h2>Edit Project</h2>

                    <form onSubmit={handleUpdateProject}>
                        <p>
                            <label>
                                Project Name
                                <br />
                                <input
                                    type="text"
                                    value={editName}
                                    onChange={(event) =>
                                        setEditName(event.target.value)
                                    }
                                    required
                                />
                            </label>
                        </p>

                        <p>
                            <label>
                                Description
                                <br />
                                <textarea
                                    value={editDescription}
                                    onChange={(event) =>
                                        setEditDescription(event.target.value)
                                    }
                                />
                            </label>
                        </p>

                        <p>
                            <label>
                                Workspace
                                <br />
                                <select
                                    value={editWorkspace}
                                    onChange={(event) =>
                                        setEditWorkspace(
                                            event.target.value as Workspace
                                        )
                                    }
                                >
                                    <option value="engineering">Engineering</option>
                                    <option value="school">School</option>
                                    <option value="career">Career</option>
                                    <option value="life">Life</option>
                                    <option value="system">System</option>
                                </select>
                            </label>
                        </p>

                        <p>
                            <label>
                                Project Type
                                <br />
                                <select
                                    value={editProjectKind}
                                    onChange={(event) =>
                                        setEditProjectKind(
                                            event.target.value as ProjectKind
                                        )
                                    }
                                >
                                    <option value="personal">Personal</option>
                                    <option value="client">Client</option>
                                    <option value="portfolio">Portfolio</option>
                                    <option value="academic">Academic</option>
                                </select>
                            </label>
                        </p>

                        <p>
                            <label>
                                Status
                                <br />
                                <select
                                    value={editStatus}
                                    onChange={(event) =>
                                        setEditStatus(event.target.value)
                                    }
                                >
                                    <option value="planned">Planned</option>
                                    <option value="active">Active</option>
                                    <option value="paused">Paused</option>
                                    <option value="completed">Completed</option>
                                    <option value="archived">Archived</option>
                                </select>
                            </label>
                        </p>

                        <p>
                            <label>
                                Priority
                                <br />
                                <select
                                    value={editPriority}
                                    onChange={(event) =>
                                        setEditPriority(
                                            event.target.value as ProjectPriority
                                        )
                                    }
                                >
                                    <option value="low">Low</option>
                                    <option value="medium">Medium</option>
                                    <option value="high">High</option>
                                    <option value="critical">Critical</option>
                                </select>
                            </label>
                        </p>

                        <p>
                            <label>
                                Start Date
                                <br />
                                <input
                                    type="date"
                                    value={editStartDate}
                                    onChange={(event) =>
                                        setEditStartDate(event.target.value)
                                    }
                                />
                            </label>
                        </p>

                        <p>
                            <label>
                                Due Date
                                <br />
                                <input
                                    type="date"
                                    value={editDueDate}
                                    onChange={(event) =>
                                        setEditDueDate(event.target.value)
                                    }
                                />
                            </label>
                        </p>

                        <p>
                            <button type="submit">
                                Save Changes
                            </button>{" "}
                            <button
                                type="button"
                                onClick={() => setEditingProjectId(null)}
                            >
                                Cancel
                            </button>
                        </p>
                    </form>

                    <hr />
                </section>
            )}
            <section>
                <h2>Your Projects</h2>

                {loading && <p>Loading projects...</p>}

                {!loading &&
                    !error &&
                    projects.length === 0 && (
                        <p>No projects yet.</p>
                    )}

                {!loading && projects.length > 0 && (
                    <div>
                        {projects.map((project) => (
                            <article key={project.id}>
                                <h3>{project.name}</h3>

                                {project.description && (
                                    <p>
                                        {
                                            project.description
                                        }
                                    </p>
                                )}

                                <p>
                                    <strong>
                                        Workspace:
                                    </strong>{" "}
                                    {project.workspace}
                                </p>

                                <p>
                                    <strong>
                                        Project Type:
                                    </strong>{" "}
                                    {project.projectKind}
                                </p>

                                <p>
                                    <strong>Status:</strong>{" "}
                                    {project.status}
                                </p>

                                <p>
                                    <strong>
                                        Priority:
                                    </strong>{" "}
                                    {project.priority}
                                </p>

                                <p>
                                    <button
                                        type="button"
                                        onClick={() => beginEditingProject(project)}
                                    >
                                        Edit Project
                                    </button>
                                </p>
                            </article>
                        ))}
                    </div>
                )}
            </section>
        </>
    );
}

export default ProjectsPage;