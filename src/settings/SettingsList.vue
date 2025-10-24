<script setup>
import { open } from "@tauri-apps/api/dialog";
import {
  CLASSIC_STYLE,
  MODERN_STYLE,
} from "../common";
import { storeMut, effectiveFolder, store, setSetting, setUiPref, setRange } from "../store";
import SettingsItem from "./SettingsItem.vue";
import SettingsCheckbox from "./SettingsCheckbox.vue";
import SettingsButton from "./SettingsButton.vue";
import { playHover, playSelect, playStart, playConfirm, bindSfx } from "../sfx";
// ────────────────────────────────────────────────────────
// Helper: choose a custom install directory
// ────────────────────────────────────────────────────────
async function onChooseFolder() {
  const folder = await open({ directory: true });
  if (folder !== null) {
    storeMut.gameFolder = folder;
  }
}

// ────────────────────────────────────────────────────────
// Helper: validate & commit numeric settings
// ────────────────────────────────────────────────────────
function setNumber(name, event) {
  let value = event.target.value;
  if (value === "") {
    value = 0;
  } else {
    value = parseInt(value);
  }
  if (!isNaN(value) && value > 0) {
    setSetting(name, value);
  } else {
    event.target.value = store.settings[name];
  }
}

</script>

<template>
  <div
    class="overflow-auto h-full scrollbar pr-2 flex flex-col gap-3 overflow-x-hidden"
  >
    <!-- ── General ───────────────────────────────────────────── -->
    <h1 id="general-settings" class="text-3xl text-[#ffd67c]">
      {{ $t("settings-general-title") }}
    </h1>
    <div class="flex flex-col gap-5 text-[25px]">
      <SettingsItem :name="$t('style-label')">
        <select
          v-model="storeMut.style"
          class="select select-primary select-sm w-max text-[20px]"
		  @click="playSelect()"
        >
          <option :value="MODERN_STYLE">{{ $t("modern-style") }}</option>
          <option :value="CLASSIC_STYLE">{{ $t("classic-style") }}</option>
        </select>
      </SettingsItem>


    </div>



    <!-- ── Audio (NEW) ───────────────────────────────────────── -->
    <div class="flex flex-col gap-2 text-[20px]">
      <SettingsCheckbox
        :model-value="store.settings.sfxEnabled"
        @update:model-value="setUiPref('sfxEnabled', $event)"
        :name="$t('Launcher SFX', 'Enable UI sounds')"
      />

      <SettingsItem
        v-if="store.settings.sfxEnabled"
        :name="$t('SFX Volume', 'UI sound volume')"
      >
        <div class="flex items-center gap-3">
          <input
            type="range"
            min="0"
            max="100"
            :value="store.settings.sfxVolume"
            class="range range-primary w-[200px]"
            @input="setRange('sfxVolume', $event)"
          />
          <span class="w-[50px] text-right">
            {{ store.settings.sfxVolume }}%
          </span>
        </div>
      </SettingsItem>
    </div>

    <div class="divider my-0 py-0"></div>

    <!-- ── Game ─────────────────────────────────────────────── -->
    <h1 id="game-settings" class="text-3xl text-[#ffd67c]">
      {{ $t("settings-game-title") }}
    </h1>
    <div class="flex flex-col gap-2 text-[20px]">
      <SettingsCheckbox
        :model-value="store.settings.hdVersion"
        @update:model-value="setSetting('hdVersion', $event)"
        :name="$t('hd-version-label')"
      />
      <SettingsCheckbox
        :model-value="store.settings.fullscreen"
        @update:model-value="setSetting('fullscreen', $event)"
        :name="$t('fullscreen-label')"
      />
      <SettingsItem :name="$t('window-resolution-label')">
        <div class="flex gap-1">
          <input
            :value="store.settings.windowW"
            @change="setNumber('windowW', $event)"
            inputmode="numeric"
            pattern="[0-9]*"
            class="input input-sm input-primary w-[90px] text-[20px]"
          />
          x
          <input
            :value="store.settings.windowH"
            @change="setNumber('windowH', $event)"
            inputmode="numeric"
            pattern="[0-9]*"
            class="input input-sm input-primary w-[90px] text-[20px]"
          />
        </div>
      </SettingsItem>
      <SettingsItem :name="$t('fullscreen-resolution-label')">
        <div class="flex gap-1">
          <input
            :value="store.settings.fullscreenW"
            @change="setNumber('fullscreenW', $event)"
            inputmode="numeric"
            pattern="[0-9]*"
            class="input input-sm input-primary w-[90px] text-[20px]"
          />
          x
          <input
            :value="store.settings.fullscreenH"
            @change="setNumber('fullscreenH', $event)"
            inputmode="numeric"
            pattern="[0-9]*"
            class="input input-sm input-primary w-[90px] text-[20px]"
          />
        </div>
      </SettingsItem>
    </div>

    <div class="divider my-0 py-0"></div>

    <!-- ── Advanced ─────────────────────────────────────────── -->
    <h1 id="advanced-settings" class="text-3xl text-[#ffd67c]">
      {{ $t("settings-advanced-title") }}
    </h1>
    <div class="flex flex-col gap-2 text-[20px]">
      <SettingsButton
		:label="$t('reset-patch-label')"
        :button-text="$t('reset-button-label')"
        :game-folder="storeMut.gameFolder ?? effectiveFolder"
      />
	  
      <SettingsItem :name="$t('game-folder-label')">
        <label class="label cursor-pointer m-auto">
          <input
            type="radio"
            name="game-folder"
            :checked="storeMut.gameFolder === null"
            @change="storeMut.gameFolder = null"
          />
          <span class="px-3 py-1 rounded border border-[#ffd67c] text-[20px]">
            {{ $t("current-folder-label") }}
          </span>
        </label>
        <label class="label cursor-pointer m-auto">
          <input
            type="radio"
            name="game-folder"
            :checked="storeMut.gameFolder !== null"
            @change="storeMut.gameFolder = effectiveFolder"
          />
          <button
            class="px-3 py-1 rounded border border-[#ffd67c] hover:bg-[#ffd67c] hover:text-white transition text-[20px]"
            :disabled="storeMut.gameFolder === null"
            @click="onChooseFolder"
          >
            {{ effectiveFolder }}
          </button>
        </label>
      </SettingsItem>

    </div>
  </div>
</template>
