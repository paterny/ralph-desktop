<script lang="ts">
  interface Props {
    show: boolean;
    onClose: () => void;
  }

  let { show, onClose }: Props = $props();

  const shortcuts = [
    { keys: ['⌘', 'N'], description: '新建项目' },
    { keys: ['⌘', ','], description: '打开设置' },
    { keys: ['⌘', 'Enter'], description: '开始/继续任务' },
    { keys: ['⌘', 'P'], description: '暂停任务' },
    { keys: ['⌘', '.'], description: '停止任务' },
    { keys: ['Esc'], description: '关闭对话框' },
    { keys: ['↑', '↓'], description: '切换项目' },
    { keys: ['⌘', '?'], description: '显示快捷键帮助' },
  ];
</script>

{#if show}
  <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" onclick={onClose}>
    <div
      class="bg-vscode-panel border border-vscode rounded-lg shadow-xl max-w-md w-full m-4"
      onclick={(e) => e.stopPropagation()}
    >
      <div class="p-4 border-b border-vscode flex items-center justify-between">
        <h2 class="text-lg font-semibold text-vscode">键盘快捷键</h2>
        <button
          class="p-1 hover:bg-vscode-hover rounded text-vscode-dim"
          onclick={onClose}
        >
          ✕
        </button>
      </div>
      <div class="p-4">
        <div class="space-y-3">
          {#each shortcuts as shortcut}
            <div class="flex items-center justify-between">
              <span class="text-vscode">{shortcut.description}</span>
              <div class="flex gap-1">
                {#each shortcut.keys as key}
                  <kbd class="px-2 py-1 text-xs font-mono bg-vscode-input border border-vscode rounded text-vscode">
                    {key}
                  </kbd>
                {/each}
              </div>
            </div>
          {/each}
        </div>
      </div>
      <div class="p-4 border-t border-vscode">
        <p class="text-xs text-vscode-muted text-center">
          按 <kbd class="px-1.5 py-0.5 text-xs font-mono bg-vscode-input border border-vscode rounded">Esc</kbd> 关闭
        </p>
      </div>
    </div>
  </div>
{/if}
