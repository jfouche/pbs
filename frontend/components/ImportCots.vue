<script setup lang="ts">
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
  <div class="card">
    <fieldset>
      <legend>Import a COTS</legend>
      <label for="pn">Part numer</label>
      <input id="pn-input" v-model="pn" placeholder="Enter the COTS PN" />
      <label for="name">Name</label>
      <input id="name-input" v-model="name" placeholder="Enter a name..." />
    </fieldset>
    <label></label>
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="button" @click="greet()">Greet</button>
  </div>

  <p>{{ greetMsg }}</p>
</template>
