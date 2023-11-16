
<template>
  <n-button type="info" @click="handleClick">对比结果</n-button>
  <div class="wrapper">
    <div ref="div1" contentEditable="true" class="content" v-html="convert.toHtml(content1)" />
    <n-divider vertical class="divider" />
    <div ref="div2" contentEditable="true" class="content" v-html="convert.toHtml(content2)" />
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { invoke } from "@tauri-apps/api/tauri"
import Convert from 'ansi-to-html'

const convert = new Convert();
const div1 = ref(null)
const div2 = ref(null)
const content1 = ref('')
const content2 = ref('')

async function handleClick() {
  let result  = await invoke("compare", { content1: div1.value.innerText, content2: div2.value.innerText });
  if (result && result.length === 2) {
    content1.value = result[0].replaceAll('\n', '</br>')
    content2.value = result[1].replaceAll('\n', '</br>')
  }
}
</script>

<style lang="scss" scoped>
.wrapper {
  display: flex;
  margin-top: 10px;
  .divider {
    height: calc(100vh - 120px);
  }
  .content {
    padding: 10px;
    flex: 1;
    background-color: rgb(250, 250, 252);
    min-width: 120px;
  }
}
</style>