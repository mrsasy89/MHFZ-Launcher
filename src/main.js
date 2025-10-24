import "./style.css";

import { createApp, watch } from "vue";

import Main from "./Main.vue";
import { fluentVue } from "./fluent";
import { backgroundUrl } from "./store.js";   // ← new import

// --- create & mount the app exactly as before ---
const app = createApp(Main).use(fluentVue);
app.mount("#app");

// --- NEW: keep #app's wallpaper in sync with backgroundUrl ---
watch(
  backgroundUrl,
  (url) => {
    document.getElementById("app").style.backgroundImage = `url('${url}')`;
  },
  { immediate: true }   // apply the current value on first load
);