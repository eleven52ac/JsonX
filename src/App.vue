<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref } from 'vue'
import Toolbar from './components/Toolbar.vue'
import DropZone from './components/DropZone.vue'
import TextView from './components/TextView.vue'
import TreeView from './components/TreeView.vue'
import InfoPanel from './components/InfoPanel.vue'
import ProgressBar from './components/ProgressBar.vue'
import { useJsonEngine } from './composables/useJsonEngine'

const engine = useJsonEngine()
const activeView = ref<'text' | 'tree'>('text')
const textViewRef = ref<InstanceType<typeof TextView> | null>(null)
const isDark = ref(false)

const validationOk = computed(() => engine.validation.value?.valid ?? null)
const hasContent = computed(() => engine.text.value.length > 0)

async function refreshDiagnostics() {
  await Promise.all([engine.validate(), engine.loadStats()])
}

async function onFile(file: File) {
  await engine.loadFile(file)
  await refreshDiagnostics()
}

async function onPaste(text: string) {
  engine.loadText(text)
  await refreshDiagnostics()
}

async function onFormat() {
  await engine.format()
  await refreshDiagnostics()
}

async function onMinify() {
  await engine.minify()
  await refreshDiagnostics()
}

function onSearch() {
  if (activeView.value === 'text') textViewRef.value?.openSearch()
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
        <span class="text-xs text-gray-400">A fast JSON formatter built for large files</span>
      </div>
      <button
        class="rounded-md p-1.5 text-sm text-gray-500 hover:bg-gray-100 dark:text-gray-400 dark:hover:bg-gray-800"
        @click="toggleDark"
      >
        {{ isDark ? '☾' : '☀' }}
      </button>
    </header>

    <Toolbar
      v-if="hasContent"
      :mode="engine.mode.value"
      :is-busy="engine.isBusy.value"
      :active-view="activeView"
      @format="onFormat"
      @minify="onMinify"
      @validate="engine.validate"
      @clear="engine.clear"
      @set-view="(v) => (activeView = v)"
      @search="onSearch"
    />
    <ProgressBar :active="engine.isBusy.value" />

    <main class="min-h-0 flex-1">
      <DropZone v-if="!hasContent" @file="onFile" @paste="onPaste" />
      <TextView v-else-if="activeView === 'text'" ref="textViewRef" v-model="engine.text.value" :dark="isDark" />
      <TreeView v-else :text="engine.text.value" :get-children="engine.childrenAt" />
    </main>

    <InfoPanel v-if="hasContent" :file-name="engine.fileName.value" :stats="engine.stats.value" :validation-ok="validationOk" />
  </div>
</template>
