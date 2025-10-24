<script setup>
import { ref, onMounted, onBeforeUnmount } from "vue";

import { openPicker } from "../common";
import {
  store,
  storeMut,
  setCurrentEndpoint,
  doLogin,
  doRegister,
  dialogEditEndpoint,
  dialogAddEndpoint,
} from "../store";

import { playHover, playSelect, playConfirm, bindSfx } from "../sfx";

const serverPicker = ref(false);

const usernameEl  = ref(null);
const passwordEl  = ref(null);
const loginBtn    = ref(null);
const registerBtn = ref(null);
const serverBtn   = ref(null);
const rememberEl  = ref(null);

// ────────────────────────────────
// SFX helpers
// ────────────────────────────────

let lastKeyTs = 0;
const MOD_KEYS = new Set(["Shift", "Control", "Alt", "Meta", "CapsLock"]);

function typeSfx(e) {
  // ignore pure modifier keys or auto-repeat of any key
  if (MOD_KEYS.has(e.key) || e.repeat) return;

  const now = performance.now();
  if (now - lastKeyTs < 45) return;
  lastKeyTs = now;
  playHover();
}

// focus sound: only once when changing focus
const lastFocusedEl = ref(null);
function onInputFocus(e) {
  if (lastFocusedEl.value !== e.target) {
    playSelect();
    lastFocusedEl.value = e.target;
  }
}
function onInputBlur(e) {
  if (lastFocusedEl.value === e.target) lastFocusedEl.value = null;
}

function onLoginClick()    { playConfirm(); doLogin();    }
function onRegisterClick() { playConfirm(); doRegister(); }

function onRememberClick() {
  playSelect();
  if (!store.authLoading) storeMut.rememberMe = !storeMut.rememberMe;
}

function openPickerRef() {
  playSelect(); // open/close sound
  openPicker(serverPicker);
}

// choose endpoint
function chooseEndpoint(endpoint) {
  playSelect();
  setCurrentEndpoint(endpoint);
  serverPicker.value = false;
}

// edit/add endpoints
function editEndpoint(i, remote) { playSelect(); dialogEditEndpoint(i, remote); }
function addEndpoint()           { playSelect(); dialogAddEndpoint(); }

// ────────────────────────────────
// mount / unmount
// ────────────────────────────────
let unbinds = [];
onMounted(() => {
  // hover for login/register buttons (click handled manually)
  if (loginBtn.value)    unbinds.push(bindSfx(loginBtn.value,    { hover: true, click: null }));
  if (registerBtn.value) unbinds.push(bindSfx(registerBtn.value, { hover: true, click: null }));

  // remember-me hover
  if (rememberEl.value)  unbinds.push(bindSfx(rememberEl.value,  { hover: true, click: null }));

  // key sounds
  if (usernameEl.value) usernameEl.value.addEventListener("keydown", typeSfx);
  if (passwordEl.value) passwordEl.value.addEventListener("keydown", typeSfx);
});

onBeforeUnmount(() => {
  unbinds.forEach(u => u && u());
  if (usernameEl.value) usernameEl.value.removeEventListener("keydown", typeSfx);
  if (passwordEl.value) passwordEl.value.removeEventListener("keydown", typeSfx);
});
</script>

<template>
  <div class="flex flex-col items-center w-full mt-2 px-12">
    <div class="min-w-[250px] flex flex-col">
      <label for="username_input">{{ $t("username-label") }}</label>
      <input
        ref="usernameEl"
        v-model="storeMut.username"
        type="text"
        id="username_input"
        class="box-text"
        spellcheck="false"
        :disabled="store.authLoading"
		@focus="onInputFocus"
		@blur="onInputBlur"
      />
    </div>

    <div class="min-w-[250px] flex flex-col">
      <label for="password_input">{{ $t("password-label") }}</label>
      <input
        ref="passwordEl"
        v-model="storeMut.password"
        type="password"
        id="password_input"
        class="box-text"
        :disabled="store.authLoading"
		@focus="onInputFocus"
		@blur="onInputBlur"
      />
    </div>

    <div class="flex flex-col">
      <label>{{ $t("server-select-label") }}</label>
      <div class="h-[50x] min-w-[250px] z-[1]">
        <div
          ref="serverBtn"
		  class="box-text cursor-pointer flex items-center"
          :class="{ 'box-disabled': store.authLoading }"
          @click="store.authLoading ? null : openPickerRef()"
        >
          <div class="grow">
            <span>{{ store.currentEndpoint.name }}</span>
          </div>
          <div :class="serverPicker ? 'arrow-up' : 'arrow-down'"></div>
        </div>

        <div
          v-if="serverPicker"
          class="absolute z-[-1] rounded-b mt-[-1px] bg-[#000000f0] border-[1px] border-t-0 border-white/20 w-[250px] cursor-pointer pt-0.5 max-h-[250px] overflow-auto scrollbar"
        >
          <div
            v-if="store.remoteEndpoints"
            class="border-b-[1px] border-white/20"
          >
            <div
              v-for="(endpoint, i) in store.remoteEndpoints"
              class="text-sm flex"
            >
              <span
                class="py-0.5 px-2 grow hover:bg-[#304368b8]"
                @click="chooseEndpoint(endpoint)"
              >
                {{ endpoint.name }}
              </span>
              <span
                class="py-0.5 px-1.5 hover:bg-[#304368b8]"
                @click="editEndpoint(i, true)"
              >
                ⚙
              </span>
            </div>
          </div>

          <div v-if="store.endpoints" class="border-b-[1px] border-white/20">
            <div v-for="(endpoint, i) in store.endpoints" class="text-sm flex">
              <span
                class="py-0.5 px-2 grow hover:bg-[#304368b8]"
                @click="chooseEndpoint(endpoint)"
              >
                {{ endpoint.name }}
              </span>
              <span
                class="py-0.5 px-1.5 hover:bg-[#304368b8]"
                @click="editEndpoint(i, false)"
              >
                ⚙
              </span>
            </div>
          </div>

          <div class="text-sm flex">
            <span
              class="py-0.5 px-2 grow hover:bg-[#304368b8]"
              @click="addEndpoint"
            >
              {{ $t("server-add-label") }}
            </span>
          </div>
        </div>
      </div>
    </div>

    <div class="flex gap-4 mt-6 text-2xl">
      <button
        ref="loginBtn"
        class="font-main w-[160px] h-[56px] bg-[url('/classic/btn-blue.png')] state-bg shadow shadow-md shadow-black rounded-md uppercase"
        :disabled="store.authLoading"
        @click="onLoginClick"
      >
        {{ $t("login-button") }}
      </button>

      <button
        ref="registerBtn"
        class="font-main w-[160px] h-[56px] bg-[url('/classic/btn-blue.png')] state-bg shadow shadow-md shadow-black rounded-md uppercase"
        :disabled="store.authLoading"
        @click="onRegisterClick"
      >
        {{ $t("register-button") }}
      </button>
    </div>

    <label
      ref="rememberEl"
	  class="flex gap-2 items-center hover:brightness-150 mt-2"
      :class="store.authLoading ? 'disabled' : 'cursor-pointer'"
      @click="store.authLoading ? null : onRememberClick()"
    >
      <img
        src="/classic/checkbox.png"
        draggable="false"
        class="h-[12px] w-[11px] object-none"
        :class="storeMut.rememberMe ? 'object-top' : 'object-bottom'"
      />
      <span class="text-sm">{{ $t("remember-me-label") }}</span>
    </label>
  </div>
</template>