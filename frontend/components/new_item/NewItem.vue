<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const name = ref("");
const message = ref("");

async function create_item() {
  message.value = await invoke("create_item", { name: name.value });
}
</script>

<template>
  <div class="card">
    <fieldset>
      <legend>Create new part number</legend>
      <label for="name">Name</label>
      <input id="name-input" v-model="name" placeholder="Enter a name..." />
      <button type="button" @click="create_item()">Create</button>
    </fieldset>
    <p>{{ message }}</p>
  </div>
</template>
