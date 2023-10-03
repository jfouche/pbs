<script setup lang="ts">
import { Item } from '../item';

interface Props {
  item?: Item
}
const props = defineProps<Props>();


</script>

<template> 
VIEW ITEM {{ props.item }}
</template>
