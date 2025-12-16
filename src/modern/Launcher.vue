<script setup>
import { open } from "@tauri-apps/api/shell";
import { computed } from "@vue/reactivity";
import { ref, watch, nextTick } from "vue";

import Login       from "./Login.vue";
import Characters  from "./Characters.vue";
import Settings    from "./Settings.vue";
import Patcher     from "./Patcher.vue";

import {
  storeMut,
  store,
  recentLog,
  bannerIndex,
  setBannerIndex,
  currentBanner,
  onSettingsButton,
  closeDialog,
  dismissRecentLog,
  dialogRemoveEndpoint,
  dialogCallback,
  effectiveBanners,
} from "../store";

import {
  LOGIN_PAGE,
  CHARACTERS_PAGE,
  SETTINGS_PAGE,
  formatDate,
  DELETE_DIALOG,
  SERVERS_DIALOG,
  PATCHER_DIALOG,
  PATCHER_PAGE,
  GAME_VERSIONS,
} from "../common";

import {
  capcomUrl,
  cogUrl,
  effectiveMessages,
  launcherHeaderUrl,
} from "../store";
import { effectiveFolder } from "../store";

import { playHover, playSelect, playConfirm, playStart, bindSfx } from "../sfx";

const alertClass = {
  info:    "alert-info",
  warning: "alert-warning",
  error:   "alert-error",
};

/* ---------------- Messages (with fallback merge) ---------------- */
const messages = computed(() => {
  const base   = effectiveMessages.value;
  const merged = [
    ...base,
    ...store.remoteMessages.map(m => ({ ...m, global: true })),
  ];
  return merged.sort((a, b) => b.date - a.date);
});

/* ---------------- SFX helpers ---------------- */
function onMessageClick(link) {
  playSelect();
  open(link).catch(e => console.error("open failed:", e));
}

function onDotClick(i) {
  playSelect();
  setBannerIndex(i);
}

function onLinkClick(url) {
  playSelect();
  open(url);
}

/* Banner hover via bindSfx (re-bind when src changes) */
const bannerImg = ref(null);
let   unbindBanner = null;

async function bindBanner() {
  await nextTick();
  if (unbindBanner) unbindBanner();
  if (bannerImg.value) {
    unbindBanner = bindSfx(bannerImg.value, { hover: true, click: null });
  }
}
watch(() => currentBanner.value?.src, bindBanner, { flush: "post" });
nextTick(bindBanner); // first mount


// ── Server dialog input SFX ───────────────────────────────────
const srvFocused = { name:false, url:false, lport:false, gport:false };

function onSrvFocus(key) {
  if (!srvFocused[key]) {
    playSelect();
    srvFocused[key] = true;
  }
}
function onSrvBlur(key) {
  srvFocused[key] = false;
}

// throttle + ignore modifier keys (Shift, Ctrl, etc.)
let lastSrvKeyTs = 0;
function srvTypeSfx(e) {
  if (e.repeat || e.key.length > 1 && e.key !== 'Backspace' && e.key !== 'Delete') return;
  const now = performance.now();
  if (now - lastSrvKeyTs < 45) return;
  lastSrvKeyTs = now;
  playHover();
}

</script>

