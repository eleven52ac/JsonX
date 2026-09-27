<script setup lang="ts">
import { ref } from 'vue'
import type { TreeNode } from '@/types'

const PAGE_SIZE = 100

const props = defineProps<{
  node: TreeNode
  path: (string | number)[]
  getChildren: (path: (string | number)[]) => Promise<TreeNode[]>
}>()

const expanded = ref(false)
const children = ref<TreeNode[]>([])
const visibleCount = ref(PAGE_SIZE)
const isLoading = ref(false)

const isExpandable = props.node.childCount > 0

async function toggle() {
  if (!isExpandable) return
  expanded.value = !expanded.value
  if (expanded.value && children.value.length === 0) {
    isLoading.value = true
    try {
      children.value = await props.getChildren(props.path)
    } finally {
      isLoading.value = false
    }
  }
}

function loadMore() {
  visibleCount.value += PAGE_SIZE
}
</script>

<template>
  <div>
    <div
      class="flex cursor-pointer items-center gap-1.5 rounded px-1 py-0.5 font-mono text-sm hover:bg-gray-100 dark:hover:bg-gray-800"
      :style="{ paddingLeft: `${node.depth * 16}px` }"
      @click="toggle"
    >
      <span class="w-4 shrink-0 select-none text-gray-400">
        <template v-if="isExpandable">{{ expanded ? '▼' : '▶' }}</template>
      </span>
      <span v-if="node.key !== null" class="text-sky-700 dark:text-sky-400">{{ node.key }}:</span>
      <span class="truncate text-gray-600 dark:text-gray-300">{{ node.preview }}</span>
    </div>

    <div v-if="expanded">
      <p v-if="isLoading" class="px-2 py-1 text-xs text-gray-400" :style="{ paddingLeft: `${(node.depth + 1) * 16}px` }">
        Loading…
      </p>
      <TreeNodeRow
        v-for="child in children.slice(0, visibleCount)"
        :key="child.id"
        :node="child"
        :path="[...path, child.key as string | number]"
        :get-children="getChildren"
      />
      <button
        v-if="children.length > visibleCount"
        class="px-2 py-1 text-xs text-blue-600 hover:underline dark:text-blue-400"
        :style="{ paddingLeft: `${(node.depth + 1) * 16}px` }"
        @click.stop="loadMore"
      >
        Load {{ Math.min(PAGE_SIZE, children.length - visibleCount) }} more…
      </button>
    </div>
  </div>
</template>
