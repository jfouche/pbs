<script setup lang="ts">
import { ref } from "vue";

import TopMenu from "./components/TopMenu.vue";
import PageNewItem from "./components/PageNewItem.vue";
import PageSearchItem from "./components/PageSearchItem.vue";
import PageViewItem from "./components/PageViewItem.vue";

const active_page = ref('search')

function changePage(page: string) {
  active_page.value = page;
}
</script>

<template>
  <div class="container">
    <TopMenu  @changePage="changePage" />
    <h1>Welcome to Tauri!</h1>

    
    <PageNewItem v-if="active_page === 'page_new_item'"/>
    <PageSearchItem v-else-if="active_page === 'page_search'"/>
    <PageViewItem v-else-if="active_page === 'page_view_item'"/>
    <p v-else>UNKNOWN PAGE</p>
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
