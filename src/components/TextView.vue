<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref, watch } from 'vue'
import { Compartment, EditorState } from '@codemirror/state'
import { EditorView, keymap, lineNumbers, highlightActiveLine } from '@codemirror/view'
import { defaultKeymap, history, historyKeymap } from '@codemirror/commands'
import { openSearchPanel, search, searchKeymap } from '@codemirror/search'
import { json } from '@codemirror/lang-json'
import { oneDark } from '@codemirror/theme-one-dark'

const props = defineProps<{
  modelValue: string
  dark?: boolean
}>()

const emit = defineEmits<{
  'update:modelValue': [value: string]
}>()

const host = ref<HTMLDivElement | null>(null)
let view: EditorView | null = null
let applyingExternalChange = false
const themeCompartment = new Compartment()

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
        themeCompartment.of(props.dark ? oneDark : []),
        keymap.of([...defaultKeymap, ...historyKeymap, ...searchKeymap]),
        EditorView.lineWrapping,
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
    view?.dispatch({ effects: themeCompartment.reconfigure(dark ? oneDark : []) })
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
