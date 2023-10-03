<script setup lang="ts">
import SearchItemRow from "./SearchItemRow.vue";

import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import {ArrayOfItem} from "../item";

const pattern = ref("");
const results = ref<ArrayOfItem>();

async function search_items() {
  if (pattern.value.length >= 3) {
    results.value = await invoke("search_items", { pattern: pattern.value });
  }
  else {
    results.value = [];
  }
}

</script>

<template>
  SEARCH ITEM
  <input v-model="pattern" @input="search_items">
  <h1>Results</h1>
  <SearchItemRow v-for="item in results" :item="item"/>
  <div></div>
</template>
