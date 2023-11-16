<template>
  <ol>
    <li v-for="(link, index) in links" :key="index">
      <router-link :to="link.path">{{ link.title }}</router-link>
    </li>
  </ol>
</template>

<script setup>
import { ref } from 'vue'

const links = ref([
  {
    title: '文件对比',
    path: '/compare/content',
  }
])
</script>