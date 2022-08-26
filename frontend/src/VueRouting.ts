import { createRouter, createWebHistory } from 'vue-router'

import AuthenticateUser  from './components/AuthenticateUser.vue'
import DashboardHome  from './components/admin/DashboardHome.vue'

const routes = [
  { path: '/', component: AuthenticateUser },
  { path: '/admin/dashboard', component: DashboardHome }
]

export const router = createRouter({
  history: createWebHistory(),
  routes,
})

