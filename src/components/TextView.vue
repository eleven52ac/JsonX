<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref, watch } from 'vue'
import { Compartment, EditorState } from '@codemirror/state'
import { EditorView, keymap, lineNumbers, highlightActiveLine } from '@codemirror/view'
import { defaultKeymap, history, historyKeymap } from '@codemirror/commands'
import { openSearchPanel, search, searchKeymap } from '@codemirror/search'
import { json } from '@codemirror/lang-json'
import { syntaxHighlighting } from '@codemirror/language'
import { oneDarkTheme } from '@codemirror/theme-one-dark'
import { jsonDarkHighlight, jsonLightHighlight } from '@/utils/jsonHighlightStyle'

const lightExtensions = syntaxHighlighting(jsonLightHighlight)
const darkExtensions = [oneDarkTheme, syntaxHighlighting(jsonDarkHighlight)]

const props = withDefaults(
  defineProps<{
    modelValue: string
    dark?: boolean
    wrap?: boolean
    editable?: boolean
  }>(),
  { editable: true },
)

const emit = defineEmits<{
  'update:modelValue': [value: string]
}>()

const host = ref<HTMLDivElement | null>(null)
let view: EditorView | null = null
let applyingExternalChange = false
const themeCompartment = new Compartment()
const wrapCompartment = new Compartment()

onMounted(() => {
  view = new EditorView({
    parent: host.value!,
    state: EditorState.create({
      doc: props.modelValue,
      extensions: [
        lineNumbers(),
        highlightActiveLine(),
        history(),
        search(),
        json(),
        themeCompartment.of(props.dark ? darkExtensions : lightExtensions),
        wrapCompartment.of(props.wrap ? EditorView.lineWrapping : []),
        keymap.of([...defaultKeymap, ...historyKeymap, ...searchKeymap]),
        EditorView.editable.of(props.editable),
        EditorState.readOnly.of(!props.editable),
        EditorView.updateListener.of((update) => {
          if (update.docChanged && !applyingExternalChange) {
            emit('update:modelValue', update.state.doc.toString())
          }
        }),
      ],
    }),
  })
})

watch(
  () => props.modelValue,
  (next) => {
    if (!view) return
    const current = view.state.doc.toString()
    if (next === current) return
    applyingExternalChange = true
    view.dispatch({ changes: { from: 0, to: current.length, insert: next } })
    applyingExternalChange = false
  },
)

watch(
  () => props.dark,
  (dark) => {
    view?.dispatch({ effects: themeCompartment.reconfigure(dark ? darkExtensions : lightExtensions) })
  },
)

watch(
  () => props.wrap,
  (wrap) => {
    view?.dispatch({ effects: wrapCompartment.reconfigure(wrap ? EditorView.lineWrapping : []) })
  },
)

onBeforeUnmount(() => view?.destroy())

defineExpose({
  openSearch: () => view && openSearchPanel(view),
})
</script>

<template>
  <div ref="host" class="h-full overflow-auto text-sm" />
</template>
