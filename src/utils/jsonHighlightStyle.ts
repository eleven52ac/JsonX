import { HighlightStyle } from '@codemirror/language'
import { tags as t } from '@lezer/highlight'

/**
 * Color scheme scoped to exactly the tags @lezer/json emits (see
 * @lezer/json/src/highlight.js): keys, strings, numbers, booleans and null
 * each get their own hue so they scan at a glance; punctuation/brackets stay
 * a muted gray so structure recedes behind the data.
 */
export const jsonLightHighlight = HighlightStyle.define([
  { tag: t.propertyName, color: '#0369a1', fontWeight: 600 }, // sky-700
  { tag: t.string, color: '#047857' }, // emerald-700
  { tag: t.number, color: '#b45309' }, // amber-700
  { tag: t.bool, color: '#e11d48' }, // rose-600
  { tag: t.null, color: '#64748b', fontStyle: 'italic' }, // slate-500
  { tag: [t.separator, t.squareBracket, t.brace], color: '#94a3b8' }, // slate-400
])

export const jsonDarkHighlight = HighlightStyle.define([
  { tag: t.propertyName, color: '#7dd3fc', fontWeight: 600 }, // sky-300
  { tag: t.string, color: '#6ee7b7' }, // emerald-300
  { tag: t.number, color: '#fcd34d' }, // amber-300
  { tag: t.bool, color: '#fda4af' }, // rose-300
  { tag: t.null, color: '#94a3b8', fontStyle: 'italic' }, // slate-400
  { tag: [t.separator, t.squareBracket, t.brace], color: '#64748b' }, // slate-500
])
