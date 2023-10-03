<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const pattern = ref("");
const results = ref("");

async function search_items() {
  if (pattern.value.length >= 3) {
    results.value = await invoke("search_items", { pattern: pattern.value });
  }
}

</script>

<template>
  SEARCH ITEM
  <input v-model="pattern" @input="search_items">
  <h1>Results</h1>
  <p>{{ results }}</p>
</template>
