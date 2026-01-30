<script lang="ts">
  import { notifications, removeNotification, type Notification } from '$lib/stores/notifications';

  function getIcon(type: Notification['type']): string {
    switch (type) {
      case 'success': return '✅';
      case 'error': return '❌';
      case 'warning': return '⚠️';
      case 'info': return 'ℹ️';
    }
  }

  function getBgColor(type: Notification['type']): string {
    switch (type) {
      case 'success': return 'bg-vscode-panel border-vscode-success';
      case 'error': return 'bg-vscode-panel border-vscode-error';
      case 'warning': return 'bg-vscode-panel border-vscode-warning';
      case 'info': return 'bg-vscode-panel border-vscode-info';
    }
  }

  function getTextColor(type: Notification['type']): string {
    switch (type) {
      case 'success': return 'text-vscode-success';
      case 'error': return 'text-vscode-error';
      case 'warning': return 'text-vscode-warning';
      case 'info': return 'text-vscode-info';
    }
  }
</script>

<div class="fixed bottom-4 right-4 z-50 flex flex-col gap-2 max-w-sm">
  {#each $notifications as notification (notification.id)}
    <div
      class="flex items-start gap-3 p-3 rounded-lg border-l-4 shadow-lg animate-slide-in {getBgColor(notification.type)}"
      role="alert"
    >
      <span class="text-lg flex-shrink-0">{getIcon(notification.type)}</span>
      <div class="flex-1 min-w-0">
        <div class="font-medium {getTextColor(notification.type)}">{notification.title}</div>
        {#if notification.message}
          <div class="text-sm mt-1 {getTextColor(notification.type)} opacity-80">{notification.message}</div>
        {/if}
      </div>
      <button
        class="flex-shrink-0 p-1 hover:bg-vscode-hover rounded {getTextColor(notification.type)}"
        onclick={() => removeNotification(notification.id)}
      >
        ✕
      </button>
    </div>
  {/each}
</div>

<style>
  @keyframes slide-in {
    from {
      transform: translateX(100%);
      opacity: 0;
    }
    to {
      transform: translateX(0);
      opacity: 1;
    }
  }
  .animate-slide-in {
    animation: slide-in 0.3s ease-out;
  }
</style>
