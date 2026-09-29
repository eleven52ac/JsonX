<script setup lang="ts">
import { ref } from 'vue'

const MIN_PERCENT = 20
const MAX_PERCENT = 80

const leftPercent = ref(50)
const isDragging = ref(false)
const container = ref<HTMLDivElement | null>(null)

function onPointerDown(event: PointerEvent) {
  isDragging.value = true
  ;(event.currentTarget as HTMLElement).setPointerCapture(event.pointerId)
}

function onPointerMove(event: PointerEvent) {
  if (!isDragging.value || !container.value) return
  const rect = container.value.getBoundingClientRect()
  const percent = ((event.clientX - rect.left) / rect.width) * 100
  leftPercent.value = Math.min(MAX_PERCENT, Math.max(MIN_PERCENT, percent))
}

function onPointerUp() {
  isDragging.value = false
}
</script>

<template>
  <div ref="container" class="flex h-full min-h-0 w-full" :class="{ 'select-none': isDragging }">
    <div class="min-w-0 overflow-hidden" :style="{ width: `${leftPercent}%` }">
      <slot name="left" />
    </div>
    <div
      class="w-1 shrink-0 cursor-col-resize bg-gray-200 transition-colors hover:bg-blue-400 active:bg-blue-500 dark:bg-gray-800 dark:hover:bg-blue-500"
      @pointerdown="onPointerDown"
      @pointermove="onPointerMove"
      @pointerup="onPointerUp"
    />
    <div class="min-w-0 flex-1 overflow-hidden">
      <slot name="right" />
    </div>
  </div>
</template>
