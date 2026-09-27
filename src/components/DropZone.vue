<script setup lang="ts">
import { ref } from 'vue'

const emit = defineEmits<{
  file: [file: File]
  paste: [text: string]
}>()

const isDragging = ref(false)
const fileInput = ref<HTMLInputElement | null>(null)

function onDrop(event: DragEvent) {
  isDragging.value = false
  const file = event.dataTransfer?.files?.[0]
  if (file) emit('file', file)
}

function onPick(event: Event) {
  const file = (event.target as HTMLInputElement).files?.[0]
  if (file) emit('file', file)
}

async function onPaste(event: ClipboardEvent) {
  const text = event.clipboardData?.getData('text')
  if (text) emit('paste', text)
}
</script>

<template>
  <div
    class="flex h-full flex-col items-center justify-center gap-4 p-12 text-center"
    :class="isDragging ? 'bg-blue-50 dark:bg-blue-950/30' : ''"
    @dragover.prevent="isDragging = true"
    @dragleave.prevent="isDragging = false"
    @drop.prevent="onDrop"
    @paste="onPaste"
  >
    <div class="text-5xl">{ }</div>
    <p class="text-lg font-medium text-gray-700 dark:text-gray-200">Drop a JSON file here</p>
    <p class="text-sm text-gray-500 dark:text-gray-400">or paste with Ctrl+V — nothing leaves your browser</p>

    <div class="flex items-center gap-3">
      <button
        class="rounded-md bg-gray-900 px-4 py-2 text-sm font-medium text-white hover:bg-gray-700 dark:bg-white dark:text-gray-900 dark:hover:bg-gray-200"
        @click="fileInput?.click()"
      >
        Choose file
      </button>
      <span class="text-xs text-gray-400">.json / .txt</span>
    </div>

    <input ref="fileInput" type="file" accept=".json,.txt" class="hidden" @change="onPick" />
  </div>
</template>
