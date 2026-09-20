import { invoke } from "@tauri-apps/api/core";

import type {
    CreateMemoryInput,
    Memory,
    UpdateMemoryInput,
} from "../types/memory";

export async function createMemory(
    input: CreateMemoryInput,
): Promise<Memory> {
    return invoke<Memory>("create_memory", { input });
}

export async function listMemories(): Promise<Memory[]> {
    return invoke<Memory[]>("list_memories");
}

export async function updateMemory(
    memoryId: string,
    input: UpdateMemoryInput,
): Promise<Memory> {
    return invoke<Memory>("update_memory", { memoryId, input });
}