<script setup lang="ts">
import { ref } from 'vue';
import { ArrayOfItem, Item } from '../../item';
import { invoke } from '@tauri-apps/api';

interface Props {
    item: Item
}
const props = defineProps<Props>();

interface Events {
  (event: 'childAdded'): void,
}
const emit = defineEmits<Events>();

const pattern = ref('');
const results = ref<ArrayOfItem>();
const quantity = ref(1);

async function search_items() {
    if (pattern.value.length >= 2) {
        results.value = await invoke("search_items", { pattern: `%${pattern.value}%` });
        console.log('search_items : ' + results.value);
    }
    else {
        results.value = [];
    }
}

async function add_child() {
    let params = { parentId: props.item.id, childPn: pattern.value, quantity: quantity.value };
    await invoke("add_child", params)
        .then((_) => emit('childAdded'))
        .catch((e) => console.error(e));
    
}

</script>

<template>
    <div class="card">
        <input type="text" name="pattern" id="pattern" placeholder="Search item"
            :list="results?.length != 0 ? 'matching-items' : ''" v-model="pattern" @input="search_items">

        <datalist v-if="results?.length != 0" id="matching-items">
            <option v-for="item in results" :value="item.pn">{{ item.name }} | {{ item.pn }}</option>
        </datalist>

        <input type="number" min="1" name="pattern" id="pattern" placeholder="Quantity" v-model="quantity">

        <button type="button" @click="add_child()">Add</button>
    </div>
</template>

