<script setup>
import { useFluent } from "fluent-vue";
import { onMounted, ref, watch, computed } from "vue";
import { formatDate, getCid } from "../common";
import { doSelectCharacter, doCreateCharacter, store } from "../store";

const { $t } = useFluent();

/* ─── HR remap ─────────────────────────────────────────────── */
const HR_MAP = { 1: 1, 30: 2, 50: 3, 99: 4, 299: 5, 998: 6, 999: 7 };
const displayHr = (raw) => HR_MAP[raw] ?? raw;

/* ─── Weapon labels ────────────────────────────────────────── */
function getWeaponLabel(weapon) {
  switch (weapon) {
    case 0:  return $t("greatsword-label");
    case 1:  return $t("heavy-bowgun-label");
    case 2:  return $t("hammer-label");
    case 3:  return $t("lance-label");
    case 4:  return $t("sword-and-shield-label");
    case 5:  return $t("light-bowgun-label");
    case 6:  return $t("dual-swords-label");
    case 7:  return $t("longsword-label");
    case 8:  return $t("hunting-horn-label");
    case 9:  return $t("gunlance-label");
    case 10: return $t("bow-label");
    case 11: return $t("tonfa-label");
    case 12: return $t("switch-axe-label");
    case 13: return $t("magnet-spike-label");
  }
}

/* ─── Props ───────────────────────────────────────────────── */
const props = defineProps({
  character: Object,
  selectable: Boolean,
});

/* ─── Placeholder flag ─────────────────────────────────────── */
// A character is considered a placeholder if it is missing, has a null ID,
// or carries an explicit `placeholder` flag.
const isPlaceholder = computed(() => {
  const c = props.character;
  return !c || c.id === null || c.placeholder;
});

/* ─── Image logic ─────────────────────────────────────────── */
// Holds the currently displayed portrait (either a remote image or a fallback)
const unitSrc = ref('/units/unitbg.png');

// Build the URL of the remote portrait image using the character's name
function portraitUrl() {
  if (!store.currentEndpoint) return '';
  const host = store.currentEndpoint.url.replace(/^https?:\/\//, '');
  const cacheBust = props.character?.lastLogin || Date.now();
  return `http://${host}:8090/launcher/units/${encodeURIComponent(
    props.character.name
  )}.png?v=${cacheBust}`;
}

// Load either the remote portrait or the appropriate fallback
function loadPortrait() {
  const c = props.character;
  // If this entry is a placeholder or missing, use the custom background
  if (!c || c.id === null || c.placeholder) {
    unitSrc.value = '/units/unitbg.png';
    return;
  }
  // Otherwise, start with the weapon fallback
  unitSrc.value =
    typeof c.weapon === 'number'
      ? `/units/${c.weapon}.png`
      : '/units/unitbg.png';
  // Try to fetch the character-specific portrait
  const img = new Image();
  img.src = portraitUrl();
  img.onload = () => {
    unitSrc.value = img.src;
  };
}

onMounted(loadPortrait);
// Reload the portrait whenever the ID or name changes
watch(
  () => [props.character?.id, props.character?.name],
  loadPortrait
);

/* ─── Click handler ────────────────────────────────────────── */
// If this card represents a placeholder character, clicking it should create
// a new character rather than attempt to select it.  Otherwise, forward
// to doSelectCharacter.
function onCardClick() {
  if (!props.selectable) return;
  const c = props.character;
  if (!c || c.id === null || c.placeholder) {
    doCreateCharacter();
  } else {
    doSelectCharacter(c.id);
  }
}
</script>

<template>
  <div
    class="text-black my-2 h-[143px] w-[520px] p-2"
    :style="{ backgroundImage: `url(${unitSrc})` }"
  >
    <div
      class="w-full h-full flex flex-col items-center"
      :class="{ 'cursor-pointer': selectable }"
      @click="onCardClick()"
    >
      <!-- Name / Create text -->
      <div class="text-3xl mt-2 font-bold">
        {{ isPlaceholder ? $t('create-character-label') : character.name }}
      </div>

      <div class="grow py-2 px-4 w-full h-full flex">
        <!-- weapon icon + label -->
        <div class="flex-1 flex gap-2">
          <!-- Only show the weapon image when this is not a placeholder -->
          <img
            v-if="!isPlaceholder"
            :src="`/weapons/${character.weapon}.png`"
            class="h-[48px] m-2"
            draggable="false"
          />
          <div
            class="weapon-col grow flex flex-col items-center leading-4 justify-center mr-7"
          >
            <!-- Weapon label / placeholder text -->
            <div class="font-bold">
              {{ isPlaceholder ? '' : $t('weapon-label') }}
            </div>
            <div
              class="font-bold whitespace-nowrap"
              :class="
                isPlaceholder
                  ? 'text-xl'
                  : getWeaponLabel(character.weapon).length > 12
                  ? 'text-lg'
                  : 'text-xl'
              "
            >
              {{ isPlaceholder ? '' : getWeaponLabel(character.weapon) }}
            </div>
          </div>
        </div>

        <!-- stats -->
        <div class="flex-1 flex flex-col text-lg leading-[1] mt-1">
          <!-- Only show HR/GR/gender when not a placeholder -->
          <div class="flex gap-4" v-if="!isPlaceholder">
            <span>HR{{ displayHr(character.hr) }}</span>
            <span v-if="character.gr">GR{{ character.gr }}</span>
            <span class="font-mono">
              <span v-if="character.isFemale">♀</span>
              <span v-else>♂</span>
            </span>
          </div>
          <!-- ID and last online -->
          <div>
            ID:
            {{ isPlaceholder ? 'To be Determined' : getCid(character.id) }}
          </div>
          <div>
            {{
              isPlaceholder
                ? '--'
                : `${$t('last-online-label')}: ${formatDate(character.lastLogin)}`
            }}
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.weapon-col {
  min-width: 140px;
}
</style>