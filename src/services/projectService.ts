import { invoke } from "@tauri-apps/api/core";

import type { Project } from "../types/project";

export async function listProjects(): Promise<Project[]> {
  return invoke<Project[]>("list_projects");
}
