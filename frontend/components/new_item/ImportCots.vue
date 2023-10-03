<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const message = ref("");
const pn = ref("");
const name = ref("");

async function import_cots() {
  if (pn.value !== '' && name.value !== '') {
    message.value = await invoke("import_item", { pn: pn.value, name: name.value });
    name.value = "";
    pn.value = "";
  }
}
</script>

<template>
  <div class="card">
    <fieldset>
      <legend>Import a COTS</legend>
      <label for="pn">Part numer</label>
      <input id="pn-input" v-model="pn" placeholder="Enter the COTS PN" />
      <br />
      <label for="name">Name</label>
      <input id="name-input" v-model="name" placeholder="Enter a name..." />
      <br />
      <button type="button" @click="import_cots()">Create</button>
    </fieldset>
    <p>{{ message }}</p>
  </div>
</template>
