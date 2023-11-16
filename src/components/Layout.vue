<template>
  <n-layout has-sider class="layout">
    <n-layout-sider bordered collapse-mode="width" :collapsed-width="64" :width="240" :collapsed="collapsed" show-trigger
      @collapse="collapsed = true" @expand="collapsed = false">
      <n-menu ref="menuInstRef" v-model:value="activeKey" :collapsed="collapsed" :collapsed-width="64" :collapsed-icon-size="22"
        :options="menuOptions" />
    </n-layout-sider>
    <n-layout>
      <n-layout-header bordered>
        header
      </n-layout-header>
      <n-layout-content content-style="padding: 24px;">
        <router-view />
      </n-layout-content>
    </n-layout>
  </n-layout>
</template>

<script setup>
import { h, ref, watch } from 'vue'
import { NIcon } from 'naive-ui'
import {
  BookOutline as BookIcon,
  PersonOutline as PersonIcon,
  FolderOutline as FolderIcon,
  DocumentOutline as DocumentIcon,
  HomeOutline as HomeIcon,
} from '@vicons/ionicons5'
import { RouterLink, useRouter } from 'vue-router'

function renderIcon (icon) {
  return () => h(NIcon, null, { default: () => h(icon) })
}

function link(title, path, params) {
  return () =>
    h(
      RouterLink,
      {
        to: {
          path,
        }
      },
      { default: () => title }
    )
}

const menuOptions = ref([
  {
    label: link('首页', '/'),
    key: 'home',
    icon: renderIcon(HomeIcon)
  },
  {
    label: '文件对比',
    key: 'compare',
    icon: renderIcon(FolderIcon),
    children: [
      {
        label: link('内容对比', '/compare/content'),
        key: 'compare-content',
        icon: renderIcon(DocumentIcon)
      },
      {
        label: link('文件对比', '/compare/file'),
        key: 'compare-file',
        icon: renderIcon(DocumentIcon)
      }
    ]
  },
  {
    label: '舞，舞，舞',
    key: 'dance-dance-dance2',
    icon: renderIcon(BookIcon),
    children: [
      {
        label: '叙事者',
        key: 'narrator',
        icon: renderIcon(PersonIcon)
      },
      {
        label: '羊男',
        key: 'sheep-man',
        icon: renderIcon(PersonIcon)
      }
    ]
  }
])
const activeKey = ref('home')
const collapsed = ref(false)
const menuInstRef = ref(null)
const router = useRouter()

// 监听路由发生改变
watch(() => router.currentRoute.value.path, (path) => {
  // 处理home
  if (path === '/') {
    activeKey.value = 'home';
    return;
  }
  // /home -> home /compare/file -> compare-file
  let paths = path.split('/');
  if (paths.length > 1) {
    paths = paths.slice(1);
    activeKey.value = paths.join('-');
    menuInstRef.value?.showOption(activeKey.value)
  }
})
</script>

<style scoped>
.layout {
  height: 100vh;
}

.n-layout-header {
  padding: 24px;
}
</style>