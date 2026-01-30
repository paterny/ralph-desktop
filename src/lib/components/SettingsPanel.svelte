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
  <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-lg w-full m-4 max-h-[90vh] overflow-hidden flex flex-col">
    <!-- Header -->
    <div class="p-4 border-b border-gray-200 dark:border-gray-700 flex items-center justify-between">
      <h2 class="text-lg font-semibold text-gray-800 dark:text-white">设置</h2>
      <button
        class="text-gray-500 hover:text-gray-700 dark:hover:text-gray-300"
        onclick={onClose}
      >
        ✕
      </button>
    </div>

    <!-- Content -->
    <div class="flex-1 overflow-y-auto p-4 space-y-6">
      <!-- CLI Settings -->
      <section>
        <h3 class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-3">CLI 设置</h3>
        <div class="space-y-3">
          <div>
            <label class="block text-sm text-gray-600 dark:text-gray-400 mb-1">默认 CLI</label>
            <select
              class="w-full p-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-800 dark:text-white"
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
        <h3 class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-3">循环设置</h3>
        <div class="space-y-3">
          <div>
            <label class="block text-sm text-gray-600 dark:text-gray-400 mb-1">默认最大迭代次数</label>
            <input
              type="number"
              class="w-full p-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-800 dark:text-white"
              bind:value={localConfig.defaultMaxIterations}
              min="1"
              max="500"
            />
          </div>
          <div>
            <label class="block text-sm text-gray-600 dark:text-gray-400 mb-1">最大并发项目数</label>
            <input
              type="number"
              class="w-full p-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-800 dark:text-white"
              bind:value={localConfig.maxConcurrentProjects}
              min="1"
              max="10"
            />
          </div>
          <div>
            <label class="block text-sm text-gray-600 dark:text-gray-400 mb-1">迭代超时 (分钟)</label>
            <input
              type="number"
              class="w-full p-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-800 dark:text-white"
              value={localConfig.iterationTimeoutMs / 60000}
              oninput={(e) => localConfig.iterationTimeoutMs = parseInt(e.currentTarget.value) * 60000}
              min="1"
              max="60"
            />
          </div>
          <div>
            <label class="block text-sm text-gray-600 dark:text-gray-400 mb-1">空闲超时 (分钟)</label>
            <input
              type="number"
              class="w-full p-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-800 dark:text-white"
              value={localConfig.idleTimeoutMs / 60000}
              oninput={(e) => localConfig.idleTimeoutMs = parseInt(e.currentTarget.value) * 60000}
              min="1"
              max="30"
            />
          </div>
        </div>
      </section>

      <!-- Appearance -->
      <section>
        <h3 class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-3">外观</h3>
        <div class="space-y-3">
          <div>
            <label class="block text-sm text-gray-600 dark:text-gray-400 mb-1">主题</label>
            <select
              class="w-full p-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-800 dark:text-white"
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
        <h3 class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-3">存储</h3>
        <div class="space-y-3">
          <div>
            <label class="block text-sm text-gray-600 dark:text-gray-400 mb-1">日志保留天数</label>
            <input
              type="number"
              class="w-full p-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-800 dark:text-white"
              bind:value={localConfig.logRetentionDays}
              min="1"
              max="90"
            />
          </div>
        </div>
      </section>

      <!-- Security -->
      <section>
        <h3 class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-3">安全</h3>
        <div class="space-y-3">
          <div class="flex items-center justify-between">
            <div>
              <div class="text-sm text-gray-600 dark:text-gray-400">权限确认状态</div>
              <div class="text-xs text-gray-500">
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
              class="px-3 py-1 text-sm bg-red-100 dark:bg-red-900/30 text-red-600 dark:text-red-400 rounded hover:bg-red-200 dark:hover:bg-red-900/50"
              onclick={handleResetPermissions}
            >
              重置
            </button>
          </div>
        </div>
      </section>

      <!-- CLI Info -->
      <section>
        <h3 class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-3">已安装的 CLI</h3>
        <div class="space-y-2">
          {#each $availableClis as cli}
            <div class="flex items-center justify-between p-2 bg-gray-50 dark:bg-gray-700/50 rounded">
              <div class="flex items-center gap-2">
                <span class={cli.available ? 'text-green-500' : 'text-gray-400'}>
                  {cli.available ? '✓' : '✕'}
                </span>
                <span class="text-sm text-gray-800 dark:text-white">{cli.name}</span>
              </div>
              <span class="text-xs text-gray-500">{cli.version || '未安装'}</span>
            </div>
          {/each}
        </div>
      </section>
    </div>

    <!-- Footer -->
    <div class="p-4 border-t border-gray-200 dark:border-gray-700 flex justify-end gap-3">
      <button
        class="px-4 py-2 text-gray-600 dark:text-gray-400 hover:text-gray-800 dark:hover:text-white"
        onclick={onClose}
      >
        取消
      </button>
      <button
        class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50"
        onclick={handleSave}
        disabled={saving}
      >
        {saving ? '保存中...' : '保存'}
      </button>
    </div>
  </div>
</div>
