<script lang="ts">
  import type { ProjectState } from '$lib/types';

  interface Props {
    project: ProjectState;
    onSave?: (prompt: string) => void;
    onCancel?: () => void;
  }

  let { project, onSave, onCancel }: Props = $props();

  let editedPrompt = $state(project.task?.prompt || '');
  let isEditing = $state(false);

  function handleEdit() {
    isEditing = true;
  }

  function handleSave() {
    onSave?.(editedPrompt);
    isEditing = false;
  }

  function handleCancel() {
    editedPrompt = project.task?.prompt || '';
    isEditing = false;
    onCancel?.();
  }

  function copyToClipboard() {
    navigator.clipboard.writeText(editedPrompt);
  }
</script>

<div class="bg-vscode-panel rounded-lg border border-vscode">
  <!-- Header -->
  <div class="flex items-center justify-between p-3 border-b border-vscode">
    <h3 class="font-medium text-vscode flex items-center gap-2">
      <span>📝</span>
      <span>Prompt 预览</span>
    </h3>
    <div class="flex gap-2">
      {#if !isEditing}
        <button
          class="px-3 py-1 text-sm bg-vscode-input hover:bg-vscode-hover rounded text-vscode-dim border border-vscode"
          onclick={copyToClipboard}
        >
          复制
        </button>
        <button
          class="px-3 py-1 text-sm bg-vscode-accent bg-vscode-accent-hover rounded text-white"
          onclick={handleEdit}
        >
          编辑
        </button>
      {:else}
        <button
          class="px-3 py-1 text-sm bg-vscode-input hover:bg-vscode-hover rounded text-vscode-dim border border-vscode"
          onclick={handleCancel}
        >
          取消
        </button>
        <button
          class="px-3 py-1 text-sm bg-vscode-accent bg-vscode-accent-hover rounded text-white"
          onclick={handleSave}
        >
          保存
        </button>
      {/if}
    </div>
  </div>

  <!-- Content -->
  <div class="p-3">
    {#if isEditing}
      <textarea
        class="w-full h-64 p-3 font-mono text-sm bg-vscode-editor border border-vscode rounded-lg resize-none focus-vscode text-vscode"
        bind:value={editedPrompt}
        placeholder="输入 prompt..."
      ></textarea>
    {:else}
      <div class="max-h-64 overflow-y-auto">
        <pre class="whitespace-pre-wrap font-mono text-sm text-vscode bg-vscode-editor p-3 rounded-lg border border-vscode">{editedPrompt || '(无 prompt)'}</pre>
      </div>
    {/if}
  </div>

  <!-- Stats -->
  <div class="px-3 pb-3">
    <div class="text-xs text-vscode-muted flex gap-4">
      <span>字符数: {editedPrompt.length}</span>
      <span>行数: {editedPrompt.split('\n').length}</span>
    </div>
  </div>
</div>
