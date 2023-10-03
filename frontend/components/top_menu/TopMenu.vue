<script setup lang="ts">
import TopMenuItem from "./TopMenuItem.vue";
import { ref } from "vue";

interface Events {
  (event: 'changePage', page: string): void,
}
const emit = defineEmits<Events>();

const active_page = ref("page_search_items");
emit('changePage', active_page.value);

async function change_page(page: string) {
  console.log("TopMenu::change_page " + page);
  active_page.value = page;
  emit('changePage', page);
}
</script>

<template>
  <ul>
    <TopMenuItem :label="'New item'"    :page="'page_new_item'"     :active_page="active_page"  @change-page="change_page"/>
    <TopMenuItem :label="'Search item'" :page="'page_search_items'" :active_page="active_page"  @change-page="change_page"/>
    <TopMenuItem :label="'View item'"   :page="'page_view_item'"    :active_page="active_page"  @change-page="change_page"/>
  </ul>
</template>


<style scoped>
ul {
  list-style-type: none;
  margin: 0;
  padding: 0;
  overflow: hidden;
  background-color: #333;
}
</style>
