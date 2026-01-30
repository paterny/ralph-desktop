<script lang="ts">
  import '../app.css';
  import { onMount } from 'svelte';
  import { projects, currentProjectId, currentProject, updateProjects, updateCurrentProject, updateProjectStatus, selectProject } from '$lib/stores/projects';
  import { config, availableClis, updateConfig, setAvailableClis } from '$lib/stores/settings';
  import { loopState, setStatus, setIteration, addLog, setError } from '$lib/stores/loop';
  import { gitRepoCheckRequest, clearGitRepoCheck, requestGitRepoCheck } from '$lib/stores/gitRepoCheck';
  import { dequeueProject, isInQueue, markRunning } from '$lib/stores/queue';
  import { notifySuccess, notifyError, notifyWarning } from '$lib/stores/notifications';
  import * as api from '$lib/services/tauri';
  import { CODEX_GIT_REPO_CHECK_REQUIRED, startLoopWithGuard } from '$lib/services/loopStart';
  import type { LoopEvent } from '$lib/types';
  import type { RecoveryInfo } from '$lib/services/tauri';
  import RecoveryDialog from '$lib/components/RecoveryDialog.svelte';
  import NotificationToast from '$lib/components/NotificationToast.svelte';
  import GitRepoCheckDialog from '$lib/components/GitRepoCheckDialog.svelte';

  let { children } = $props();
  let initialized = $state(false);
  let showPermissionDialog = $state(false);
  let showRecoveryDialog = $state(false);
  let interruptedTasks = $state<RecoveryInfo[]>([]);
  let gitRepoBusy = $state(false);

  onMount(async () => {
    try {
      // Load config
      const loadedConfig = await api.getConfig();
      updateConfig(loadedConfig);

      // Check if permissions need confirmation
      if (!loadedConfig.permissionsConfirmed) {
        showPermissionDialog = true;
      }

      // Detect CLIs
      const clis = await api.detectInstalledClis();
      setAvailableClis(clis);

      // Load projects
      const projectList = await api.listProjects();
      updateProjects(projectList);

      // Check for interrupted tasks
      const interrupted = await api.checkInterruptedTasks();
      if (interrupted.length > 0) {
        interruptedTasks = interrupted;
        showRecoveryDialog = true;
      }

      // Listen to loop events
      await api.listenToLoopEvents(handleLoopEvent);

      // Clean up old logs on startup
      await api.cleanupLogs();

      initialized = true;
    } catch (error) {
      console.error('Initialization error:', error);
      initialized = true; // Still show UI even on error
    }
  });

  function handleLoopEvent(event: LoopEvent) {
    const statusMap: Record<string, string> = {
      iterationStart: 'running',
      pausing: 'pausing',
      paused: 'paused',
      resumed: 'running',
      completed: 'done',
      maxIterationsReached: 'failed',
      stopped: 'cancelled'
    };

    if (event.type === 'output' && event.content) {
      addLog({
        iteration: event.iteration || 0,
        content: event.content,
        isStderr: event.isStderr || false,
        timestamp: new Date()
      });
    }

    if (event.type === 'error' && event.error) {
      if (event.error.includes(CODEX_GIT_REPO_CHECK_REQUIRED)) {
        requestGitRepoCheck(event.projectId, 'runtime');
        return;
      }
      setError(event.error);
      notifyError('执行错误', event.error);
      updateProjectStatus(event.projectId, 'failed');
    }

    if (event.type === 'completed') {
      notifySuccess('任务完成', `项目已成功完成所有迭代`);
    }

    if (event.type === 'maxIterationsReached') {
      notifyWarning('达到最大迭代次数', `任务已在第 ${event.iteration} 次迭代后停止`);
    }

    if (event.iteration !== undefined) {
      setIteration(event.iteration);
    }

    const newStatus = statusMap[event.type];
    if (newStatus) {
      setStatus(newStatus as any);
      updateProjectStatus(event.projectId, newStatus as any);
    }
  }

  async function handleConfirmPermissions() {
    await api.confirmPermissions();
    const loadedConfig = await api.getConfig();
    updateConfig(loadedConfig);
    showPermissionDialog = false;
  }

  function handleRecoverTask(projectId: string) {
    selectProject(projectId);
    interruptedTasks = interruptedTasks.filter(t => t.projectId !== projectId);
    if (interruptedTasks.length === 0) {
      showRecoveryDialog = false;
    }
  }

  function handleCancelTask(projectId: string) {
    interruptedTasks = interruptedTasks.filter(t => t.projectId !== projectId);
    if (interruptedTasks.length === 0) {
      showRecoveryDialog = false;
    }
  }

  function handleDismissRecovery() {
    showRecoveryDialog = false;
  }

  async function startLoopFromDialog(projectId: string) {
    const started = await startLoopWithGuard(projectId);
    if (started && isInQueue(projectId)) {
      dequeueProject(projectId);
      markRunning(projectId);
    }
  }

  async function handleInitGitRepo(projectId: string) {
    gitRepoBusy = true;
    try {
      await api.initProjectGitRepo(projectId);
      clearGitRepoCheck();
      await startLoopFromDialog(projectId);
    } catch (error) {
      console.error('Failed to init git repo:', error);
      notifyError('初始化 Git 失败', String(error));
    } finally {
      gitRepoBusy = false;
    }
  }

  async function handleSkipGitRepoCheck(projectId: string) {
    gitRepoBusy = true;
    try {
      const updated = await api.setProjectSkipGitRepoCheck(projectId, true);
      if ($currentProject && $currentProject.id === updated.id) {
        updateCurrentProject(updated);
      }
      clearGitRepoCheck();
      await startLoopFromDialog(projectId);
    } catch (error) {
      console.error('Failed to skip git repo check:', error);
      notifyError('跳过检查失败', String(error));
    } finally {
      gitRepoBusy = false;
    }
  }

  function handleCancelGitRepoCheck() {
    clearGitRepoCheck();
  }
