import { writable } from 'svelte/store';
import type { UpdateState } from '../types';
import * as api from '$lib/services/tauri';

const defaultState: UpdateState = {
  status: 'idle',
  currentVersion: '0.0.0',
  targetVersion: null,
  lastCheckedAt: null,
  failureCount: 0,
  lastError: null,
  downloadPath: null,
  sha256: null,
  pending: false
};

export const updateState = writable<UpdateState>(defaultState);

let timer: ReturnType<typeof setInterval> | null = null;

export async function initAutoUpdate(getIsIdle: () => boolean) {
  try {
    const state = await api.loadUpdateState();
    updateState.set(state);
  } catch {
    updateState.set(defaultState);
  }

  if (!timer) {
    timer = setInterval(() => {
      void checkForUpdatesIfIdle(getIsIdle());
    }, 60 * 60 * 1000);
  }

  setTimeout(() => {
    void checkForUpdatesIfIdle(getIsIdle());
  }, 90 * 1000);
}

export async function checkForUpdates() {
  try {
    const state = await api.checkForUpdates(false);
    updateState.set(state);
  } catch (error) {
    updateState.update(prev => ({
      ...prev,
      status: 'failed',
      lastError: String(error)
    }));
  }
}

export async function checkForUpdatesIfIdle(isIdle: boolean) {
  try {
    const state = await api.checkForUpdates(isIdle);
    updateState.set(state);
  } catch (error) {
    updateState.update(prev => ({
      ...prev,
      status: 'failed',
      lastError: String(error)
    }));
  }
}
