<script setup lang="ts">
import { Item } from '../../item';
import TreeItem from './TreeItem.vue'
import AddChild from './AddChild.vue'
import { ref } from 'vue';

interface Props {
  item?: Item
}
const props = defineProps<Props>();

const tree = ref< InstanceType<typeof TreeItem> | null >(null);

async function update() {
  tree.value?.update_children();
}

</script>

<template>
  <div class="tree" v-if="props.item !== undefined">
    <h2>{{ props.item?.pn }} : {{ props.item?.name }}</h2>
    <TreeItem :item="props.item" :quantity=1 ref="tree"/>
    <AddChild :item="props.item" @child-added="update"/>
  </div>
  <div v-else>Select an item in the [Search]</div>
</template>

<style>
div.tree {
  text-align: left;
}</style>