import type { JsonStatistics, RootKind, TreeNode, ValidationResult } from '@/types'

/**
 * The engine boundary the worker talks to. `rust-core` (compiled to WASM via
 * `npm run build:wasm`) is meant to implement this same shape for the
 * streaming/large-file path — see rust-core/src for the module layout
 * (tokenizer/parser/formatter/validator/minifier/indexer/statistics).
 *
 * Until the WASM build is wired in, `jsFallbackEngine` below covers normal
 * mode using native JSON.parse/stringify so the app is usable end to end.
 */
export interface JsonEngine {
  format(text: string): string
  minify(text: string): string
  validate(text: string): ValidationResult
  stats(text: string): JsonStatistics
  childrenAt(text: string, path: (string | number)[]): TreeNode[]
}

function rootKindOf(value: unknown): RootKind {
  if (value === null) return 'null'
  if (Array.isArray(value)) return 'array'
  switch (typeof value) {
    case 'object':
      return 'object'
    case 'string':
      return 'string'
    case 'number':
      return 'number'
    case 'boolean':
      return 'boolean'
    default:
      return 'unknown'
  }
}

function locateError(text: string, err: unknown): ValidationResult {
  const message = err instanceof Error ? err.message : String(err)
  const match = /position (\d+)/.exec(message)
  const offset = match ? Number(match[1]) : 0
  let line = 1
  let column = 1
  for (let i = 0; i < offset && i < text.length; i++) {
    if (text[i] === '\n') {
      line++
      column = 1
    } else {
      column++
    }
  }
  return { valid: false, error: { message, line, column, offset } }
}

function walkStats(value: unknown, depth: number, acc: { objects: number; arrays: number; maxDepth: number }) {
  acc.maxDepth = Math.max(acc.maxDepth, depth)
  if (Array.isArray(value)) {
    acc.arrays++
    for (const item of value) walkStats(item, depth + 1, acc)
  } else if (value !== null && typeof value === 'object') {
    acc.objects++
    for (const key of Object.keys(value as Record<string, unknown>)) {
      walkStats((value as Record<string, unknown>)[key], depth + 1, acc)
    }
  }
}

function preview(value: unknown): string {
  if (value === null) return 'null'
  if (Array.isArray(value)) return `Array[${value.length}]`
  if (typeof value === 'object') return `Object{${Object.keys(value as object).length}}`
  if (typeof value === 'string') return JSON.stringify(value.length > 80 ? `${value.slice(0, 80)}…` : value)
  return String(value)
}

function resolvePath(root: unknown, path: (string | number)[]): unknown {
  let current = root
  for (const segment of path) {
    if (current === null || typeof current !== 'object') return undefined
    current = (current as Record<string | number, unknown>)[segment as never]
  }
  return current
}

export const jsFallbackEngine: JsonEngine = {
  format(text: string): string {
    return JSON.stringify(JSON.parse(text), null, 2)
  },

  minify(text: string): string {
    return JSON.stringify(JSON.parse(text))
  },

  validate(text: string): ValidationResult {
    try {
      JSON.parse(text)
      return { valid: true, error: null }
    } catch (err) {
      return locateError(text, err)
    }
  },

  stats(text: string): JsonStatistics {
    const start = performance.now()
    const parsed = JSON.parse(text)
    const acc = { objects: 0, arrays: 0, maxDepth: 0 }
    walkStats(parsed, 1, acc)
    return {
      sizeBytes: text.length,
      root: rootKindOf(parsed),
      objects: acc.objects,
      arrays: acc.arrays,
      maxDepth: acc.maxDepth,
      parseTimeMs: performance.now() - start,
    }
  },

  childrenAt(text: string, path: (string | number)[]): TreeNode[] {
    const root = JSON.parse(text)
    const target = resolvePath(root, path)
    if (target === null || typeof target !== 'object') return []

    const entries: [string | number, unknown][] = Array.isArray(target)
      ? target.map((v, i) => [i, v])
      : Object.entries(target as Record<string, unknown>)

    return entries.map(([key, value]) => ({
      id: [...path, key].join('.'),
      key,
      valueKind: rootKindOf(value),
      preview: preview(value),
      childCount:
        Array.isArray(value) ? value.length : value !== null && typeof value === 'object' ? Object.keys(value).length : 0,
      depth: path.length + 1,
      offset: 0,
    }))
  },
}
