<script setup lang="ts">
import { ref } from "vue";
import {Item} from "../item";

interface Props {
  item: Item
}
const props = defineProps<Props>()
</script>

<template>
    <li :id="props.item.id.toString()">
      {{ props.item.name }} : {{ props.item.pn }}
    </li>
</template>


<style scoped>
</style>
