<!-- A counter, its props and a list. -->
<template>
  <div class="counter" :class="{ big: count > 10 }">
    <h1>{{ title.toUpperCase() }}</h1>
    <button @click="count++" :disabled="count >= max">Add one</button>
    <ul v-if="items.length">
      <li v-for="item in items" :key="item.id">{{ item.name }}</li>
    </ul>
    <p v-else>Nothing yet.</p>
    <slot name="footer" :count="count" />
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";

const props = defineProps<{ title: string; max: number }>();
const count = ref(0);
const items = ref<{ id: number; name: string }[]>([]);
</script>

<style scoped lang="scss">
.counter {
  padding: 1rem;
  &.big { font-weight: bold; }
}
</style>