</script>

{#if showPermissionDialog}
  <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
    <div class="bg-vscode-panel border border-vscode rounded-lg shadow-xl max-w-lg p-6 m-4">
      <h2 class="text-xl font-bold text-vscode-warning mb-4">
        ⚠️ 重要安全提示
      </h2>
      <div class="text-vscode space-y-3 mb-6">
        <p>Ralph Desktop 需要以自动执行模式运行 AI 编程助手。这意味着：</p>
        <ul class="list-disc list-inside space-y-1 ml-2">
          <li>AI 可以自动读取、创建、修改、删除项目目录中的文件</li>
          <li>AI 可以自动执行命令（如 npm install、git commit 等）</li>
          <li>AI 的操作不会逐一询问确认</li>
        </ul>
        <p class="font-medium">建议：</p>
        <ul class="list-disc list-inside space-y-1 ml-2">
          <li>仅在你信任的项目目录中使用</li>
          <li>确保项目使用 Git 版本控制，以便回滚</li>
          <li>不要在包含敏感数据的目录中使用</li>
        </ul>
      </div>
      <div class="flex justify-end gap-3">
        <button
          class="px-4 py-2 rounded-lg bg-vscode-panel border border-vscode text-vscode-dim hover:bg-vscode-hover"
          onclick={() => window.close()}
        >
          取消
        </button>
        <button
          class="px-4 py-2 rounded-lg bg-vscode-accent bg-vscode-accent-hover text-white"
          onclick={handleConfirmPermissions}
        >
          确认并继续
        </button>
      </div>
    </div>
  </div>
{/if}

{#if showRecoveryDialog && interruptedTasks.length > 0}
  <RecoveryDialog
    tasks={interruptedTasks}
    onRecover={handleRecoverTask}
    onCancel={handleCancelTask}
    onDismiss={handleDismissRecovery}
  />
{/if}

{#if $gitRepoCheckRequest}
  {@const pending = $gitRepoCheckRequest}
  {@const meta = $projects.find(p => p.id === pending.projectId)}
  <GitRepoCheckDialog
    projectName={meta?.name || $currentProject?.name || 'Unknown Project'}
    projectPath={meta?.path || $currentProject?.path || ''}
    reason={pending.reason}
    busy={gitRepoBusy}
    onInit={() => handleInitGitRepo(pending.projectId)}
    onSkip={() => handleSkipGitRepoCheck(pending.projectId)}
    onCancel={handleCancelGitRepoCheck}
  />
{/if}

{#if initialized}
  {@render children()}
{:else}
  <div class="flex items-center justify-center h-screen bg-vscode-editor">
    <div class="text-center">
      <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-vscode-accent mx-auto mb-4"></div>
      <p class="text-vscode-muted">加载中...</p>
    </div>
  </div>
{/if}

<NotificationToast />
