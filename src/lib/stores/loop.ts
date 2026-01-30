import { writable, derived } from 'svelte/store';
import type { LogEntry, ProjectStatus } from '../types';

export interface LoopStoreState {
  status: ProjectStatus;
  currentIteration: number;
  maxIterations: number;
  logs: LogEntry[];
  lastError: string | null;
}

const initialState: LoopStoreState = {
  status: 'ready',
  currentIteration: 0,
  maxIterations: 50,
  logs: [],
  lastError: null
};

export const loopState = writable<LoopStoreState>(initialState);

// Derived stores
export const isRunning = derived(loopState, ($state) => $state.status === 'running');
export const isPaused = derived(loopState, ($state) => $state.status === 'paused');
export const isPausing = derived(loopState, ($state) => $state.status === 'pausing');
export const isDone = derived(loopState, ($state) => $state.status === 'done');
export const isFailed = derived(loopState, ($state) => $state.status === 'failed');

// Actions
export function resetLoop() {
  loopState.set(initialState);
}

export function setMaxIterations(max: number) {
  loopState.update(state => ({ ...state, maxIterations: max }));
}

export function addLog(entry: LogEntry) {
  loopState.update(state => ({
    ...state,
    logs: [...state.logs.slice(-999), entry] // Keep last 1000 logs
  }));
}

export function setStatus(status: ProjectStatus) {
  loopState.update(state => ({ ...state, status }));
}

export function setIteration(iteration: number) {
  loopState.update(state => ({ ...state, currentIteration: iteration }));
}

export function setError(error: string | null) {
  loopState.update(state => ({ ...state, lastError: error }));
}

export function clearLogs() {
  loopState.update(state => ({ ...state, logs: [] }));
}