<template>
  <div class="w-full h-full">
    <Settings
      v-if="storeMut.page == SETTINGS_PAGE"
      class="w-full h-full"
      @back="storeMut.page = prevPage"
    >
    </Settings>
    <div
      v-else
      class="grid p-2 gap-4 grid-cols-[600px_auto] grid-rows-[135px_auto] w-full h-full ml-1"
    >
      <div>
        <img
          id="banner"
          class="rounded shadow shadow-black cursor-pointer w-full"
          draggable="false"
          :src="currentBanner?.src"
          @mouseenter="playHover()"
		  @click="playSelect(); open(currentBanner?.link)"
        />
        <div class="flex gap-5 justify-center">
          <button
            v-for="(_, i) in effectiveBanners"
            class="w-[13px] h-[13px] rounded-lg hover:bg-[#888888] my-2 z-10"
            :class="i === bannerIndex ? 'bg-[#888888]' : 'bg-[#444444]'"
            @click="onDotClick(i)"
          ></button>
        </div>
      </div>
      <div class="flex flex-col gap-3 row-span-2 overflow-hidden">
        <Login v-if="storeMut.page == LOGIN_PAGE"></Login>
        <Characters v-else-if="storeMut.page == CHARACTERS_PAGE"></Characters>
        <Patcher v-else-if="storeMut.page == PATCHER_PAGE"></Patcher>
        <div
          class="grow flex items-end justify-between"
        >
          <!-- bottom‐left: icon links -->
          <div class="flex gap-3">
            <button
              v-for="link in store.links"
              :key="link.name"
              class="btn btn-sm btn-ghost text-[#A6D8FF] text-[20px]"
              @mouseenter="playHover()"
			  @click="onLinkClick(link.link)"
            >
              <img
                v-if="link.icon"
                :src="link.icon"
                class="h-[14px] link-image"
                draggable="false"
              />
              {{ link.name }}
            </button>
          </div>
		  <!-- ─── future ─────────────────────────── -->
        </div>
      </div>
      <div
        class="p-4 grid gap-0.5 gap-x-2 grid-cols-[max-content_auto] overflow-auto content-start ml-1 scrollbar text-[20px]"
      >
        <template v-for="message in messages">
          <span class="py-1" :class="{ 'text-yellow-300': message.kind == 1 }">
            {{ formatDate(message.date) }}
          </span>
          <button
            class="btn btn-sm btn-ghost mr-2 h-max px-1.5 py-1 text-start justify-start leading-5 text-[19px]"
            :class="{ 'text-yellow-300': message.kind == 1 }"
            @click="onMessageClick(message.link)"
          >
            {{ message.message }}
          </button>
        </template>
      </div>
	  <div class="h-[50px] col-span-2 flex gap-3 px-[30px] items-center overflow-hidden absolute bottom-0">
		<img :src="capcomUrl" :key="capcomUrl" @error="e => (e.target.src = fallbackcapcomUrl)" class="object-contain" draggable="false" />
		<img :src="cogUrl" :key="cogUrl" @error="e => (e.target.src = fallbackcogUrl)" class="object-contain" draggable="false" />
	  </div>
	  <div class="col-span-2 flex gap-3 px-[30px] items-center overflow-hidden absolute top-[0px] right-[30px] z-[0]">
	    <img :src="launcherHeaderUrl" :key="launcherHeaderUrl" @error="e => (e.target.src = fallbacklauncherHeaderUrl)" class="object-contain w-[400px] h-[100px]" draggable="false" />
		<a class="absolute bottom-[22px] right-[50px] text-[15px]">release ver. 1.4.6</a>
	  </div>
    </div>
  </div>
  <div
    v-if="recentLog"
    class="toast toast-start z-[5]"
    @click="dismissRecentLog"
  >
    <div class="alert cursor-pointer py-2 text-[20px]" :class="alertClass[recentLog.level]">
      <span class="text-black font-medium">{{ recentLog.message }}</span>
    </div>
  </div>
  <dialog
    :open="store.dialogOpen"
    @close="closeDialog"
    class="absolute top-0 h-full w-full bg-transparent z-[10]"
  >
    <div
      v-if="store.dialogOpen"
      class="flex items-center justify-center h-full"
    >
      <div class="absolute top-0 left-0 h-full w-full bg-black/25"></div>
      <div
        class="modal-box rounded-lg shadow shadow-black bg-[#111] text-white flex flex-col gap-4"
      >
        <template v-if="store.dialogKind === DELETE_DIALOG">
          <h3 class="font-bold text-lg">{{ $t("delete-character-label") }}</h3>
          <p class="py-4">
            {{
              $t("delete-character-confirmation", {
                character_name: store.deleteCharacter.name,
              })
            }}
          </p>
        </template>
        <template v-else-if="store.dialogKind === PATCHER_DIALOG">
          <h3 class="font-bold text-lg">{{ $t("patcher-updates-label") }}</h3>
          <p class="py-4" v-html="$t('patcher-updates-confirmation')"></p>
        </template>
        <template v-else-if="store.dialogKind === SERVERS_DIALOG">
          <h3 class="font-bold text-lg">
            <span v-if="store.editEndpointNew">
              {{ $t("server-add-label") }}
            </span>
            <span v-else>
              {{ $t("server-edit-label") }}
            </span>
          </h3>
          <div class="grid grid-cols-12 gap-y-0.5 gap-x-3">
            <label for="server-name" class="col-span-12 mt-1">
              {{ $t("server-name-label") }}
            </label>
            <input
              v-model="storeMut.editEndpoint.name"
              type="text"
              spellcheck="false"
              class="input input-sm input-primary text-[20px]"
              :class="
                store.editEndpointNew || storeMut.editEndpoint.isRemote
                  ? 'col-span-12'
                  : 'col-span-9'
              "
              :disabled="storeMut.editEndpoint.isRemote"
              @focus="onSrvFocus('name')"
              @blur="onSrvBlur('name')"
              @keydown="srvTypeSfx"
            />
            <button
              v-if="!store.editEndpointNew && !storeMut.editEndpoint.isRemote"
              class="btn btn-sm btn-primary col-span-3"
              @mouseenter="playHover()"
              @click.prevent="playSelect(); dialogRemoveEndpoint()"
            >
              ❌ {{ $t("delete-button") }}
            </button>
            <label for="server-host" class="col-span-6 mt-1">
              {{ $t("server-host-label") }}
            </label>
            <label class="col-span-3 mt-1">
              {{ $t("server-launcher-port-label") }}
            </label>
            <label class="col-span-3 mt-1">
              {{ $t("server-game-port-label") }}
            </label>
            <input
              v-model="storeMut.editEndpoint.url"
              type="text"
              spellcheck="false"
              class="input input-sm input-primary col-span-6 text-[20px]"
              :disabled="storeMut.editEndpoint.isRemote"
              @focus="onSrvFocus('url')"
              @blur="onSrvBlur('url')"
              @keydown="srvTypeSfx"
            />
            <input
              v-model.number="storeMut.editEndpoint.launcherPort"
              type="text"
              class="input input-sm input-primary col-span-3 text-[20px]"
              spellcheck="false"
              placeholder="9010"
              :disabled="storeMut.editEndpoint.isRemote"
              @focus="onSrvFocus('lport')"
              @blur="onSrvBlur('lport')"
              @keydown="srvTypeSfx"
            />
            <input
              v-model.number="storeMut.editEndpoint.gamePort"
              type="text"
              class="input input-sm input-primary col-span-3 text-[20px]"
              spellcheck="false"
              placeholder="53310"
              :disabled="storeMut.editEndpoint.isRemote"
              @focus="onSrvFocus('gport')"
              @blur="onSrvBlur('gport')"
              @keydown="srvTypeSfx"
            />
          </div>
        </template>
        <div class="flex justify-between gap-2 items-center">
          <form method="dialog">
            <button class="btn btn-sm btn-primary text-[20px]" @mouseenter="playHover()" @click="playSelect()">
              {{ $t("cancel-button") }}
            </button>
          </form>
          <div class="warning text-[20px]">
            {{ store.dialogError }}
          </div>
          <form method="dialog">
            <button
              class="btn btn-sm btn-primary text-[20px]"
              @mouseenter="playHover()"
			  @click.prevent="dialogCallback"
            >
              <span v-if="store.dialogKind === DELETE_DIALOG">
                {{ $t("delete-button") }}
              </span>
              <span v-else-if="store.dialogKind === PATCHER_DIALOG">
                {{ $t("install-button") }}
              </span>
              <span v-else-if="store.editEndpointNew">
                {{ $t("add-button") }}
              </span>
              <span v-else>
                {{ $t("save-button") }}
              </span>
            </button>
          </form>
        </div>
      </div>
    </div>
  </dialog>
</template>
