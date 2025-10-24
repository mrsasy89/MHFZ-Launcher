<script setup>
import {
  store,
  doCreateCharacter,
  doSelectCharacter,
  doExportCharacter,
  dialogDeleteCharacter,
} from "../store";
import { addPlaceholderCharacter } from "../store";
import {
  LOGIN_PAGE,
  closeDropdown,
  formatDate,
  getCid,
  copyCid,
} from "../common";
import { storeMut } from "../store";

import { useFluent } from "fluent-vue";
import { onMounted, ref, watch } from "vue";

import { playHover, playSelect, playStart, playConfirm, bindSfx } from "../sfx";

function characterWeaponIcon(weapon) {
  return `/weapons/${weapon}.png`;
}

/* ─── HR remap ─────────────────────────────────────────────── */
const HR_MAP = { 1: 1, 30: 2, 50: 3, 99: 4, 299: 5, 998: 6, 999: 7 };
const displayHr = (raw) => HR_MAP[raw] ?? raw;

function onLogoutClick() {
  playSelect();
  storeMut.page = LOGIN_PAGE;
}

// Insert a single placeholder when the user clicks create
function onCreateClick() {
  playSelect();
  addPlaceholderCharacter();
}
</script>

<template>
  <div
    class="mhf-card row-span-2 overflow-hidden text-[20px] mt-[100px] mx-[5px] z-[10]"
  >
    <div class="flex flex-col gap-4 h-full">
      <div class="flex flex-row-reverse justify-between gap-2">
        <div
          @mouseenter="playHover()"
          @click="onLogoutClick()"
          class="btn btn-sm btn-primary text-[20px]"
        >
          {{ $t("logout-button") }}
        </div>
        <div
          class="btn btn-sm btn-primary text-[20px]"
          v-if="store.characters.length > 0"
          @mouseenter="playHover()"
          @click="onCreateClick"
        >
          {{ $t("create-character-label") }}
        </div>
      </div>
      <div
        class="grid grid-cols-[1fr_auto] gap-2 overflow-auto scrollbar mr-[-4px] pr-[4px] pb-5"
      >
        <template v-for="character in store.characters">
          <button
            class="btn btn-primary h-[200px] grid grid-cols-9 grow gap-1 gap-x-4 px-3 py-3 items-end justify-items-start text-[20px]"
            :disabled="store.characterLoading"
            @mouseenter="playHover()"
            @click="
              (character.id === null || character.placeholder)
                ? doCreateCharacter()
                : doSelectCharacter(character.id)
            "
          >
            <!-- weapon icon: only show for real characters -->
            <img
              v-if="character.id !== null && !character.placeholder"
              class="col-span-3 row-span-4 w-full h-full object-contain pt-0"
              draggable="false"
              :src="characterWeaponIcon(character.weapon)"
            />
            <div class="col-span-6 text-lg self-center text-[30px]">
              {{
                (character.id === null || character.placeholder)
                  ? $t('create-character-label')
                  : character.name
              }}
            </div>
            <!-- gender: only show for real characters -->
            <div
              class="col-span-6"
              v-if="character.id !== null && !character.placeholder"
            >
              {{ $t("character-gender-label") }}:
              <span v-if="character.isFemale">
                {{ $t("character-gender-female") }}
              </span>
              <span v-else>
                {{ $t("character-gender-male") }}
              </span>
            </div>
            <span class="col-span-3">
              ID:
              {{
                (character.id === null || character.placeholder)
                  ? '--'
                  : getCid(character.id)
              }}
            </span>
            <!-- HR/GR: only show for real characters -->
            <span
              class="col-span-3 flex gap-3 justify-end"
              v-if="character.id !== null && !character.placeholder"
            >
              <span>HR{{ displayHr(character.hr) }}</span>
              <span>GR{{ character.gr }}</span>
            </span>
            <div class="col-span-6">
              {{ $t("last-online-label") }}:
              <span>
                {{
                  (character.id === null || character.placeholder)
                    ? '--'
                    : formatDate(character.lastLogin)
                }}
              </span>
            </div>
          </button>
          <div class="dropdown dropdown-end">
            <label
              tabindex="0"
              @mouseenter="playHover()"
              @click="playSelect()"
              class="btn btn-sm btn-primary text-[20px]"
              >...</label
            >
            <ul
              tabindex="0"
              class="dropdown-content bg-[#111] z-[1] menu shadow shadow-black rounded-md w-max text-[20px]"
            >
              <li @click="closeDropdown(() => copyCid(character.id))">
                <a @click="playConfirm()">
                  {{ $t("copy-cid-label") }}
                </a>
              </li>
            </ul>
          </div>
        </template>
        <!-- Only show the big button if there are no characters at all -->
        <div
          class="btn btn-primary h-[120px] col-span-2 text-white text-[20px]"
          v-if="store.characters.length === 0"
          @click="onCreateClick"
        >
          {{ $t("create-character-label") }}
        </div>
      </div>
    </div>
  </div>
</template>