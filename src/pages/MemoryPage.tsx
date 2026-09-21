import { useEffect, useState } from "react";

import { listMemories } from "../services/memoryService";
import type { Memory } from "../types/memory";

function MemoryPage() {
    const [memories, setMemories] = useState<Memory[]>([]);
    const [isLoading, setIsLoading] = useState(true);
    const [errorMessage, setErrorMessage] = useState<string | null>(null);

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

    return (
        <>
            <h1>Memory</h1>

            {isLoading && <p>Loading memories...</p>}

            {errorMessage && <p role="alert">{errorMessage}</p>}

            {!isLoading && !errorMessage && memories.length === 0 && (
                <p>No AVORYN memory records have been created yet.</p>
            )}

            {!isLoading && !errorMessage && memories.length > 0 && (
                <ul>
                    {memories.map((memory) => (
                        <li key={memory.id}>
                            <h2>{memory.title}</h2>
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