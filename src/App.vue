<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref } from 'vue'
import Toolbar from './components/Toolbar.vue'
import InputPane from './components/InputPane.vue'
import SplitPane from './components/SplitPane.vue'
import TextView from './components/TextView.vue'
import TreeView from './components/TreeView.vue'
import InfoPanel from './components/InfoPanel.vue'
import ProgressBar from './components/ProgressBar.vue'
import { useJsonEngine } from './composables/useJsonEngine'

const engine = useJsonEngine()
const activeView = ref<'text' | 'tree'>('text')
const inputPaneRef = ref<InstanceType<typeof InputPane> | null>(null)
const isDark = ref(false)
const wrapLines = ref(false)

const validationOk = computed(() => engine.validation.value?.valid ?? null)
const hasContent = computed(() => engine.text.value.length > 0)

async function onFile(file: File) {
  await engine.loadFile(file)
}

function onSearch() {
  inputPaneRef.value?.openSearch()
}

function toggleDark() {
  isDark.value = !isDark.value
  document.documentElement.classList.toggle('dark', isDark.value)
  localStorage.setItem('jsonx-theme', isDark.value ? 'dark' : 'light')
}

onMounted(() => {
  isDark.value =
    localStorage.getItem('jsonx-theme') === 'dark' ||
    (!localStorage.getItem('jsonx-theme') && window.matchMedia('(prefers-color-scheme: dark)').matches)
  document.documentElement.classList.toggle('dark', isDark.value)
})

onBeforeUnmount(() => engine.dispose())
</script>

<template>
  <div class="flex h-screen flex-col">
    <header class="flex items-center justify-between border-b border-gray-200 px-4 py-2 dark:border-gray-800">
      <div class="flex items-baseline gap-2">
        <span class="font-mono text-lg font-semibold">JsonX</span>
        <span class="text-xs text-gray-400">专为大文件打造的高性能 JSON 格式化工具</span>
      </div>
      <button
        class="rounded-md p-1.5 text-sm text-gray-500 hover:bg-gray-100 dark:text-gray-400 dark:hover:bg-gray-800"
        @click="toggleDark"
      >
        {{ isDark ? '☾' : '☀' }}
      </button>
    </header>

    <Toolbar
      :mode="engine.mode.value"
      :is-busy="engine.isBusy.value"
      :active-view="activeView"
      :output-mode="engine.outputMode.value"
      :wrap="wrapLines"
      @set-output-mode="engine.setOutputMode"
      @set-view="(v) => (activeView = v)"
      @search="onSearch"
      @toggle-wrap="wrapLines = !wrapLines"
      @clear="engine.clear"
      @refresh="engine.refreshNow"
    />
    <ProgressBar :active="engine.isBusy.value" />

    <main class="min-h-0 flex-1">
      <SplitPane>
        <template #left>
          <InputPane
            ref="inputPaneRef"
            v-model="engine.text.value"
            :file-name="engine.fileName.value"
            :dark="isDark"
            :wrap="wrapLines"
            @file="onFile"
          />
        </template>
        <template #right>
          <div v-if="!hasContent" class="flex h-full items-center justify-center text-sm text-gray-400">
            格式化结果会显示在这里
          </div>
          <div v-else-if="engine.outputError.value" class="h-full overflow-auto p-4 text-sm text-red-500">
            {{ engine.outputError.value }}
          </div>
          <TextView
            v-else-if="activeView === 'text'"
            :model-value="engine.output.value"
            :dark="isDark"
            :wrap="wrapLines"
            :editable="false"
          />
          <TreeView v-else :text="engine.text.value" :get-children="engine.childrenAt" />
        </template>
      </SplitPane>
    </main>

    <InfoPanel
      v-if="hasContent"
      :file-name="engine.fileName.value"
      :stats="engine.stats.value"
      :validation-ok="validationOk"
    />
  </div>
</template>
