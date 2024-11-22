/*
 * @Author: Liuheng
 * @Date: 2024-10-24 16:48:33
 * @LastEditors: Liuheng
 * @LastEditTime: 2024-10-28 11:30:48
 * @Description: file content
 */
import { createRouter, createWebHistory } from 'vue-router'
import Main from '../views/Main.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: Main,
    },
  ],
})

export default router