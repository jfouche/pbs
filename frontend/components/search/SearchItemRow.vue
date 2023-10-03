<script setup lang="ts">
import { Item } from "../../item";

interface Props {
  item: Item
}
const props = defineProps<Props>();

interface Events {
  (event: 'itemSelection', id: number): void,
}
const emit = defineEmits<Events>();

async function view() {
  emit('itemSelection', props.item.id);
}


</script>

<template>
  <tr>
    <td>{{ props.item.name }}</td>
    <td>{{ props.item.pn }}</td>
    <td><a href="#" :id="props.item.id.toString()" @click="view">view</a></td>
  </tr>
</template>


<style scoped></style>
