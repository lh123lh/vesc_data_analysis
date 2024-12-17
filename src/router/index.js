/*
 * @Author: Liuheng
 * @Date: 2024-10-24 16:48:33
 * @LastEditors: Liuheng
 * @LastEditTime: 2024-10-28 11:30:48
 * @Description: file content
 */
import { createRouter, createWebHistory } from 'vue-router'
import Home from '../views/Home.vue'
import Data from '../views/DataProcess.vue'
import CheckGoodnessOfFit from '../views/CheckGoodnessOfFit.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: Home,
      meta: {
        //	当前页面要不要缓存
        keepAlive: true,
        //	当前页面层级
        deepth: 1,
      },
    },
    {
      path: '/dataProcess',
      name: 'dataProcess',
      component: Data,
      meta: {
        //	当前页面要不要缓存
        keepAlive: true,
        //	当前页面层级
        deepth: 2,
      },
    },
    {
      path: '/checkGoodnessOfFit',
      name: 'checkGoodnessOfFit',
      component: CheckGoodnessOfFit,
      meta: {
        //	当前页面要不要缓存
        keepAlive: true,
        //	当前页面层级
        deepth: 3,
      },
    },
  ],
})

export default router