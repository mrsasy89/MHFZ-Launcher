<script setup>
import "./style.css";

import { ref, computed, onMounted, onUnmounted, watch, nextTick } from "vue";
import { appWindow } from "@tauri-apps/api/window";
import { open } from "@tauri-apps/api/shell";

import Login        from "./Login.vue";
import Characters   from "./Characters.vue";
import MessageList  from "./MessageList.vue";
import Settings     from "./Settings.vue";
import Patcher      from "./Patcher.vue";

import { availableLocales } from "../fluent";
import {
  DELETE_DIALOG,
  SERVERS_DIALOG,
  LOGIN_PAGE,
  SETTINGS_PAGE,
  CHARACTERS_PAGE,
  openPicker,
  PATCHER_PAGE,
  PATCHER_DIALOG,
  GAME_VERSIONS,
} from "../common";

import {
  store,
  storeMut,
  setCurrentEndpoint,
  closeDialog,
  dialogRemoveEndpoint,
  dialogSaveEndpoint,
  dialogDeleteCharacterConfirm,
  recentLog,
  currentBanner,
  bannerIndex,
  setBannerIndex,
  onSettingsButton,
  dialogCallback,
  patcherLog,
  effectiveBanners,
} from "../store";

import {
  launcherHeaderUrl,
  capcomUrl,
  cogUrl,
  effectiveMessages,
} from "../store";

// ──────────────────────────────────────────────
// SFX
// ─────────────────────────────────────────────-
import { playHover, playConfirm, playStart, playSelect, bindSfx } from "../sfx";

// ──────────────────────────────────────────────
// Refs
// ─────────────────────────────────────────────-
const settingsBtn = ref(null);
const bannerImg   = ref(null);

// multiple link wrappers (one per v-for item)
const linkRefs    = ref([]);

// Unbinders
let unbindSettings = null;
let unbindBanner   = null;
let linkUnbinders  = [];

// ──────────────────────────────────────────────
// SFX handlers
// ─────────────────────────────────────────────-
function onSettingsClick() {
  playSelect();
  onSettingsButton();
}

function onNameserverClick() {
  playSelect();
  storeMut.page = LOGIN_PAGE
}

function onbanlinkClick(url) {
  playSelect();
  open(url);
}

function onDotClick(i) {
  playSelect();
  setBannerIndex(i);
}

function onLinkClick(url) {
  playSelect();
  open(url);
}

// called from template to capture element refs of v-for
function setLinkRef(i, el) {
  linkRefs.value[i] = el;
}

// bind hover to a single element
function bindHover(el) {
  return el ? bindSfx(el, { hover: true, click: null }) : null;
}

// (re)bind all link hover handlers
async function bindLinks() {
  // clear old
  linkUnbinders.forEach(fn => fn && fn());
  linkUnbinders = [];
  await nextTick();
  linkRefs.value.forEach(el => {
    if (el) linkUnbinders.push(bindHover(el));
  });
}

// rebind for single refs (settings & banner)
async function bindSingles() {
  await nextTick();
  if (unbindSettings) unbindSettings();
  if (settingsBtn.value) unbindSettings = bindHover(settingsBtn.value);

  if (unbindBanner) unbindBanner();
  if (bannerImg.value) unbindBanner = bindHover(bannerImg.value);
}

// ──────────────────────────────────────────────
// Mount / Watch / Unmount
// ─────────────────────────────────────────────-
onMounted(() => {
  bindSingles();
  bindLinks();
});

watch(
  () => [storeMut.page, currentBanner.value?.src, store.links.length],
  () => {
    bindSingles();
    bindLinks();
  },
  { flush: "post" }
);

onUnmounted(() => {
  if (unbindSettings) unbindSettings();
  if (unbindBanner)   unbindBanner();
  linkUnbinders.forEach(fn => fn && fn());
});

// ──────────────────────────────────────────────
// Locale picker
// ─────────────────────────────────────────────-
const localePicker = ref(false);
function openLocalePicker() { openPicker(localePicker); }

// ──────────────────────────────────────────────
// Init
// ─────────────────────────────────────────────-
setCurrentEndpoint(store.currentEndpoint);

