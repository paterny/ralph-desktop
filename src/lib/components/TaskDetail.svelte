<script lang="ts">
  import type { ProjectState, LogEntry } from '$lib/types';
  import type { LoopStoreState } from '$lib/stores/loop';
  import * as api from '$lib/services/tauri';
  import { startLoopWithGuard } from '$lib/services/loopStart';
  import LogViewer from './LogViewer.svelte';
  import PromptEditor from './PromptEditor.svelte';

  interface Props {
    project: ProjectState;
    loopState: LoopStoreState;
  }

  let { project, loopState }: Props = $props();

  let starting = $state(false);
  let showPrompt = $state(false);

  const statusConfig: Record<string, { icon: string; color: string; label: string }> = {
    ready: { icon: '⚪', color: 'text-vscode-muted', label: '就绪' },
    queued: { icon: '🔵', color: 'text-vscode-info', label: '排队中' },
    running: { icon: '🟢', color: 'text-vscode-success', label: '运行中' },
    pausing: { icon: '🟡', color: 'text-vscode-warning', label: '正在暂停...' },
    paused: { icon: '🟡', color: 'text-vscode-warning', label: '已暂停' },
    done: { icon: '✅', color: 'text-vscode-success', label: '已完成' },
    failed: { icon: '❌', color: 'text-vscode-error', label: '失败' },
    cancelled: { icon: '🚫', color: 'text-vscode-muted', label: '已取消' },
    brainstorming: { icon: '💭', color: 'text-vscode-accent', label: 'Brainstorming' }
  };

  const status = $derived(statusConfig[project.status] || statusConfig.ready);
  const isRunning = $derived(project.status === 'running');
  const isPaused = $derived(project.status === 'paused');
  const isPausing = $derived(project.status === 'pausing');
  const canStart = $derived(['ready', 'failed', 'cancelled'].includes(project.status));

  async function handleStart() {
    starting = true;
    try {
      await startLoopWithGuard(project.id);
    } catch (error) {
      console.error('Failed to start loop:', error);
    } finally {
      starting = false;
    }
  }

  async function handlePause() {
    try {
      await api.pauseLoop(project.id);
    } catch (error) {
      console.error('Failed to pause loop:', error);
    }
  }

  async function handleResume() {
    try {
      await api.resumeLoop(project.id);
    } catch (error) {
      console.error('Failed to resume loop:', error);
    }
  }

  async function handleStop() {
    if (confirm('确定要停止吗？这将立即终止当前执行。')) {
      try {
        await api.stopLoop(project.id);
      } catch (error) {
        console.error('Failed to stop loop:', error);
      }
    }
  }
</script>

<div class="flex-1 flex flex-col overflow-hidden">
  <!-- Header -->
  <div class="p-4 bg-vscode-panel border-b border-vscode">
    <div class="flex items-start justify-between">
      <div>
        <div class="flex items-center gap-2">
          <span class="text-2xl">📁</span>
          <h2 class="text-xl font-bold text-vscode">{project.name}</h2>
        </div>
        <p class="text-sm text-vscode-dim mt-1">{project.path}</p>
      </div>
      <div class="flex items-center gap-2">
        <span class={status.color}>{status.icon}</span>
        <span class="text-sm font-medium text-vscode-dim">{status.label}</span>
      </div>
    </div>

    <!-- Task Info -->
    {#if project.task}
      <div class="mt-4 p-3 bg-vscode-input rounded-lg border border-vscode">
        <div class="flex items-center justify-between mb-2">
          <div class="grid grid-cols-3 gap-4 text-sm flex-1">
            <div>
              <span class="text-vscode-muted">CLI:</span>
              <span class="ml-2 text-vscode font-medium">
                {project.task.cli === 'claude' ? 'Claude Code' : 'Codex'}
              </span>
            </div>
            <div>
              <span class="text-vscode-muted">Iteration:</span>
              <span class="ml-2 text-vscode font-medium">
                {loopState.currentIteration} / {project.task.maxIterations}
              </span>
            </div>
            <div>
              <span class="text-vscode-muted">Status:</span>
              <span class="ml-2 {status.color} font-medium">{status.label}</span>
            </div>
          </div>
          <button
            class="ml-4 px-3 py-1 text-sm bg-vscode-panel border border-vscode hover:bg-vscode-hover rounded text-vscode-dim"
            onclick={() => showPrompt = !showPrompt}
          >
            {showPrompt ? '隐藏 Prompt' : '查看 Prompt'}
          </button>
        </div>
        {#if showPrompt}
          <div class="mt-3">
            <PromptEditor {project} />
          </div>
        {/if}
      </div>
    {/if}
  </div>

  <!-- Log Viewer -->
  <div class="flex-1 overflow-hidden bg-vscode-editor">
    <LogViewer logs={loopState.logs} />
  </div>

  <!-- Control Bar -->
  <div class="p-4 bg-vscode-panel border-t border-vscode">
    <div class="flex items-center justify-between">
      <div class="flex gap-2">
        {#if canStart}
          <button
            class="px-4 py-2 bg-vscode-accent bg-vscode-accent-hover text-white rounded-lg flex items-center gap-2 disabled:opacity-50"
            onclick={handleStart}
            disabled={starting}
          >
            <span>▶</span>
            <span>{starting ? '启动中...' : '开始'}</span>
          </button>
        {/if}

        {#if isRunning}
          <button
            class="px-4 py-2 bg-vscode-warning text-black rounded-lg flex items-center gap-2 hover:opacity-90"
            onclick={handlePause}
          >
            <span>⏸</span>
            <span>暂停</span>
          </button>
        {/if}

        {#if isPaused}
          <button
            class="px-4 py-2 bg-vscode-accent bg-vscode-accent-hover text-white rounded-lg flex items-center gap-2"
            onclick={handleResume}
          >
            <span>▶</span>
            <span>继续</span>
          </button>
        {/if}

        {#if isRunning || isPaused || isPausing}
          <button
            class="px-4 py-2 bg-vscode-error text-white rounded-lg flex items-center gap-2 hover:opacity-90"
            onclick={handleStop}
          >
            <span>⏹</span>
            <span>停止</span>
          </button>
        {/if}
      </div>

      {#if loopState.lastError}
        <div class="text-sm text-vscode-error">
          Error: {loopState.lastError}
        </div>
      {/if}
    </div>
  </div>
</div>
