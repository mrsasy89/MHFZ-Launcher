<script setup>
const props = defineProps({
  name: String,
});
</script>

<template>
  <div class="flex gap-1 items-center h-[45px]">
    <h2>
      {{ name }}
    </h2>
    <div class="grow"></div>
    <slot></slot>
  </div>
  <slot name="extended"></slot>
</template>
