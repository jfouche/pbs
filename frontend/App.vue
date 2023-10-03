<script setup lang="ts">
import { ref } from "vue";

import TopMenu from "./components/top_menu/TopMenu.vue";
import PageNewItem from "./components/new_item/PageNewItem.vue";
import PageSearchItem from "./components/search/PageSearchItem.vue";
import PageViewItem from "./components/PageViewItem.vue";

const active_page = ref('')

function changePage(page: string) {
  active_page.value = page;
}
</script>

<template>
  <TopMenu  @changePage="changePage" />
  <div class="container">
    <h1>Product Breakdow Software</h1>
    
    <PageNewItem v-if="active_page === 'page_new_item'"/>
    <PageSearchItem v-else-if="active_page === 'page_search_items'"/>
    <PageViewItem v-else-if="active_page === 'page_view_item'"/>
    <p v-else>UNKNOWN PAGE</p>
  </div>
</template>

