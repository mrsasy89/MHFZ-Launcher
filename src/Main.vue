<script setup>
import { ref, watch, onMounted, onBeforeUnmount } from "vue";
import { listen } from "@tauri-apps/api/event";

import "./style.css";

import {
  storeMut,
  initStore,
  initRemoteEndpoints,
  updateRemoteMessages,
  updatePatcher,
  logText,
} from "./store";
import ClassicLauncher from "./classic/Launcher.vue";
import ModernLauncher from "./modern/Launcher.vue";
import { MODERN_STYLE, CLASSIC_STYLE } from "./common";
import { logMessage } from "./store";

const uiScale = ref(1);

// Recalculate scale whenever window resizes or style changes
function updateScale() {
  const designW = 1124;
  const designH = 600;
  const w = window.innerWidth;
  const h = window.innerHeight;
  // never upscale above 1
  uiScale.value = Math.min(w / designW, h / designH, 1);
  document.documentElement.style.setProperty(
    "--ui-scale",
    uiScale.value.toString()
  );
}

onMounted(() => {
  updateScale();
  window.addEventListener("resize", updateScale);
});

// Re-run scale when user flips between Classic/Modern
watch(() => storeMut.style, () => {
  updateScale();
});

onBeforeUnmount(() => {
  window.removeEventListener("resize", updateScale);
});

const initialLoaded = ref(false);
initStore().then(() => (initialLoaded.value = true));

listen("userdata", ({ payload }) => {
  storeMut.username   = payload.userdata.username;
  storeMut.password   = payload.password;
  storeMut.rememberMe = payload.userdata.rememberMe;
});
listen("endpoints", ({ payload }) => {
  initRemoteEndpoints(payload);
});
listen("remote_messages", ({ payload }) => {
  updateRemoteMessages(payload);
});
listen("patcher", ({ payload }) => {
  updatePatcher(payload);
});
listen("log", ({ payload }) => {
  logMessage(payload.level, payload.message);
});
</script>

<template>
  <div id="app-wrapper">
    <template v-if="initialLoaded">
      <ClassicLauncher
        v-if="storeMut.style === CLASSIC_STYLE"
      />
      <ModernLauncher
        v-else-if="storeMut.style === MODERN_STYLE"
      />
    </template>
  </div>
</template>

<style>
html, body {
  overflow: hidden;
  margin: 0;
  padding: 0;
}

#app-wrapper {
  width: 1124px;
  height: 600px;
  transform-origin: top left;
  transform: scale(var(--ui-scale, 1));
  overflow: hidden;
}
</style>
