import { useEffect, useState, type FormEvent } from "react";

import {
    createMemory,
    listMemories,
} from "../services/memoryService";
import type {
    CreateMemoryInput,
    Memory,
    MemoryImportance,
    MemoryKind,
    Workspace,
} from "../types/memory";

function MemoryPage() {
    const [memories, setMemories] = useState<Memory[]>([]);
    const [isLoading, setIsLoading] = useState(true);
    const [errorMessage, setErrorMessage] = useState<string | null>(null);

    const [title, setTitle] = useState("");
    const [content, setContent] = useState("");
    const [workspace, setWorkspace] =
        useState<Workspace>("engineering");
    const [memoryKind, setMemoryKind] =
        useState<MemoryKind>("note");
    const [importance, setImportance] =
        useState<MemoryImportance>("medium");
    const [source, setSource] = useState("");
    const [isSaving, setIsSaving] = useState(false);
    const [saveErrorMessage, setSaveErrorMessage] =
        useState<string | null>(null);

    useEffect(() => {
        async function loadMemories() {
            try {
                const loadedMemories = await listMemories();
                setMemories(loadedMemories);
            } catch (error) {
                const message =
                    error instanceof Error
                        ? error.message
                        : "Unable to load memories.";

                setErrorMessage(message);
            } finally {
                setIsLoading(false);
            }
        }

        void loadMemories();
    }, []);

    async function handleSubmit(event: FormEvent<HTMLFormElement>) {
        event.preventDefault();
        setIsSaving(true);
        setSaveErrorMessage(null);

        const input: CreateMemoryInput = {
            projectId: null,
            title: title.trim(),
            content: content.trim(),
            workspace,
            memoryKind,
            importance,
            source: source.trim() || null,
            occurredAt: null,
        };

        try {
            const createdMemory = await createMemory(input);

            setMemories((currentMemories) => [
                createdMemory,
                ...currentMemories,
            ]);

            setTitle("");
            setContent("");
            setSource("");
        } catch (error) {
            const message =
                error instanceof Error
                    ? error.message
                    : "Unable to save the memory.";

            setSaveErrorMessage(message);
        } finally {
            setIsSaving(false);
        }
    }

    return (
        <>
            <h1>Memory</h1>

            <form onSubmit={handleSubmit}>
                <h2>Create Memory</h2>

                <p>
                    <label htmlFor="memory-title">Title</label>
                    <br />
                    <input
                        id="memory-title"
                        type="text"
                        value={title}
                        onChange={(event) => setTitle(event.target.value)}
                        maxLength={160}
                        required
                    />
                </p>

                <p>
                    <label htmlFor="memory-content">Content</label>
                    <br />
                    <textarea
                        id="memory-content"
                        value={content}
                        onChange={(event) => setContent(event.target.value)}
                        maxLength={10000}
                        rows={6}
                        required
                    />
                </p>

                <p>
                    <label htmlFor="memory-workspace">Workspace</label>
                    <br />
                    <select
                        id="memory-workspace"
                        value={workspace}
                        onChange={(event) =>
                            setWorkspace(event.target.value as Workspace)
                        }
                    >
                        <option value="engineering">Engineering</option>
                        <option value="school">School</option>
                        <option value="career">Career</option>
                        <option value="life">Life</option>
                        <option value="system">System</option>
                    </select>
                </p>

                <p>
                    <label htmlFor="memory-kind">Memory type</label>
                    <br />
                    <select
                        id="memory-kind"
                        value={memoryKind}
                        onChange={(event) =>
                            setMemoryKind(event.target.value as MemoryKind)
                        }
                    >
                        <option value="note">Note</option>
                        <option value="fact">Fact</option>
                        <option value="preference">Preference</option>
                        <option value="lesson">Lesson</option>
                        <option value="milestone">Milestone</option>
                    </select>
                </p>

                <p>
                    <label htmlFor="memory-importance">Importance</label>
                    <br />
                    <select
                        id="memory-importance"
                        value={importance}
                        onChange={(event) =>
                            setImportance(
                                event.target.value as MemoryImportance,
                            )
                        }
                    >
                        <option value="low">Low</option>
                        <option value="medium">Medium</option>
                        <option value="high">High</option>
                        <option value="critical">Critical</option>
                    </select>
                </p>

                <p>
                    <label htmlFor="memory-source">Source (optional)</label>
                    <br />
                    <input
                        id="memory-source"
                        type="text"
                        value={source}
                        onChange={(event) => setSource(event.target.value)}
                        maxLength={500}
                    />
                </p>

                {saveErrorMessage && (
                    <p role="alert">{saveErrorMessage}</p>
                )}

                <button type="submit" disabled={isSaving}>
                    {isSaving ? "Saving..." : "Save Memory"}
                </button>
            </form>

            <h2>Stored Memories</h2>

            {isLoading && <p>Loading memories...</p>}

            {errorMessage && <p role="alert">{errorMessage}</p>}

            {!isLoading && !errorMessage && memories.length === 0 && (
                <p>No AVORYN memory records have been created yet.</p>
            )}

            {!isLoading && !errorMessage && memories.length > 0 && (
                <ul>
                    {memories.map((memory) => (
                        <li key={memory.id}>
                            <h3>{memory.title}</h3>
                            <p>{memory.content}</p>
                            <p>
                                {memory.workspace} · {memory.memoryKind} ·{" "}
                                {memory.importance}
                            </p>
                        </li>
                    ))}
                </ul>
            )}
        </>
    );
}

export default MemoryPage;