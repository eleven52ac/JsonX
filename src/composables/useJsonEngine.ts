import { ref, shallowRef, watch } from 'vue'
import type { JsonStatistics, TreeNode, ValidationResult, WorkerRequest, WorkerResponse, WorkMode } from '@/types'
import { LARGE_FILE_THRESHOLD_BYTES } from '@/types'

let requestId = 0
const pending = new Map<number, (res: WorkerResponse) => void>()

function createWorker(): Worker {
  return new Worker(new URL('../worker/json.worker.ts', import.meta.url), { type: 'module' })
}

export type OutputMode = 'formatted' | 'minified'

const REFRESH_DEBOUNCE_MS = 250

/**
 * Owns the source JSON text (left pane), a derived output (right pane —
 * formatted or minified, recomputed automatically as the source changes),
 * and all communication with the background worker. UI components should
 * read/act through this composable rather than talking to the worker
 * directly.
 */
export function useJsonEngine() {
  const worker = shallowRef<Worker>(createWorker())
  worker.value.onmessage = (event: MessageEvent<WorkerResponse>) => {
    const res = event.data
    const resolve = pending.get(res.id)
    if (resolve) {
      pending.delete(res.id)
      resolve(res)
    }
  }

  function send<T extends WorkerResponse>(req: WorkerRequest): Promise<T> {
    return new Promise((resolve, reject) => {
      pending.set(req.id, (res) => {
        if (res.type === 'error') reject(new Error(res.message))
        else resolve(res as T)
      })
      worker.value.postMessage(req)
    })
  }

  const text = ref('')
  const fileName = ref<string | null>(null)
  const mode = ref<WorkMode>('normal')
  const isBusy = ref(false)
  const validation = ref<ValidationResult | null>(null)
  const stats = ref<JsonStatistics | null>(null)
  const outputMode = ref<OutputMode>('formatted')
  const output = ref('')
  const outputError = ref<string | null>(null)

  let debounceTimer: ReturnType<typeof setTimeout> | null = null

  async function refreshAll() {
    if (!text.value.trim()) {
      validation.value = null
      stats.value = null
      output.value = ''
      outputError.value = null
      return
    }

    isBusy.value = true
    try {
      const [validateRes, statsRes] = await Promise.all([
        send<{ id: number; type: 'validate-result'; result: ValidationResult }>({
          id: ++requestId,
          type: 'validate',
          text: text.value,
        }),
        send<{ id: number; type: 'stats-result'; stats: JsonStatistics }>({
          id: ++requestId,
          type: 'stats',
          text: text.value,
        }),
      ])
      validation.value = validateRes.result
      stats.value = statsRes.stats

      const outRes = await send<{ id: number; type: 'format-result' | 'minify-result'; text: string }>({
        id: ++requestId,
        type: outputMode.value === 'formatted' ? 'format' : 'minify',
        text: text.value,
      })
      output.value = outRes.text
      outputError.value = null
    } catch (err) {
      outputError.value = err instanceof Error ? err.message : String(err)
    } finally {
      isBusy.value = false
    }
  }

  function scheduleRefresh() {
    if (debounceTimer) clearTimeout(debounceTimer)
    debounceTimer = setTimeout(() => {
      debounceTimer = null
      void refreshAll()
    }, REFRESH_DEBOUNCE_MS)
  }

  function refreshNow() {
    if (debounceTimer) {
      clearTimeout(debounceTimer)
      debounceTimer = null
    }
    return refreshAll()
  }

  watch(text, scheduleRefresh)
  watch(outputMode, refreshNow)

  function loadText(next: string, name: string | null = null) {
    text.value = next
    fileName.value = name
    mode.value = new TextEncoder().encode(next).length >= LARGE_FILE_THRESHOLD_BYTES ? 'large' : 'normal'
  }

  async function loadFile(file: File) {
    fileName.value = file.name
    mode.value = file.size >= LARGE_FILE_THRESHOLD_BYTES ? 'large' : 'normal'
    text.value = await file.text()
  }

  function setOutputMode(next: OutputMode) {
    outputMode.value = next
  }

  async function childrenAt(path: (string | number)[]): Promise<TreeNode[]> {
    const res = await send<{ id: number; type: 'tree-children-result'; nodes: TreeNode[] }>({
      id: ++requestId,
      type: 'tree-children',
      text: text.value,
      path,
    })
    return res.nodes
  }

  function clear() {
    if (debounceTimer) {
      clearTimeout(debounceTimer)
      debounceTimer = null
    }
    text.value = ''
    fileName.value = null
    mode.value = 'normal'
    validation.value = null
    stats.value = null
    output.value = ''
    outputError.value = null
  }

  function dispose() {
    if (debounceTimer) clearTimeout(debounceTimer)
    worker.value.terminate()
  }

  return {
    text,
    fileName,
    mode,
    isBusy,
    validation,
    stats,
    outputMode,
    output,
    outputError,
    loadText,
    loadFile,
    setOutputMode,
    refreshNow,
    childrenAt,
    clear,
    dispose,
  }
}
