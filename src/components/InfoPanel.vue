<script setup lang="ts">
import type { JsonStatistics } from '@/types'
import { formatBytes } from '@/utils/formatBytes'

defineProps<{
  fileName: string | null
  stats: JsonStatistics | null
  validationOk: boolean | null
}>()
</script>

<template>
  <div
    class="grid grid-cols-2 gap-3 border-t border-gray-200 bg-gray-50 px-4 py-3 text-sm sm:grid-cols-4 lg:grid-cols-7 dark:border-gray-800 dark:bg-gray-900"
  >
    <div class="truncate">
      <div class="text-xs text-gray-400">File</div>
      <div class="truncate font-medium">{{ fileName ?? '(pasted)' }}</div>
    </div>
    <div>
      <div class="text-xs text-gray-400">Size</div>
      <div class="font-medium">{{ stats ? formatBytes(stats.sizeBytes) : '—' }}</div>
    </div>
    <div>
      <div class="text-xs text-gray-400">Status</div>
      <div
        class="font-medium"
        :class="validationOk === false ? 'text-red-500' : validationOk ? 'text-emerald-500' : ''"
      >
        {{ validationOk === null ? '—' : validationOk ? 'Valid JSON' : 'Invalid JSON' }}
      </div>
    </div>
    <div>
      <div class="text-xs text-gray-400">Root</div>
      <div class="font-medium capitalize">{{ stats?.root ?? '—' }}</div>
    </div>
    <div>
      <div class="text-xs text-gray-400">Objects</div>
      <div class="font-medium">{{ stats?.objects.toLocaleString() ?? '—' }}</div>
    </div>
    <div>
      <div class="text-xs text-gray-400">Arrays</div>
      <div class="font-medium">{{ stats?.arrays.toLocaleString() ?? '—' }}</div>
    </div>
    <div>
      <div class="text-xs text-gray-400">Max Depth</div>
      <div class="font-medium">{{ stats?.maxDepth ?? '—' }}</div>
    </div>
  </div>
</template>
