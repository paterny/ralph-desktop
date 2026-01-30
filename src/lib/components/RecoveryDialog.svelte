<script lang="ts">
  import * as api from '$lib/services/tauri';
  import type { RecoveryInfo } from '$lib/services/tauri';

  interface Props {
    tasks: RecoveryInfo[];
    onRecover: (projectId: string) => void;
    onCancel: (projectId: string) => void;
    onDismiss: () => void;
  }

  let { tasks, onRecover, onCancel, onDismiss }: Props = $props();

  async function handleRecover(projectId: string) {
    onRecover(projectId);
  }

  async function handleCancel(projectId: string) {
    try {
      await api.cancelInterruptedTask(projectId);
      onCancel(projectId);
    } catch (error) {
      console.error('Failed to cancel task:', error);
    }
  }

  async function handleCancelAll() {
    for (const task of tasks) {
      try {
        await api.cancelInterruptedTask(task.projectId);
      } catch (error) {
        console.error('Failed to cancel task:', error);
      }
    }
    onDismiss();
  }
</script>

<div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
  <div class="bg-vscode-panel border border-vscode rounded-lg shadow-xl max-w-lg w-full m-4">
    <!-- Header -->
    <div class="p-4 border-b border-vscode">
      <h2 class="text-lg font-semibold text-vscode flex items-center gap-2">
        <span class="text-vscode-warning">⚠️</span>
        检测到中断的任务
      </h2>
      <p class="text-sm text-vscode-dim mt-1">
        上次关闭时有 {tasks.length} 个任务正在运行
      </p>
    </div>

    <!-- Task List -->
    <div class="p-4 max-h-64 overflow-y-auto">
      <div class="space-y-3">
        {#each tasks as task (task.projectId)}
          <div class="p-3 bg-vscode-input border border-vscode rounded-lg">
            <div class="flex items-start justify-between">
              <div>
                <div class="font-medium text-vscode">
                  {task.projectName}
                </div>
                <div class="text-sm text-vscode-muted mt-1">
                  迭代 #{task.iteration} · 状态: {task.status}
                </div>
              </div>
              <div class="flex gap-2">
                <button
                  class="px-3 py-1 text-sm bg-vscode-accent bg-vscode-accent-hover text-white rounded"
                  onclick={() => handleRecover(task.projectId)}
                >
                  恢复
                </button>
                <button
                  class="px-3 py-1 text-sm bg-vscode-panel border border-vscode text-vscode-dim rounded hover:bg-vscode-hover"
                  onclick={() => handleCancel(task.projectId)}
                >
                  取消
                </button>
              </div>
            </div>
          </div>
        {/each}
      </div>
    </div>

    <!-- Footer -->
    <div class="p-4 border-t border-vscode flex justify-between">
      <button
        class="px-4 py-2 text-sm text-vscode-error hover:opacity-90"
        onclick={handleCancelAll}
      >
        全部取消
      </button>
      <button
        class="px-4 py-2 text-sm text-vscode-dim hover:text-vscode"
        onclick={onDismiss}
      >
        稍后处理
      </button>
    </div>
  </div>
</div>
