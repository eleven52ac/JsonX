<script setup lang="ts">
import { ref, watch } from 'vue'
import type { TreeNode } from '@/types'
import TreeNodeRow from './TreeNodeRow.vue'

const props = defineProps<{
  text: string
  getChildren: (path: (string | number)[]) => Promise<TreeNode[]>
}>()

const rootChildren = ref<TreeNode[]>([])
const isLoading = ref(false)
const errorMessage = ref<string | null>(null)

async function loadRoot() {
  if (!props.text.trim()) {
    rootChildren.value = []
    return
  }
  isLoading.value = true
  errorMessage.value = null
  try {
    rootChildren.value = await props.getChildren([])
  } catch (err) {
    errorMessage.value = err instanceof Error ? err.message : String(err)
  } finally {
    isLoading.value = false
  }
}

watch(() => props.text, loadRoot, { immediate: true })
</script>

<template>
  <div class="h-full overflow-auto p-2">
    <p v-if="isLoading" class="p-2 text-sm text-gray-400">加载中…</p>
    <p v-else-if="errorMessage" class="p-2 text-sm text-red-500">{{ errorMessage }}</p>
    <p v-else-if="rootChildren.length === 0" class="p-2 text-sm text-gray-400">暂无内容。</p>
    <template v-else>
      <TreeNodeRow
        v-for="child in rootChildren"
        :key="child.id"
        :node="child"
        :path="[child.key as string | number]"
        :get-children="getChildren"
      />
    </template>
  </div>
</template>
