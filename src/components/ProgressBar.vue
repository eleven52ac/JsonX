<script setup lang="ts">
defineProps<{
  active: boolean
}>()
</script>

<template>
  <div v-if="active" class="h-0.5 w-full overflow-hidden bg-gray-100 dark:bg-gray-800">
    <div class="h-full w-1/3 animate-[progress_1.2s_ease-in-out_infinite] bg-blue-500" />
  </div>
</template>

<style scoped>
@keyframes progress {
  0% {
    transform: translateX(-100%);
  }
  100% {
    transform: translateX(300%);
  }
}
</style>
