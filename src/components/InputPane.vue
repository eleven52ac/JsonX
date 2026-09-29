<script setup lang="ts">
import { ref } from 'vue'
import TextView from './TextView.vue'

const props = defineProps<{
  modelValue: string
  fileName: string | null
  dark?: boolean
  wrap?: boolean
}>()

const emit = defineEmits<{
  'update:modelValue': [value: string]
  file: [file: File]
}>()

const fileInput = ref<HTMLInputElement | null>(null)
const textViewRef = ref<InstanceType<typeof TextView> | null>(null)

// Capture-phase so a dropped *file* never reaches CodeMirror's own drop
// handler (which would otherwise insert the raw drag data as text).
function onDropCapture(event: DragEvent) {
  const file = event.dataTransfer?.files?.[0]
  if (!file) return
  event.preventDefault()
  event.stopPropagation()
  emit('file', file)
}

function onPick(event: Event) {
  const input = event.target as HTMLInputElement
  const file = input.files?.[0]
  if (file) emit('file', file)
  input.value = ''
}

defineExpose({
  openSearch: () => textViewRef.value?.openSearch(),
})
</script>

<template>
  <div class="flex h-full flex-col">
    <div
      class="flex shrink-0 items-center justify-between border-b border-gray-200 px-3 py-1.5 text-xs text-gray-500 dark:border-gray-800 dark:text-gray-400"
    >
      <span class="truncate">{{ fileName ?? '粘贴的内容' }}</span>
      <button class="rounded px-2 py-0.5 font-medium hover:bg-gray-200 dark:hover:bg-gray-800" @click="fileInput?.click()">
        选择文件
      </button>
      <input ref="fileInput" type="file" accept=".json,.txt" class="hidden" @change="onPick" />
    </div>

    <div class="relative min-h-0 flex-1" @dragover.prevent @drop.capture="onDropCapture">
      <TextView
        ref="textViewRef"
        :model-value="props.modelValue"
        :dark="props.dark"
        :wrap="props.wrap"
        @update:model-value="emit('update:modelValue', $event)"
      />
      <div v-if="!props.modelValue" class="pointer-events-none absolute inset-0 flex flex-col items-center justify-center gap-3 text-center">
        <div class="text-4xl">{ }</div>
        <p class="text-lg font-medium text-gray-700 dark:text-gray-200">将 JSON 文件拖拽到此处</p>
        <p class="text-sm text-gray-500 dark:text-gray-400">或点击此处使用 Ctrl+V 粘贴 — 数据不会离开你的浏览器</p>
        <button
          class="pointer-events-auto rounded-md bg-gray-900 px-4 py-2 text-sm font-medium text-white hover:bg-gray-700 dark:bg-white dark:text-gray-900 dark:hover:bg-gray-200"
          @click="fileInput?.click()"
        >
          选择文件
        </button>
        <span class="text-xs text-gray-400">支持 .json / .txt</span>
      </div>
    </div>
  </div>
</template>
