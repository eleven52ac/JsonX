import { ref, shallowRef } from 'vue'
import type { JsonStatistics, TreeNode, ValidationResult, WorkerRequest, WorkerResponse, WorkMode } from '@/types'
import { LARGE_FILE_THRESHOLD_BYTES } from '@/types'

let requestId = 0
const pending = new Map<number, (res: WorkerResponse) => void>()

function createWorker(): Worker {
  return new Worker(new URL('../worker/json.worker.ts', import.meta.url), { type: 'module' })
}

/**
 * Owns the JSON text buffer, the mode (normal vs. large-file), and all
 * communication with the background worker. UI components should read/act
 * through this composable rather than talking to the worker directly.
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

  function loadText(next: string, name: string | null = null) {
    text.value = next
    fileName.value = name
    mode.value = new TextEncoder().encode(next).length >= LARGE_FILE_THRESHOLD_BYTES ? 'large' : 'normal'
    validation.value = null
    stats.value = null
  }

  async function loadFile(file: File) {
    fileName.value = file.name
    mode.value = file.size >= LARGE_FILE_THRESHOLD_BYTES ? 'large' : 'normal'
    text.value = await file.text()
    validation.value = null
    stats.value = null
  }

  async function format() {
    isBusy.value = true
    try {
      const res = await send<{ id: number; type: 'format-result'; text: string }>({
        id: ++requestId,
        type: 'format',
        text: text.value,
      })
      text.value = res.text
    } finally {
      isBusy.value = false
    }
  }

  async function minify() {
    isBusy.value = true
    try {
      const res = await send<{ id: number; type: 'minify-result'; text: string }>({
        id: ++requestId,
        type: 'minify',
        text: text.value,
      })
      text.value = res.text
    } finally {
      isBusy.value = false
    }
  }

  async function validate() {
    isBusy.value = true
    try {
      const res = await send<{ id: number; type: 'validate-result'; result: ValidationResult }>({
        id: ++requestId,
        type: 'validate',
        text: text.value,
      })
      validation.value = res.result
      return res.result
    } finally {
      isBusy.value = false
    }
  }

  async function loadStats() {
    isBusy.value = true
    try {
      const res = await send<{ id: number; type: 'stats-result'; stats: JsonStatistics }>({
        id: ++requestId,
        type: 'stats',
        text: text.value,
      })
      stats.value = res.stats
      return res.stats
    } finally {
      isBusy.value = false
    }
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
    text.value = ''
    fileName.value = null
    mode.value = 'normal'
    validation.value = null
    stats.value = null
  }

  function dispose() {
    worker.value.terminate()
  }

  return {
    text,
    fileName,
    mode,
    isBusy,
    validation,
    stats,
    loadText,
    loadFile,
    format,
    minify,
    validate,
    loadStats,
    childrenAt,
    clear,
    dispose,
  }
}
