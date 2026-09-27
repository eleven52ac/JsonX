export type WorkMode = 'normal' | 'large'

/** Files at or above this size switch the workspace into Large File Mode. */
export const LARGE_FILE_THRESHOLD_BYTES = 20 * 1024 * 1024 // 20 MB

export type RootKind = 'object' | 'array' | 'string' | 'number' | 'boolean' | 'null' | 'unknown'

export interface JsonStatistics {
  sizeBytes: number
  root: RootKind
  objects: number
  arrays: number
  maxDepth: number
  parseTimeMs: number
}

export interface ValidationError {
  message: string
  line: number
  column: number
  offset: number
}

export interface ValidationResult {
  valid: boolean
  error: ValidationError | null
}

/** A lazily-materialized node in the Tree View. Children are loaded on demand. */
export interface TreeNode {
  id: string
  key: string | number | null
  valueKind: RootKind
  preview: string
  childCount: number
  depth: number
  /** Byte offset into the source text, used to jump the Text View to this node. */
  offset: number
}

export type EngineStage = 'reading' | 'parsing' | 'indexing' | 'formatting'

export interface EngineProgress {
  stage: EngineStage
  loadedBytes: number
  totalBytes: number
  ratio: number
}

// ---- Worker message protocol ----------------------------------------------

export type WorkerRequest =
  | { id: number; type: 'format'; text: string }
  | { id: number; type: 'minify'; text: string }
  | { id: number; type: 'validate'; text: string }
  | { id: number; type: 'stats'; text: string }
  | { id: number; type: 'tree-children'; text: string; path: (string | number)[] }

export type WorkerResponse =
  | { id: number; type: 'progress'; progress: EngineProgress }
  | { id: number; type: 'format-result'; text: string }
  | { id: number; type: 'minify-result'; text: string }
  | { id: number; type: 'validate-result'; result: ValidationResult }
  | { id: number; type: 'stats-result'; stats: JsonStatistics }
  | { id: number; type: 'tree-children-result'; nodes: TreeNode[] }
  | { id: number; type: 'error'; message: string }
