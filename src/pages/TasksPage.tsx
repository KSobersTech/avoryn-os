import { useEffect, useState } from "react";
import type { FormEvent } from "react";
import { invoke } from "@tauri-apps/api/core";

type Workspace =
    | "engineering"
    | "school"
    | "career"
    | "life"
    | "system";

type ProjectPriority =
    | "low"
    | "medium"
    | "high"
    | "critical";

type TaskStatus =
    | "todo"
    | "in_progress"
    | "blocked"
    | "completed"
    | "cancelled";

type Task = {
    id: string;
    projectId: string | null;
    title: string;
    description: string | null;
    workspace: Workspace;
    status: TaskStatus;
    priority: ProjectPriority;
    startDate: string | null;
    dueDate: string | null;
    completedAt: string | null;
    createdAt: string;
    updatedAt: string;
    deletedAt: string | null;
};

type ProjectSummary = {
    id: string;
    name: string;
};

function TasksPage() {
    const [tasks, setTasks] = useState<Task[]>([]);
    const [projects, setProjects] = useState<ProjectSummary[]>([]);

    const [loading, setLoading] = useState(true);
    const [creating, setCreating] = useState(false);

    const [error, setError] = useState<string | null>(null);
    const [success, setSuccess] = useState<string | null>(null);

    const [title, setTitle] = useState("");
    const [description, setDescription] = useState("");
    const [workspace, setWorkspace] =
        useState<Workspace>("engineering");
    const [priority, setPriority] =
        useState<ProjectPriority>("medium");
    const [projectId, setProjectId] = useState("");
    const [startDate, setStartDate] = useState("");
    const [dueDate, setDueDate] = useState("");

    const [editingTaskId, setEditingTaskId] =
        useState<string | null>(null);

    const [editTitle, setEditTitle] = useState("");
    const [editDescription, setEditDescription] = useState("");
    const [editWorkspace, setEditWorkspace] =
        useState<Workspace>("engineering");
    const [editStatus, setEditStatus] =
        useState<TaskStatus>("todo");
    const [editPriority, setEditPriority] =
        useState<ProjectPriority>("medium");
    const [editProjectId, setEditProjectId] = useState("");
    const [editStartDate, setEditStartDate] = useState("");
    const [editDueDate, setEditDueDate] = useState("");

    useEffect(() => {
        async function loadData() {
            try {
                const [taskResult, projectResult] =
                    await Promise.all([
                        invoke<Task[]>("list_tasks"),
                        invoke<ProjectSummary[]>("list_projects"),
                    ]);

                setTasks(taskResult);
                setProjects(projectResult);
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

        void loadData();
    }, []);

    function beginEditingTask(task: Task) {
        setEditingTaskId(task.id);
        setEditTitle(task.title);
        setEditDescription(task.description ?? "");
        setEditWorkspace(task.workspace);
        setEditStatus(task.status);
        setEditPriority(task.priority);
        setEditProjectId(task.projectId ?? "");
        setEditStartDate(task.startDate ?? "");
        setEditDueDate(task.dueDate ?? "");

        setError(null);
        setSuccess(null);
    }

    function getProjectName(taskProjectId: string | null) {
        if (!taskProjectId) {
            return "None";
        }

        const project = projects.find(
            (project) => project.id === taskProjectId
        );

        return project?.name ?? "Unknown project";
    }

    function displayStatus(status: TaskStatus) {
        return status.replace("_", " ");
    }

    async function handleCreateTask(
        event: FormEvent<HTMLFormElement>
    ) {
        event.preventDefault();

        const trimmedTitle = title.trim();

        if (!trimmedTitle) {
            setError("A task title is required.");
            return;
        }

        setCreating(true);
        setError(null);
        setSuccess(null);

        try {
            const newTask = await invoke<Task>(
                "create_task",
                {
                    input: {
                        projectId: projectId || null,
                        title: trimmedTitle,
                        description:
                            description.trim() || null,
                        workspace,
                        priority,
                        startDate: startDate || null,
                        dueDate: dueDate || null,
                    },
                }
            );

            setTasks((currentTasks) => [
                newTask,
                ...currentTasks,
            ]);

            setTitle("");
            setDescription("");
            setProjectId("");
            setStartDate("");
            setDueDate("");

            setSuccess(
                `Task "${newTask.title}" was created.`
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

    async function handleUpdateTask(
        event: FormEvent<HTMLFormElement>
    ) {
        event.preventDefault();

        if (!editingTaskId) {
            return;
        }

        const trimmedTitle = editTitle.trim();

        if (!trimmedTitle) {
            setError("A task title is required.");
            return;
        }

        setError(null);
        setSuccess(null);

        try {
            const updatedTask = await invoke<Task>(
                "update_task",
                {
                    taskId: editingTaskId,
                    input: {
                        projectId: editProjectId || null,
                        title: trimmedTitle,
                        description:
                            editDescription.trim() || null,
                        workspace: editWorkspace,
                        status: editStatus,
                        priority: editPriority,
                        startDate: editStartDate || null,
                        dueDate: editDueDate || null,
                    },
                }
            );

            setTasks((currentTasks) =>
                currentTasks.map((task) =>
                    task.id === updatedTask.id
                        ? updatedTask
                        : task
                )
            );

            setEditingTaskId(null);

            setSuccess(
                `Task "${updatedTask.title}" was updated.`
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
            <h1>Tasks</h1>

            <section>
                <h2>Create Task</h2>

                <form onSubmit={handleCreateTask}>
                    <p>
                        <label>
                            Task Title
                            <br />
                            <input
                                type="text"
                                value={title}
                                onChange={(event) =>
                                    setTitle(event.target.value)
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
                            Related Project
                            <br />
                            <select
                                value={projectId}
                                onChange={(event) =>
                                    setProjectId(
                                        event.target.value
                                    )
                                }
                            >
                                <option value="">
                                    No Project
                                </option>

                                {projects.map((project) => (
                                    <option
                                        key={project.id}
                                        value={project.id}
                                    >
                                        {project.name}
                                    </option>
                                ))}
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
                                <option value="low">Low</option>
                                <option value="medium">
                                    Medium
                                </option>
                                <option value="high">High</option>
                                <option value="critical">
                                    Critical
                                </option>
                            </select>
                        </label>
                    </p>

                    <p>
                        <label>
                            Start Date
                            <br />
                            <input
                                type="date"
                                value={startDate}
                                onChange={(event) =>
                                    setStartDate(
                                        event.target.value
                                    )
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
                                value={dueDate}
                                onChange={(event) =>
                                    setDueDate(
                                        event.target.value
                                    )
                                }
                            />
                        </label>
                    </p>

                    <button
                        type="submit"
                        disabled={creating}
                    >
                        {creating
                            ? "Creating..."
                            : "Create Task"}
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

            {editingTaskId && (
                <section>
                    <h2>Edit Task</h2>

                    <form onSubmit={handleUpdateTask}>
                        <p>
                            <label>
                                Task Title
                                <br />
                                <input
                                    type="text"
                                    value={editTitle}
                                    onChange={(event) =>
                                        setEditTitle(
                                            event.target.value
                                        )
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
                                        setEditDescription(
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
                                    value={editWorkspace}
                                    onChange={(event) =>
                                        setEditWorkspace(
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
                                Related Project
                                <br />
                                <select
                                    value={editProjectId}
                                    onChange={(event) =>
                                        setEditProjectId(
                                            event.target.value
                                        )
                                    }
                                >
                                    <option value="">
                                        No Project
                                    </option>

                                    {projects.map((project) => (
                                        <option
                                            key={project.id}
                                            value={project.id}
                                        >
                                            {project.name}
                                        </option>
                                    ))}
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
                                        setEditStatus(
                                            event.target
                                                .value as TaskStatus
                                        )
                                    }
                                >
                                    <option value="todo">
                                        To Do
                                    </option>
                                    <option value="in_progress">
                                        In Progress
                                    </option>
                                    <option value="blocked">
                                        Blocked
                                    </option>
                                    <option value="completed">
                                        Completed
                                    </option>
                                    <option value="cancelled">
                                        Cancelled
                                    </option>
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

                        <p>
                            <label>
                                Start Date
                                <br />
                                <input
                                    type="date"
                                    value={editStartDate}
                                    onChange={(event) =>
                                        setEditStartDate(
                                            event.target.value
                                        )
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
                                        setEditDueDate(
                                            event.target.value
                                        )
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
                                onClick={() =>
                                    setEditingTaskId(null)
                                }
                            >
                                Cancel
                            </button>
                        </p>
                    </form>

                    <hr />
                </section>
            )}

            <section>
                <h2>Your Tasks</h2>

                {loading && (
                    <p>Loading tasks...</p>
                )}

                {!loading &&
                    !error &&
                    tasks.length === 0 && (
                        <p>No tasks yet.</p>
                    )}

                {!loading && tasks.length > 0 && (
                    <div>
                        {tasks.map((task) => (
                            <article key={task.id}>
                                <h3>{task.title}</h3>

                                {task.description && (
                                    <p>
                                        {task.description}
                                    </p>
                                )}

                                <p>
                                    <strong>
                                        Workspace:
                                    </strong>{" "}
                                    {task.workspace}
                                </p>

                                <p>
                                    <strong>
                                        Project:
                                    </strong>{" "}
                                    {getProjectName(
                                        task.projectId
                                    )}
                                </p>

                                <p>
                                    <strong>
                                        Status:
                                    </strong>{" "}
                                    {displayStatus(
                                        task.status
                                    )}
                                </p>

                                <p>
                                    <strong>
                                        Priority:
                                    </strong>{" "}
                                    {task.priority}
                                </p>

                                {task.startDate && (
                                    <p>
                                        <strong>
                                            Start Date:
                                        </strong>{" "}
                                        {task.startDate}
                                    </p>
                                )}

                                {task.dueDate && (
                                    <p>
                                        <strong>
                                            Due Date:
                                        </strong>{" "}
                                        {task.dueDate}
                                    </p>
                                )}

                                <p>
                                    <button
                                        type="button"
                                        onClick={() =>
                                            beginEditingTask(
                                                task
                                            )
                                        }
                                    >
                                        Edit Task
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

export default TasksPage;