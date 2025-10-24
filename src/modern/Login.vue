<script setup>
import { ref, computed, onMounted, onBeforeUnmount } from "vue";
import { useFluent } from "fluent-vue";

import Settings from "./Settings.vue";
import { closeDropdown } from "../common";
import { availableLocales } from "../fluent";
import {
  storeMut,
  store,
  setCurrentEndpoint,
  doRegister,
  doLogin,
  dialogAddEndpoint,
  dialogEditEndpoint,
  onSettingsButton,
} from "../store";

// 🔊 SFX helpers
import { playHover, playSelect, playConfirm, playStart, bindSfx } from "../sfx";

const { $t } = useFluent();

const localeFlag = computed(() => `./src/icons/${storeMut.locale}.svg`);
setCurrentEndpoint(store.currentEndpoint);

// ----------------------
// Refs for SFX bindings
// ----------------------
const userInput    = ref(null);
const passInput    = ref(null);
const loginBtn     = ref(null);
const registerBtn  = ref(null);
const rememberWrap = ref(null);
const serverBtn    = ref(null);
const settingsBtn  = ref(null);

// ----------------------
// Helpers (SFX + logic)
// ----------------------

// throttle typing + ignore modifiers/repeats
let lastKeyTs = 0;
const MOD_KEYS = new Set(["Shift", "Control", "Alt", "Meta", "CapsLock"]);

function typeSfx(e) {
  if (MOD_KEYS.has(e.key) || e.repeat) return;
  const now = performance.now();
  if (now - lastKeyTs < 45) return;
  lastKeyTs = now;
  playHover();
}

// focus -> click sound once per focus cycle
const focusedMap = { user: false, pass: false };
function onFieldFocus(key) {
  if (!focusedMap[key]) {
    playSelect();
    focusedMap[key] = true;
  }
}
function onFieldBlur(key) {
  focusedMap[key] = false;
}

function onLoginClick()    { playConfirm(); doLogin();    }
function onRegisterClick() { playConfirm(); doRegister(); }
function onRememberChange(){ playSelect(); }

function onServerBtnClick(){ playSelect(); }

function chooseEndpoint(ep) {
  playSelect();
  closeDropdown(() => setCurrentEndpoint(ep));
}
function editEndpoint(i, remote) {
  playSelect();
  closeDropdown(() => dialogEditEndpoint(i, remote));
}
function addEndpoint() {
  playSelect();
  closeDropdown(dialogAddEndpoint);
}

function onLocaleChange(locale) {
  document.activeElement.blur();
  playSelect();
  storeMut.locale = locale;
}

function isCurrentEndpoint(endpoint) {
  return Object.entries(endpoint).every(
    ([k, v]) => v === store.currentEndpoint[k]
  );
}

function onSettingsClick() {
  playSelect();
  onSettingsButton();
}

// ----------------------
// Mount / unmount
// ----------------------
let unbinds = [];
onMounted(() => {
  // Hover SFX for buttons
  if (loginBtn.value)    unbinds.push(bindSfx(loginBtn.value,    { hover: true, click: null }));
  if (registerBtn.value) unbinds.push(bindSfx(registerBtn.value, { hover: true, click: null }));
  if (settingsBtn.value) unbinds.push(bindSfx(settingsBtn.value, { hover: true, click: null }));
  if (serverBtn.value)   unbinds.push(bindSfx(serverBtn.value,   { hover: true, click: null }));

  // Remember-me hover
  if (rememberWrap.value) unbinds.push(bindSfx(rememberWrap.value, { hover: true, click: null }));

  // Typing SFX
  if (userInput.value) userInput.value.addEventListener("keydown", typeSfx);
  if (passInput.value) passInput.value.addEventListener("keydown", typeSfx);
});

onBeforeUnmount(() => {
  unbinds.forEach(fn => fn && fn());
  if (userInput.value) userInput.value.removeEventListener("keydown", typeSfx);
  if (passInput.value) passInput.value.removeEventListener("keydown", typeSfx);
});
</script>

