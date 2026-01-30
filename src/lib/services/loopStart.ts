import * as api from '$lib/services/tauri';
import { requestGitRepoCheck } from '$lib/stores/gitRepoCheck';

export const CODEX_GIT_REPO_CHECK_REQUIRED = 'codex_git_repo_check_required';

export function isGitRepoCheckError(error: unknown): boolean {
  const message = error instanceof Error ? error.message : String(error);
  return message.includes(CODEX_GIT_REPO_CHECK_REQUIRED);
}

export async function startLoopWithGuard(projectId: string): Promise<boolean> {
  try {
    await api.startLoop(projectId);
    return true;
  } catch (error) {
    if (isGitRepoCheckError(error)) {
      requestGitRepoCheck(projectId, 'precheck');
      return false;
    }
    throw error;
  }
}
