<script setup lang="ts">
import type { WorkMode } from '@/types'

defineProps<{
  mode: WorkMode
  isBusy: boolean
  activeView: 'text' | 'tree'
}>()

const emit = defineEmits<{
  format: []
  minify: []
  validate: []
  clear: []
  'set-view': [view: 'text' | 'tree']
  search: []
}>()
</script>

<template>
  <div
    class="flex items-center gap-1 border-b border-gray-200 bg-gray-50 px-3 py-2 dark:border-gray-800 dark:bg-gray-900"
  >
    <button class="toolbar-btn" :disabled="isBusy" @click="emit('format')">Format</button>
    <button class="toolbar-btn" :disabled="isBusy" @click="emit('minify')">Minify</button>
    <button class="toolbar-btn" :disabled="isBusy" @click="emit('validate')">Validate</button>

    <div class="mx-1 h-5 w-px bg-gray-300 dark:bg-gray-700" />

    <button
      class="toolbar-btn"
      :class="{ 'toolbar-btn--active': activeView === 'text' }"
      @click="emit('set-view', 'text')"
    >
      Text
    </button>
    <button
      class="toolbar-btn"
      :class="{ 'toolbar-btn--active': activeView === 'tree' }"
      @click="emit('set-view', 'tree')"
    >
      Tree
    </button>
    <button class="toolbar-btn" @click="emit('search')">Search</button>

    <div class="flex-1" />

    <span
      v-if="mode === 'large'"
      class="rounded-full bg-amber-100 px-2 py-0.5 text-xs font-medium text-amber-800 dark:bg-amber-900/40 dark:text-amber-300"
    >
      ⚡ Large File Mode
    </span>

    <button class="toolbar-btn" @click="emit('clear')">Clear</button>
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
