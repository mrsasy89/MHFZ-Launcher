<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { confirm } from "@tauri-apps/api/dialog";
import SettingsItem from "./SettingsItem.vue";
import { playHover, playSelect, playStart, playConfirm, bindSfx } from "../sfx";
const props = defineProps<{
  /** Absolute path to the game root */
  gameFolder: string;
  /** Left-hand label shown by SettingsItem */
  label?: string;
  /** Button caption when idle */
  buttonText?: string;
}>();

const busy = ref(false);

async function handleClick() {
  const ok = await confirm(
    "Restore the game to its pristine state?\nAll server patches will be removed."
  );
  if (!ok) return;

  busy.value = true;
  try {
    await invoke("reset_game_files", { gameFolder: props.gameFolder });
    await confirm("Files restored to original. Happy hunting!");
  } catch (err) {
    console.error(err);
    await confirm("Reset failed: " + err);
  } finally {
    busy.value = false;
  }
}
</script>

<template>
  <!-- put the label on the left, button on the right -->
  <SettingsItem :name="label">
    <button
      :disabled="busy"
      class="px-3 py-1 rounded border border-red-400 hover:bg-red-500 hover:text-white transition"
      @click="handleClick"
	  @mouseenter="playHover()"
    >
      {{ busy ? "Resetting…" : buttonText }}
    </button>
  </SettingsItem>
</template>