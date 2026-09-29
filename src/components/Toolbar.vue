<script setup lang="ts">
import type { WorkMode } from '@/types'
import type { OutputMode } from '@/composables/useJsonEngine'

defineProps<{
  mode: WorkMode
  isBusy: boolean
  activeView: 'text' | 'tree'
  outputMode: OutputMode
  wrap: boolean
}>()

const emit = defineEmits<{
  'set-output-mode': [mode: OutputMode]
  'set-view': [view: 'text' | 'tree']
  search: []
  'toggle-wrap': []
  clear: []
  refresh: []
}>()
</script>

<template>
  <div
    class="flex items-center gap-1 border-b border-gray-200 bg-gray-50 px-3 py-2 dark:border-gray-800 dark:bg-gray-900"
  >
    <template v-if="activeView === 'text'">
      <button
        class="toolbar-btn"
        :class="{ 'toolbar-btn--active': outputMode === 'formatted' }"
        @click="emit('set-output-mode', 'formatted')"
      >
        格式化
      </button>
      <button
        class="toolbar-btn"
        :class="{ 'toolbar-btn--active': outputMode === 'minified' }"
        @click="emit('set-output-mode', 'minified')"
      >
        压缩
      </button>
      <div class="mx-1 h-5 w-px bg-gray-300 dark:bg-gray-700" />
    </template>

    <button class="toolbar-btn" :disabled="isBusy" @click="emit('refresh')">校验</button>

    <div class="mx-1 h-5 w-px bg-gray-300 dark:bg-gray-700" />

    <button
      class="toolbar-btn"
      :class="{ 'toolbar-btn--active': activeView === 'text' }"
      @click="emit('set-view', 'text')"
    >
      文本
    </button>
    <button
      class="toolbar-btn"
      :class="{ 'toolbar-btn--active': activeView === 'tree' }"
      @click="emit('set-view', 'tree')"
    >
      树形
    </button>
    <button class="toolbar-btn" @click="emit('search')">搜索</button>
    <button
      v-if="activeView === 'text'"
      class="toolbar-btn"
      :class="{ 'toolbar-btn--active': wrap }"
      title="自动换行"
      @click="emit('toggle-wrap')"
    >
      自动换行
    </button>

    <div class="flex-1" />

    <span
      v-if="mode === 'large'"
      class="rounded-full bg-amber-100 px-2 py-0.5 text-xs font-medium text-amber-800 dark:bg-amber-900/40 dark:text-amber-300"
    >
      ⚡ 大文件模式
    </span>

    <button class="toolbar-btn" @click="emit('clear')">清空</button>
  </div>
</template>

<style scoped>
.toolbar-btn {
  @apply rounded-md px-3 py-1.5 text-sm font-medium text-gray-700 hover:bg-gray-200 disabled:cursor-not-allowed disabled:opacity-50 dark:text-gray-200 dark:hover:bg-gray-800;
}
.toolbar-btn--active {
  @apply bg-gray-200 dark:bg-gray-800;
}
</style>
