<script lang="ts">
  import { config, availableClis, updateConfig } from '$lib/stores/settings';
  import * as api from '$lib/services/tauri';
  import type { GlobalConfig, CliType, Theme } from '$lib/types';

  interface Props {
    onClose: () => void;
  }

  let { onClose }: Props = $props();

  let localConfig = $state<GlobalConfig>({ ...$config });
  let saving = $state(false);

  async function handleSave() {
    saving = true;
    try {
      await api.saveConfig(localConfig);
      updateConfig(localConfig);
      onClose();
    } catch (error) {
      console.error('Failed to save config:', error);
    } finally {
      saving = false;
    }
  }

  async function handleResetPermissions() {
    if (confirm('确定要重置权限确认吗？下次启动时会再次显示安全提示。')) {
      localConfig.permissionsConfirmed = false;
      localConfig.permissionsConfirmedAt = undefined;
    }
  }
</script>

<div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
  <div class="bg-vscode-panel border border-vscode rounded-lg shadow-xl max-w-lg w-full m-4 max-h-[90vh] overflow-hidden flex flex-col">
    <!-- Header -->
    <div class="p-4 border-b border-vscode flex items-center justify-between">
      <h2 class="text-lg font-semibold text-vscode">设置</h2>
      <button
        class="text-vscode-dim hover:text-vscode"
        onclick={onClose}
      >
        ✕
      </button>
    </div>

    <!-- Content -->
    <div class="flex-1 overflow-y-auto p-4 space-y-6">
      <!-- CLI Settings -->
      <section>
        <h3 class="text-sm font-medium text-vscode mb-3">CLI 设置</h3>
        <div class="space-y-3">
          <div>
            <label class="block text-sm text-vscode-muted mb-1">默认 CLI</label>
            <select
              class="w-full p-2 border border-vscode rounded-lg bg-vscode-input text-vscode focus-vscode"
              bind:value={localConfig.defaultCli}
            >
              {#each $availableClis.filter(c => c.available) as cli}
                <option value={cli.cliType}>{cli.name}</option>
              {/each}
            </select>
          </div>
        </div>
      </section>

      <!-- Loop Settings -->
      <section>
        <h3 class="text-sm font-medium text-vscode mb-3">循环设置</h3>
        <div class="space-y-3">
          <div>
            <label class="block text-sm text-vscode-muted mb-1">默认最大迭代次数</label>
            <input
              type="number"
              class="w-full p-2 border border-vscode rounded-lg bg-vscode-input text-vscode focus-vscode"
              bind:value={localConfig.defaultMaxIterations}
              min="1"
              max="500"
            />
          </div>
          <div>
            <label class="block text-sm text-vscode-muted mb-1">最大并发项目数</label>
            <input
              type="number"
              class="w-full p-2 border border-vscode rounded-lg bg-vscode-input text-vscode focus-vscode"
              bind:value={localConfig.maxConcurrentProjects}
              min="1"
              max="10"
            />
          </div>
          <div>
            <label class="block text-sm text-vscode-muted mb-1">迭代超时 (分钟，0 = 不限制)</label>
            <input
              type="number"
              class="w-full p-2 border border-vscode rounded-lg bg-vscode-input text-vscode focus-vscode"
              value={localConfig.iterationTimeoutMs / 60000}
              oninput={(e) => {
                const minutes = Number.parseInt(e.currentTarget.value, 10);
                localConfig.iterationTimeoutMs = Number.isFinite(minutes) && minutes > 0 ? minutes * 60000 : 0;
              }}
              min="0"
            />
          </div>
          <div>
            <label class="block text-sm text-vscode-muted mb-1">空闲超时 (分钟，0 = 不限制)</label>
            <input
              type="number"
              class="w-full p-2 border border-vscode rounded-lg bg-vscode-input text-vscode focus-vscode"
              value={localConfig.idleTimeoutMs / 60000}
              oninput={(e) => {
                const minutes = Number.parseInt(e.currentTarget.value, 10);
                localConfig.idleTimeoutMs = Number.isFinite(minutes) && minutes > 0 ? minutes * 60000 : 0;
              }}
              min="0"
            />
          </div>
        </div>
      </section>

      <!-- Appearance -->
      <section>
        <h3 class="text-sm font-medium text-vscode mb-3">外观</h3>
        <div class="space-y-3">
          <div>
            <label class="block text-sm text-vscode-muted mb-1">主题</label>
            <select
              class="w-full p-2 border border-vscode rounded-lg bg-vscode-input text-vscode focus-vscode"
              bind:value={localConfig.theme}
            >
              <option value="system">跟随系统</option>
              <option value="light">浅色</option>
              <option value="dark">深色</option>
            </select>
          </div>
        </div>
      </section>

      <!-- Storage -->
      <section>
        <h3 class="text-sm font-medium text-vscode mb-3">存储</h3>
        <div class="space-y-3">
          <div>
            <label class="block text-sm text-vscode-muted mb-1">日志保留天数</label>
            <input
              type="number"
              class="w-full p-2 border border-vscode rounded-lg bg-vscode-input text-vscode focus-vscode"
              bind:value={localConfig.logRetentionDays}
              min="1"
              max="90"
            />
          </div>
        </div>
      </section>

      <!-- Security -->
      <section>
        <h3 class="text-sm font-medium text-vscode mb-3">安全</h3>
        <div class="space-y-3">
          <div class="flex items-center justify-between">
            <div>
              <div class="text-sm text-vscode-muted">权限确认状态</div>
              <div class="text-xs text-vscode-muted">
                {#if localConfig.permissionsConfirmed}
                  已确认
                  {#if localConfig.permissionsConfirmedAt}
                    ({new Date(localConfig.permissionsConfirmedAt).toLocaleDateString('zh-CN')})
                  {/if}
                {:else}
                  未确认
                {/if}
              </div>
            </div>
            <button
              class="px-3 py-1 text-sm bg-vscode-input border border-vscode text-vscode-error rounded hover:bg-vscode-hover"
              onclick={handleResetPermissions}
            >
              重置
            </button>
          </div>
        </div>
      </section>

      <!-- CLI Info -->
      <section>
        <h3 class="text-sm font-medium text-vscode mb-3">已安装的 CLI</h3>
        <div class="space-y-2">
          {#each $availableClis as cli}
            <div class="flex items-center justify-between p-2 bg-vscode-input border border-vscode rounded">
              <div class="flex items-center gap-2">
                <span class={cli.available ? 'text-vscode-success' : 'text-vscode-muted'}>
                  {cli.available ? '✓' : '✕'}
                </span>
                <span class="text-sm text-vscode">{cli.name}</span>
              </div>
              <span class="text-xs text-vscode-muted">{cli.version || '未安装'}</span>
            </div>
          {/each}
        </div>
      </section>
    </div>

    <!-- Footer -->
    <div class="p-4 border-t border-vscode flex justify-end gap-3">
      <button
        class="px-4 py-2 text-vscode-dim hover:text-vscode"
        onclick={onClose}
      >
        取消
      </button>
      <button
        class="px-4 py-2 bg-vscode-accent bg-vscode-accent-hover text-white rounded-lg disabled:opacity-50"
        onclick={handleSave}
        disabled={saving}
      >
        {saving ? '保存中...' : '保存'}
      </button>
    </div>
  </div>
</div>
