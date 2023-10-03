<script setup lang="ts">
import NewItem from "./NewItem.vue";
import ImportCots from "./ImportCots.vue";

import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const greetMsg = ref("");
const pn = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
  <NewItem />
  <ImportCots />
</template>
