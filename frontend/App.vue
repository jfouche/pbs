<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import TopMenu from "./components/top_menu/TopMenu.vue";
import PageNewItem from "./components/new_item/PageNewItem.vue";
import PageSearchItem from "./components/search/PageSearchItem.vue";
import PageViewItem from "./components/PageViewItem.vue";
import { Item } from "./item";

const active_page = ref('')
const active_item = ref<Item>();

function changePage(page: string) {
  active_page.value = page;
}

async function select_item(id: number) {
  console.log(`App - select_item ${id}`);
  active_item.value = await invoke("get_item_by_id", { id: id });
  active_page.value = 'page_view_item';

}
</script>

<template>
  <TopMenu  @changePage="changePage" />
  <div class="container">
    <h1>Product Breakdow Software</h1>
    <PageNewItem v-if="active_page === 'page_new_item'"/>
    <PageSearchItem v-else-if="active_page === 'page_search_items'"  @item-selection="select_item"/>
    <PageViewItem v-else-if="active_page === 'page_view_item'" :item="active_item"/>
    <p v-else>UNKNOWN PAGE</p>
  </div>
</template>

