<script setup lang="ts">
import SearchItemRow from "./SearchItemRow.vue";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { ArrayOfItem } from "../../item";

interface Events {
  (event: 'itemSelection', id: number): void,
}
const emit = defineEmits<Events>();


const pattern = ref("");
const results = ref<ArrayOfItem>();

async function search_items() {
  if (pattern.value.length >= 2) {
    results.value = await invoke("search_items", { pattern: `%${pattern.value}%` });
  }
  else {
    results.value = [];
  }
}

async function select_item(id: number) {
  console.log(`PageSearchItem - select_item ${id}`);
  emit('itemSelection', id);
}

</script>

<template>
  SEARCH ITEM
  <input v-model="pattern" @input="search_items">
  <h1>Results</h1>
  <table>
    <tr>
      <th>Name</th>
      <th>Part number</th>
      <th>Actions</th>
    </tr>
    <SearchItemRow @item-selection="select_item" v-for="item in results" :item="item" />
  </table>
</template>
