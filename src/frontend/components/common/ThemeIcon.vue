<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  theme: string
}

const props = defineProps<Props>()

const iconClass = computed(() => {
  return props.theme === 'light' ? 'i-carbon-sun' : 'i-carbon-moon'
})
</script>

<template>
  <div
    class="inline-flex items-center justify-center w-4 h-4 transition-transform duration-300 hover:rotate-180"
    :class="[iconClass]"
  />
</template>
