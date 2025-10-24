<script setup>
import { computed } from "@vue/reactivity";
import { open } from "@tauri-apps/api/shell";
import { formatDate } from "../common";
import { playSelect } from "../sfx"; // ← add

const props = defineProps({
  title: String,
  important: Boolean,
  messages: Array,
});

const listClass = computed(() =>
  props.important ? "news-important" : "news-default"
);

function onMessageClick(link) {
  if (!link) return;
  playSelect();                 // ← play select.mp3
  open(link).catch((e) => console.error("open failed:", e));
}
</script>

<template>
  <div
    v-if="messages.length"
    class="col-span-3 w-full text-xl mb-[-5px]"
    :class="listClass"
  >
    <img
      :src="
        important
          ? '/classic/msg-line-important.png'
          : '/classic/msg-line-base.png'
      "
      draggable="false"
    />
    <div class="messages-header font-old relative bottom-[25px] left-[18px]">
      {{ title }}
    </div>
  </div>

  <template v-for="message in messages" :key="message.date">
    <div class="ml-[18px]" :class="listClass">
      {{ formatDate(message.date) }}
    </div>
    <div
      class="cursor-pointer news-button"
      :class="listClass"
      @click="onMessageClick(message.link)"
    >
      {{ message.message }}
    </div>
    <div>
      <img
        v-if="important"
        src="/classic/new.gif"
        class="mt-1.5"
        draggable="false"
      />
    </div>
  </template>
</template>