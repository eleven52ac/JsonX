<script setup lang="ts">
import type { JsonStatistics, RootKind } from '@/types'
import { formatBytes } from '@/utils/formatBytes'

defineProps<{
  fileName: string | null
  stats: JsonStatistics | null
  validationOk: boolean | null
}>()

const rootKindLabels: Record<RootKind, string> = {
  object: '对象',
  array: '数组',
  string: '字符串',
  number: '数字',
  boolean: '布尔值',
  null: '空值',
  unknown: '未知',
}
</script>

<template>
  <div
    class="grid grid-cols-2 gap-3 border-t border-gray-200 bg-gray-50 px-4 py-3 text-sm sm:grid-cols-4 lg:grid-cols-7 dark:border-gray-800 dark:bg-gray-900"
  >
    <div class="truncate">
      <div class="text-xs text-gray-400">文件</div>
      <div class="truncate font-medium">{{ fileName ?? '(粘贴内容)' }}</div>
    </div>
    <div>
      <div class="text-xs text-gray-400">大小</div>
      <div class="font-medium">{{ stats ? formatBytes(stats.sizeBytes) : '—' }}</div>
    </div>
    <div>
      <div class="text-xs text-gray-400">状态</div>
      <div
        class="font-medium"
        :class="validationOk === false ? 'text-red-500' : validationOk ? 'text-emerald-500' : ''"
      >
        {{ validationOk === null ? '—' : validationOk ? '有效 JSON' : '无效 JSON' }}
      </div>
    </div>
    <div>
      <div class="text-xs text-gray-400">根类型</div>
      <div class="font-medium">{{ stats ? rootKindLabels[stats.root] : '—' }}</div>
    </div>
    <div>
      <div class="text-xs text-gray-400">对象数</div>
      <div class="font-medium">{{ stats?.objects.toLocaleString() ?? '—' }}</div>
    </div>
    <div>
      <div class="text-xs text-gray-400">数组数</div>
      <div class="font-medium">{{ stats?.arrays.toLocaleString() ?? '—' }}</div>
    </div>
    <div>
      <div class="text-xs text-gray-400">最大深度</div>
      <div class="font-medium">{{ stats?.maxDepth ?? '—' }}</div>
    </div>
  </div>
</template>
