<script setup lang="ts">
import { Item } from "../../item";

interface Props {
  item: Item
}
const props = defineProps<Props>()
</script>

<template>
  <tr>
    <td>{{ props.item.name }}</td>
    <td>{{ props.item.pn }}</td>
    <td><a href="#" :id="props.item.id.toString()">view</a></td>
  </tr>
</template>


<style scoped></style>
