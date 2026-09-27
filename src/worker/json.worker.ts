import type { WorkerRequest, WorkerResponse } from '@/types'
import { jsFallbackEngine } from './engine'

// Normal mode runs entirely inside this worker so the main thread never
// blocks on JSON.parse/stringify. Large File Mode will swap this engine for
// the streaming Rust/WASM core (see src/worker/engine.ts and rust-core/).
const engine = jsFallbackEngine

self.onmessage = (event: MessageEvent<WorkerRequest>) => {
  const req = event.data
  try {
    switch (req.type) {
      case 'format':
        respond({ id: req.id, type: 'format-result', text: engine.format(req.text) })
        break
      case 'minify':
        respond({ id: req.id, type: 'minify-result', text: engine.minify(req.text) })
        break
      case 'validate':
        respond({ id: req.id, type: 'validate-result', result: engine.validate(req.text) })
        break
      case 'stats':
        respond({ id: req.id, type: 'stats-result', stats: engine.stats(req.text) })
        break
      case 'tree-children':
        respond({
          id: req.id,
          type: 'tree-children-result',
          nodes: engine.childrenAt(req.text, req.path),
        })
        break
    }
  } catch (err) {
    respond({ id: req.id, type: 'error', message: err instanceof Error ? err.message : String(err) })
  }
}

function respond(res: WorkerResponse) {
  self.postMessage(res)
}
