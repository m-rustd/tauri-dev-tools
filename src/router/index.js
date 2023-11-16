import { createRouter, createWebHashHistory } from 'vue-router';

const routes = [
  {
    path: '/',
    name: 'Home',
    component: () => import('../views/home/index.vue')
  },
  {
    path: '/compare',
    name: 'Compare',
    children: [
      {
        path: 'content',
        name: 'CompareContent',
        component: () => import('../views/compare/index.vue')
      },
      {
        path: 'file',
        name: 'CompareFile',
        component: () => import('../views/compare/file.vue')
      },
    ]
  }
];

const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

export default router;