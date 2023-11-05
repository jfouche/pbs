<!-- ====================== -->
<!-- PageViewItem -->
<!-- ====================== -->
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



<!-- ====================== -->
<!-- AddChild -->
<!-- ====================== -->
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

<!-- ====================== -->
<!-- TreeItem -->
<!-- ====================== -->
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

