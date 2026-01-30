import { writable } from 'svelte/store';

export type GitRepoCheckReason = 'precheck' | 'runtime';

export interface GitRepoCheckRequest {
  projectId: string;
  reason: GitRepoCheckReason;
}

export const gitRepoCheckRequest = writable<GitRepoCheckRequest | null>(null);

export function requestGitRepoCheck(projectId: string, reason: GitRepoCheckReason) {
  gitRepoCheckRequest.set({ projectId, reason });
}

export function clearGitRepoCheck() {
  gitRepoCheckRequest.set(null);
}