<template>
  <!-- Settings page -->
  <Settings
    v-if="storeMut.page == SETTINGS_PAGE"
    class="w-full h-full"
    @back="storeMut.page = prevPage"
  />

  <!-- Main modern layout -->
  <div
    v-else
    class="row-span-2 flex flex-col gap-3 w-[400px] absolute top-[100px] right-[100px]"
  >
    <!-- LOGIN CARD -->
    <div
      class="mhf-card !py-4 !px-10 flex flex-col gap-2 w-[300px] absolute left-[100px] top-[5px]"
    >
      <input
        ref="userInput"
        v-model="storeMut.username"
        :disabled="store.authLoading"
        class="input input-sm input-primary text-[20px] focus:border-[1px] focus:border-[#c5c3b6] focus:ring-2 focus:ring-[#c5c3b6] focus:outline-none"
        type="text"
        spellcheck="false"
        :placeholder="$t('username-label')"
		@focus="onFieldFocus('user')"
		@blur="onFieldBlur('user')"
      />
      <input
        ref="passInput"
        v-model="storeMut.password"
        :disabled="store.authLoading"
        class="input input-sm input-primary text-[20px] focus:border-[1px] focus:border-[#c5c3b6] focus:ring-2 focus:ring-[#c5c3b6] focus:outline-none"
        type="password"
        :placeholder="$t('password-label')"
		@focus="onFieldFocus('pass')"
		@blur="onFieldBlur('pass')"
      />

      <!-- Remember me -->
      <div class="flex flex-col">
        <label
          ref="rememberWrap"
          class="label cursor-pointer"
        >
          <input
            v-model="storeMut.rememberMe"
            :disabled="store.authLoading"
            type="checkbox"
            class="checkbox checkbox-info checkbox-sm"
            @change="onRememberChange"
          />
          <span
            class="label-text text-[20px]"
            :class="{ disabled: store.authLoading }"
          >
            {{ $t('remember-me-label') }}
          </span>
        </label>
      </div>

      <!-- Login/Register -->
      <div class="flex gap-2 justify-center">
        <button
          ref="loginBtn"
          class="btn btn-sm btn-primary text-[20px]"
          :disabled="store.authLoading"
          @click="onLoginClick"
        >
          {{ $t('login-button') }}
        </button>
        <button
          ref="registerBtn"
          class="btn btn-sm btn-primary text-[20px]"
          :disabled="store.authLoading"
          @click="onRegisterClick"
        >
          {{ $t('register-button') }}
        </button>
      </div>
    </div>

    <!-- SERVER + LANGUAGE + SETTINGS CARD -->
    <div
      class="mhf-card !py-3 !px-13 flex gap-2 absolute left-[50px] top-[210px] w-full"
    >
      <!-- Server dropdown -->
      <div class="dropdown dropdown-origin dropdown-glass" @click.stop>
        <label
          ref="serverBtn"
          tabindex="0"
          class="btn btn-sm btn-primary text-[20px]"
          :class="{ 'btn-disabled': store.authLoading }"
          @click.stop="onServerBtnClick"
        >
          <span>{{ store.currentEndpoint.name }}</span>
        </label>
        <div
          tabindex="0"
          class="dropdown-content z-[1] menu shadow shadow-black rounded-md w-max p-0 grid grid-cols-[1fr_auto] p-1 gap-x-0 overflow-auto scrollbar max-w-[250px]"
        >
          <!-- Remote endpoints -->
          <template v-if="store.remoteEndpoints.length">
            <ul class="menu p-1 text-[18px]">
              <li
                v-for="endpoint in store.remoteEndpoints"
                :key="'r-' + endpoint.name"
                :class="{ active: isCurrentEndpoint(endpoint) }"
                @click="chooseEndpoint(endpoint)"
              >
                <a>{{ endpoint.name }}</a>
              </li>
            </ul>
            <ul class="menu p-1 text-[18px]">
              <li
                v-for="(_, i) in store.remoteEndpoints"
                :key="'re-' + i"
                @click="editEndpoint(i, true)"
              >
                <a class="px-2">⚙</a>
              </li>
            </ul>
            <hr class="col-span-2 m-0" />
          </template>

          <!-- Local endpoints -->
          <template v-if="store.endpoints.length">
            <ul class="menu p-1 text-[18px]">
              <li
                v-for="endpoint in store.endpoints"
                :key="'l-' + endpoint.name"
                :class="{ active: isCurrentEndpoint(endpoint) }"
                @click="chooseEndpoint(endpoint)"
              >
                <a>{{ endpoint.name }}</a>
              </li>
            </ul>
            <ul class="menu p-1 text-[18px]">
              <li
                v-for="(_, i) in store.endpoints"
                :key="'le-' + i"
                @click="editEndpoint(i, false)"
              >
                <a class="px-2">⚙</a>
              </li>
            </ul>
            <hr class="col-span-2 m-0" />
          </template>

          <!-- Add endpoint -->
          <ul class="menu col-span-2 p-1 text-[18px]">
            <li @click="addEndpoint">
              <a>{{ $t('server-add-label') }}</a>
            </li>
          </ul>
        </div>
      </div>

      <!-- Settings button -->
      <div>
        <button
          ref="settingsBtn"
          class="btn btn-sm btn-primary text-[20px]"
          :disabled="store.authLoading"
          @click="onSettingsClick"
        >
          {{ $t('settings-button') }}
        </button>
      </div>

      <!-- Language dropdown -->
      <div class="grow flex justify-end">
        <div class="dropdown dropdown-origin dropdown-glass dropdown-end">
          <label tabindex="0" class="btn btn-sm btn-primary text-[20px]">
            <img
              :src="`/flags/${storeMut.locale}.svg`"
              draggable="false"
              class="h-[12px]"
            />
            <div>{{ storeMut.locale.toUpperCase() }}</div>
          </label>
          <ul
            tabindex="0"
            class="dropdown-content z-[1] menu shadow shadow-black rounded-md w-max text-[20px]"
          >
            <li
              v-for="l in availableLocales"
              :key="l"
              :class="{ active: l === storeMut.locale }"
              @click="onLocaleChange(l)"
            >
              <a>
                <img
                  :src="`/flags/${l}.svg`"
                  draggable="false"
                  class="h-[12px]"
                />
                {{ l.toUpperCase() }}
              </a>
            </li>
          </ul>
        </div>
      </div>
    </div>
  </div>
</template>