<script setup>
import { open } from "@tauri-apps/api/dialog";

import { onSettingsButton, storeMut } from "../store";
import SettingsList from "../settings/SettingsList.vue";
import { playHover, playSelect, playStart, playConfirm, bindSfx } from "../sfx";
async function onChooseFolder() {
  const folder = await open({ directory: true });
  if (folder !== null) {
    storeMut.gameFolder = folder;
  }
}

function onBackClick() {
  playSelect();
  onSettingsButton();
}

</script>

<template>
  <div class="flex gap-2 mx-2 grow mt-7 mb-2 overflow-hidden">
    <div class="flex flex-col box-text gap-2 !py-2 ">
      <a href="#general-settings" class="box-text box-btn !px-8 text-center" @mouseenter="playHover()" @click="playSelect">
        {{ $t("settings-general-title") }}
      </a>
      <a href="#game-settings" class="box-text box-btn !px-8 text-center" @mouseenter="playHover()" @click="playSelect">
        {{ $t("settings-game-title") }}
      </a>
      <a href="#advanced-settings" class="box-text box-btn !px-8 text-center" @mouseenter="playHover()" @click="playSelect">
        {{ $t("settings-advanced-title") }}
      </a>
      <div class="grow"></div>
      <div class="box-text box-btn !px-5 text-center" @mouseenter="playHover()" @click="onBackClick">
        {{ $t("go-back-button") }}
      </div>
    </div>
    <div class="grow box-text !px-4 !py-4 flex flex-col gap-2 overflow-hidden">
      <SettingsList></SettingsList>
    </div>
  </div>
</template>