// ──────────────────────────────────────────────
// Messages split
// ─────────────────────────────────────────────-
const messages = computed(() => {
  let announcements = [];
  let news = [];
  for (const m of effectiveMessages.value) {
    (m.kind === 1 ? announcements : news).push(m);
  }
  for (const m of store.remoteMessages) {
    (m.kind === 1 ? announcements : news).push(m);
  }
  announcements.sort((a, b) => b.date - a.date);
  news.sort((a, b) => b.date - a.date);
  return { announcements, news };
});

// ── Server‑dialog input SFX (classic) ─────────────────────────
const srvFocused = { name: false, url: false, lport: false, gport: false };

function onSrvFocus(key) {
  if (!srvFocused[key]) {
    playSelect();
    srvFocused[key] = true;
  }
}
function onSrvBlur(key) {
  srvFocused[key] = false;
}

let lastSrvKeyTs = 0;
function srvTypeSfx(e) {
  // ignore modifier keys & repeats
  if (
    e.repeat ||
    e.key === 'Shift'   ||
    e.key === 'Control' ||
    e.key === 'Alt'     ||
    e.key === 'Meta'
  ) return;

  const now = performance.now();
  if (now - lastSrvKeyTs < 45) return;
  lastSrvKeyTs = now;
  playHover();
}
</script>

