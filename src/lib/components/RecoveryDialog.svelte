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
  <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-lg w-full m-4">
    <!-- Header -->
    <div class="p-4 border-b border-gray-200 dark:border-gray-700">
      <h2 class="text-lg font-semibold text-gray-800 dark:text-white flex items-center gap-2">
        <span class="text-yellow-500">⚠️</span>
        检测到中断的任务
      </h2>
      <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">
        上次关闭时有 {tasks.length} 个任务正在运行
      </p>
    </div>

    <!-- Task List -->
    <div class="p-4 max-h-64 overflow-y-auto">
      <div class="space-y-3">
        {#each tasks as task (task.projectId)}
          <div class="p-3 bg-gray-50 dark:bg-gray-700/50 rounded-lg">
            <div class="flex items-start justify-between">
              <div>
                <div class="font-medium text-gray-800 dark:text-white">
                  {task.projectName}
                </div>
                <div class="text-sm text-gray-500 dark:text-gray-400 mt-1">
                  迭代 #{task.iteration} · 状态: {task.status}
                </div>
              </div>
              <div class="flex gap-2">
                <button
                  class="px-3 py-1 text-sm bg-blue-600 text-white rounded hover:bg-blue-700"
                  onclick={() => handleRecover(task.projectId)}
                >
                  恢复
                </button>
                <button
                  class="px-3 py-1 text-sm bg-gray-200 dark:bg-gray-600 text-gray-700 dark:text-gray-200 rounded hover:bg-gray-300 dark:hover:bg-gray-500"
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
    <div class="p-4 border-t border-gray-200 dark:border-gray-700 flex justify-between">
      <button
        class="px-4 py-2 text-sm text-red-600 hover:text-red-700"
        onclick={handleCancelAll}
      >
        全部取消
      </button>
      <button
        class="px-4 py-2 text-sm text-gray-600 dark:text-gray-400 hover:text-gray-800 dark:hover:text-white"
        onclick={onDismiss}
      >
        稍后处理
      </button>
    </div>
  </div>
</div>
