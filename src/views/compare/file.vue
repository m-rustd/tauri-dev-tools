
<template>
  <n-button type="info" @click="handleClick">对比结果</n-button>
  <div class="dragWrapper">
    <DragBox type="file" desc="点击打开文件 或 拖拽文件至此" @onFileChange="onFileChange1" class="dragItem" />
    <DragBox type="file" desc="点击打开文件 或 拖拽文件至此" @onFileChange="onFileChange2" class="dragItem" />
  </div>
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
import DragBox from '@/components/DragBox.vue'

const convert = new Convert();
const div1 = ref(null)
const div2 = ref(null)
const content1 = ref('')
const content2 = ref('')

async function handleClick() {
  let result  = await invoke("compare", { content1: div1.value.innerText, content2: div2.value.innerText });
  if (result && result.length === 2) {
    console.log('$---', result);
    content1.value = result[0].replaceAll('\n', '</br>')
    content2.value = result[1].replaceAll('\n', '</br>')
  }
}

const onFileChange1 = async (data) => {
  const { file } = data;
  const reader = new FileReader(); // 创建FileReader对象  
  reader.readAsText(file); // 读取文件内容  
  reader.onload = () => {  
    div1.value.innerText = reader.result
  };  
}
const onFileChange2 = async (data) => {
  const { file } = data;
  const reader = new FileReader(); // 创建FileReader对象  
  reader.readAsText(file); // 读取文件内容  
  reader.onload = () => {  
    console.log('$---', reader.result);
    div2.value.innerText = reader.result
  }; 
}

</script>

<style lang="scss" scoped>
.dragWrapper {
  margin: 20px 0;
  display: flex;
  align-items: center;
  :deep(.dragItem) {
    flex: 1;
    &:first-child {
      margin-right: 20px;
    }
  }
}
.wrapper {
  height: calc(100vh - 360px);
  display: flex;
  margin-top: 10px;
  .divider {
    height: 100%;
  }
  .content {
    padding: 10px;
    flex: 1;
    background-color: rgb(250, 250, 252);
    min-width: 120px;
    overflow-y: scroll;
  }
}
</style>