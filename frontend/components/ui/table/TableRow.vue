<script setup lang="ts">
import type { HTMLAttributes } from 'vue'
import { cn } from '@/lib/utils'

const props = defineProps<{
  class?: HTMLAttributes['class']
  onClick?: HTMLAttributes['onClick']
  isSelected?: boolean
}>()

</script>


<template>
  <tr 
    :class="cn(
      'border-b transition-colors hover:bg-muted/50 data-[state=selected]:bg-muted cursor-pointer',  
    { 'bg-muted/50': props.isSelected },
     props.class,
     )"
    @click="props.onClick"
  >
    <slot />
  </tr>
</template>
