import { createRouter, createWebHistory } from 'vue-router'

import AuthenticateUser  from './components/AuthenticateUser.vue'
import DashboardHome  from './components/admin/DashboardHome.vue'
import DashboardWorksites from "./components/admin/DashboardWorksites.vue"
import DashboardClients from "./components/admin/DashboardClients.vue"

const routes = [
  { path: '/', component: AuthenticateUser },
  { path: '/admin/dashboard', component: DashboardHome },
  { path: '/admin/worksites', component: DashboardWorksites },
  { path: '/admin/clients', component: DashboardClients },
]

export const router = createRouter({
  history: createWebHistory(),
  routes,
})

