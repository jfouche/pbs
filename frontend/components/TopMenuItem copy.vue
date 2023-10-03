<script setup lang="ts">
import { ref } from "vue";

const props = defineProps(['label', 'page', 'active_page'])

const emit = defineEmits<{(event: 'changePage', page: string): void}>();

async function change_page() {
  console.log("TopMenuItem<" + props.page + ">::change_page()");
  emit('changePage', props.page);
}

</script>

<template>
    <li v-bind:class="props.active_page === props.page ? 'active' : ''">
      <a href="#" @click="change_page">{{ props.label }}</a>
    </li>
</template>


<style scoped>
li {
  float: left;
}

li a {
  display: block;
  color: white;
  text-align: center;
  padding: 14px 16px;
  text-decoration: none;
}

li a:hover {
  background-color: #111;
}

li.active {
  background-color: black;
}
li.active a {
  text-decoration: underline;
  color :lightgreen;
}
</style>
