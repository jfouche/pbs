<script setup lang="ts">
import { ref } from "vue";

const emit = defineEmits(['changePage'])


const name = ref("");
const message = ref("");

async function change_page(page: string) {
  console.log(page)
  emit('changePage', page);
}
</script>

<template>
  <div>
    <div><a href="#" @click="change_page('page_new_item')">New item</a></div>
    <div><a href="#" @click="change_page('page_search')">Search item</a></div>
    <div><a href="#" @click="change_page('page_view_item')">View item</a></div>
  </div>
</template>