<template>
  <div class="h-full w-full flex flex-col" :class="storeMut.locale">
    <Settings v-if="storeMut.page === SETTINGS_PAGE"></Settings>
    <div v-else class="grow w-full h-0 flex text-white gap-8">
      <div class="flex flex-col items-center mb-2 mt-5">
        <div class="self-start">
          <img draggable="false" :key="launcherHeaderUrl" :src="launcherHeaderUrl" @error="e => (e.target.src = fallbackLauncherHeader)"/>
          <div class="absolute">
            <div class="relative bottom-[45px] left-[350px] text-[#dcdcdc]">
              release ver. 1.4.5
            </div>
          </div>
          <div
            v-if="storeMut.page === CHARACTERS_PAGE"
            class="relative h-0 text-right bottom-4 right-0 text-sm"
          >
            {{ storeMut.username }}@{{ store.currentEndpoint.name }}|
            <span class="cursor-pointer" @mouseenter="playHover()" @click="onNameserverClick"
              >Disconnect</span
            >
          </div>
        </div>
        <div
          class="ml-3 h-[50px] w-full grow flex flex-col items-center overflow-hidden"
        >
          <Characters v-if="storeMut.page === CHARACTERS_PAGE"></Characters>
          <template v-else>
            <Login v-if="storeMut.page === LOGIN_PAGE"></Login>
            <Patcher v-else-if="storeMut.page === PATCHER_PAGE"></Patcher>
            <div
              class="grow bg-[#00000099] border-[1px] border-white/20 w-full rounded-sm m-2 p-[6px] text-[15px] leading-[14px] h-0 w-[426px] max-w-[426px]"
            >
              <div class="overflow-auto scrollbar h-full">
                <div v-for="log in store.log" style="overflow-anchor: none">
                  <div :class="log.level">{{ log.message }}</div>
                </div>
                <div v-if="patcherLog">
                  <div class="warning">{{ patcherLog }}</div>
                </div>
                <div style="overflow-anchor: auto; height: 1px"></div>
              </div>
            </div>
          </template>
        </div>
		<button
			ref="settingsBtn"
			class="settings-btn font-main relative text-lg"
			@click="onSettingsClick"
		>
			<span
			class="absolute inset-0 flex items-center justify-center text-[#d1c0a5] font-['Shippori Mincho'] pointer-events-none select-none"
			>
			<template v-if="storeMut.page !== SETTINGS_PAGE">
			</template>
			<template v-else>
				{{ $t('go-back-button') }}
			</template>
			</span>
		</button>
      </div>
      <div class="w-[532px] flex flex-col mr-[30px] mt-[30px] mb-3 gap-4">
        <div class="flex gap-2">
          <img
            ref="bannerImg"
			class="rounded shadow shadow-black shadow-md cursor-pointer"
            :src="currentBanner?.src"
            draggable="false"
            @click="onbanlinkClick(currentBanner?.link)"
          />
          <div class="flex flex-col justify-center gap-3">
            <button
              v-for="(_, i) in effectiveBanners"
              class="w-[10px] h-[10px] rounded-lg hover:bg-[#888888]"
              :class="i === bannerIndex ? 'bg-[#888888]' : 'bg-[#444444]'"
              @click="onDotClick(i)"
            ></button>
          </div>
        </div>
        <div
          class="grid grid-cols-[auto_auto_45px] auto-rows-auto gap-x-6 gap-y-2 overflow-auto scrollbar leading-4"
        >
          <MessageList
            :messages="messages.announcements"
            :title="$t('announcements-label')"
            :important="true"
          ></MessageList>
          <MessageList
            :messages="messages.news"
            :title="$t('news-label')"
          ></MessageList>
        </div>
        <div class="grow flex gap-8 flex-row-reverse">
          <div
            v-for="(link, i) in store.links"
            :key="link.link"
            class="inline-flex flex-col items-center cursor-pointer text-[#9DA7B9] hover:text-[#C4C6CA] link-item leading-none p-0 m-auto"
            :ref="el => setLinkRef(i, el)"
            @click="onLinkClick(link.link)"
          >
            <div
              class="rounded-[100px] h-[35px] w-[35px] mb-1 flex link-icon m-auto"
            >
              <img
                class="h-[32px] w-[32px] object-contain m-auto"
                draggable="false"
                :src="link.icon || '/classic/icon-inquiry.png'"
              />
            </div>
            <div class="text-sm text-center">{{ link.name }}</div>
          </div>
        </div>
      </div>
    </div>
    <div
      class="bg-[#00000080] h-[39px] col-span-2 flex gap-3 px-[30px] items-center overflow-clip flex-shrink-0"
    >
      <img :src="capcomUrl" :key="capcomUrl" @error="e => (e.target.src = fallbackcapcomUrl)" class="object-contain" draggable="false" />
      <img :src="cogUrl" :key="cogUrl" @error="e => (e.target.src = fallbackcogUrl)" class="object-contain" draggable="false" />
      <div class="text-[#a0a0a0] text-sm">
        ©CAPCOM CO., LTD. ALL RIGHTS RESERVED.
      </div>
      <div class="grow text-right">
        <span v-if="recentLog" :class="recentLog.level">
          {{ recentLog.message }}
        </span>
      </div>
    </div>
  </div>
  <div
    data-tauri-drag-region
    class="absolute top-0 left-0 right-0 px-2 pb-2 flex gap-1 text-white/60 justify-start"
  >
    <div data-tauri-drag-region class="grow"></div>
    <div>
      <div
        class="locale-picker flex flex-col rounded-b bg-[#00000099] w-max leading-5 text-sm uppercase cursor-pointer"
      >
        <div
          class="flex w-[60px] hover:bg-[#1b1b1b99]"
          @click="openLocalePicker"
        >
          <img
            class="w-[16px] ml-2"
            :src="`/flags/${storeMut.locale}.svg`"
            draggable="false"
          />
          <span class="ml-2">{{ storeMut.locale }}</span>
        </div>
        <template v-if="localePicker">
          <template v-for="l in availableLocales">
            <template v-if="l !== storeMut.locale">
              <div
                class="flex w-[60px] hover:bg-[#1b1b1b99]"
                @click="storeMut.locale = l"
              >
                <img
                  class="w-[16px] ml-2"
                  :src="`/flags/${l}.svg`"
                  draggable="false"
                />
                <span class="ml-2">{{ l }}</span>
              </div>
            </template>
          </template>
        </template>
      </div>
    </div>
    <img
      @click="appWindow.minimize"
      src="/classic/minimize.png"
      class="h-[20px] w-[50px] state-img"
      draggable="false"
    />
    <img
      @click="appWindow.close"
      src="/classic/close.png"
      class="h-[20px] w-[50px] state-img"
      draggable="false"
    />
  </div>
  <dialog
    :open="store.dialogOpen"
    @close="closeDialog"
    class="absolute top-0 h-full w-full bg-transparent z-[10]"
  >
    <div class="flex items-center h-full">
      <div
        class="bg-[url('/classic/dialog.jpg')] bg-contain flex flex-col items-center m-auto news-default gap-1 px-14"
        :class="
          store.dialogKind === DELETE_DIALOG
            ? 'w-[560px] h-[320px] pt-[90px]'
            : 'w-[700px] h-[400px] pt-[112px]'
        "
      >
        <template
          v-if="store.dialogKind === DELETE_DIALOG && store.deleteCharacter"
          class=""
        >
          <div class="text-xl">
            {{ $t("delete-character-label") }}
          </div>
          <div class="warning">
            {{
              $t("delete-character-confirmation", {
                character_name: store.deleteCharacter.name,
              })
            }}
          </div>
        </template>
        <template v-else-if="store.dialogKind === PATCHER_DIALOG">
          <div class="text-xl">
            {{ $t("patcher-updates-label") }}
          </div>
          <div v-html="$t('patcher-updates-confirmation')"></div>
        </template>
        <template
          v-if="store.dialogKind === SERVERS_DIALOG && storeMut.editEndpoint"
        >
          <div class="text-xl">
            <span v-if="store.editEndpointNew">
              {{ $t("server-add-label") }}
            </span>
            <span v-else>
              {{ $t("server-edit-label") }}
            </span>
          </div>
          <div class="grid grid-cols-7 gap-x-2 items-end gap-y-0.5 px-[100px]">
            <label for="server-name" class="col-span-7">
              {{ $t("server-name-label") }}
            </label>
            <input
              v-model="storeMut.editEndpoint.name"
              type="text"
              class="box-text w-full col-span-5 text-white"
              spellcheck="false"
              :class="
                (store.editEndpointNew || storeMut.editEndpoint.isRemote
                  ? 'col-span-7'
                  : 'col-span-5') +
                (storeMut.editEndpoint.isRemote ? ' disabled' : '')
              "
              :disabled="storeMut.editEndpoint.isRemote"
              @focus="onSrvFocus('name')"
              @blur="onSrvBlur('name')"
              @keydown="srvTypeSfx"
            />
            <button
              v-if="!store.editEndpointNew && !storeMut.editEndpoint.isRemote"
              class="box-text box-btn col-span-2"
              @mouseenter="playHover()"
              @click.prevent="playSelect(); dialogRemoveEndpoint()"
            >
              ❌ {{ $t("delete-button") }}
            </button>
            <label for="server-host" class="col-span-3">{{
              $t("server-host-label")
            }}</label>
            <label class="text-md news-default col-span-2">{{
              $t("server-launcher-port-label")
            }}</label>
            <label class="text-md news-default col-span-2">{{
              $t("server-game-port-label")
            }}</label>
            <input
              v-model="storeMut.editEndpoint.url"
              type="text"
              spellcheck="false"
              class="box-text w-full col-span-3 text-white"
              :class="{ disabled: storeMut.editEndpoint.isRemote }"
              :disabled="storeMut.editEndpoint.isRemote"
              @focus="onSrvFocus('url')"
              @blur="onSrvBlur('url')"
              @keydown="srvTypeSfx"
            />
            <input
              v-model.number="storeMut.editEndpoint.launcherPort"
              type="text"
              class="box-text col-span-2 text-white"
              spellcheck="false"
              placeholder="9010"
              :class="{ disabled: storeMut.editEndpoint.isRemote }"
              :disabled="storeMut.editEndpoint.isRemote"
              @focus="onSrvFocus('lport')"
              @blur="onSrvBlur('lport')"
              @keydown="srvTypeSfx"
            />
            <input
              v-model.number="storeMut.editEndpoint.gamePort"
              type="text"
              class="box-text col-span-2 text-white"
              spellcheck="false"
              placeholder="53310"
              :class="{ disabled: storeMut.editEndpoint.isRemote }"
              :disabled="storeMut.editEndpoint.isRemote"
              @focus="onSrvFocus('gport')"
              @blur="onSrvBlur('gport')"
              @keydown="srvTypeSfx"
            />
          </div>
        </template>
        <div class="grow"></div>
        <div class="flex gap-12 m-4 news-default items-center justify-between">
          <form method="dialog">
            <button class="box-text box-lg box-btn" @mouseenter="playHover()" @click="playSelect();">
              {{ $t("cancel-button") }}
            </button>
          </form>
          <div class="warning">
            {{ store.dialogError }}
          </div>
          <form method="dialog">
            <button
              class="box-text box-lg box-btn"
			  @mouseenter="playHover()"
			  @click="playConfirm();"
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
