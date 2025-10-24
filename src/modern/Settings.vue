<script setup>
import { onSettingsButton } from "../store";
import SettingsList from "../settings/SettingsList.vue";
import { playHover, playSelect, playStart, playConfirm, bindSfx } from "../sfx";
const emit = defineEmits(["back"]);

function onBackClick() {
  playSelect();
  onSettingsButton();
}

</script>

<template>
  <div class="flex w-full h-full text-white overflow-clip">
    <div
      class="flex flex-col shadow shadow-black p-3 bg-black/50 flex gap-4 h-full"
    >
      <a href="#general-settings" class="btn btn-sm px-6 btn-primary w-full text-[20px]" @mouseenter="playHover()" @click="playSelect">
        {{ $t("settings-general-title") }}
      </a>
      <a href="#game-settings" class="btn btn-sm px-6 btn-primary text-[20px]" @mouseenter="playHover()" @click="playSelect">
        {{ $t("settings-game-title") }}
      </a>
      <a href="#advanced-settings" class="btn btn-sm px-6 btn-primary text-[20px]" @mouseenter="playHover()" @click="playSelect">
        {{ $t("settings-advanced-title") }}
      </a>
      <div class="grow"></div>
      <div class="btn btn-sm px-6 btn-primary w-max text-[20px]" @mouseenter="playHover()" @click="onBackClick">
        {{ $t("go-back-button") }}
      </div>
    </div>
    <div class="mhf-card !p-4 !pr-2 m-3 w-full overflow-clip overflow-x-auto">
      <SettingsList></SettingsList>
    </div>
  </div>
</template>
