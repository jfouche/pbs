<script setup lang="ts">
import { ref } from 'vue';
import { Item } from '../../item';
import { invoke } from '@tauri-apps/api';

interface Props {
    item: Item,
    quantity: number
}
const props = defineProps<Props>();

interface ChildQuantity {
    item: Item,
    quantity: number
}

interface Children {
    val?: Array<ChildQuantity>
}

defineExpose({ update_children });

const isOpen = ref(false);
const children = ref<Children>({ val: undefined });

async function toggle() {
    isOpen.value = !isOpen.value;
    if (children.value.val === undefined) {
        update_children();
    }
}
async function update_children() {
    children.value.val = await invoke("get_children", { id: props.item.id });
    console.log('update_children', children.value);
}
</script>

<template>
    <li>
        <div @click="toggle">
            {{ props.item.pn }} - {{ props.item.name }} | quantity : {{ props.quantity }}
            <span v-if="children.val?.length != 0">[{{ isOpen ? '-' : '+' }}]</span>
        </div>
        <ul v-show="isOpen" v-if="children.val?.length != 0">
            <TreeItem class="item" v-for="child in children.val" :item="child.item" :quantity="child.quantity" />
        </ul>
    </li>
</template>

