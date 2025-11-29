import { createRouter, createWebHistory } from 'vue-router'
import Home from '../pages/Home.vue'
import Admin from '../pages/Admin.vue'
import JadwalTayang from '../pages/JadwalTayang.vue'
import Movies from '../pages/Movies.vue'
import Users from '../pages/Users.vue'
import Loginpages from '../pages/loginpages.vue'
import Regispage from '../pages/regispage.vue'

const routes = [
  // ===== PUBLIC ROUTES =====
  {
    path: '/',
    name: 'Home',
    component: Home
  },
  {
    path: '/bioskop/:slug',        
    name: 'CinemaSchedule',
    component: CinemaSchedule,
    props: true
  },

  // ===== ADMIN ROUTES =====
  {
    path: '/admin',
    name: 'Admin',
    component: Admin
  },
  {
    path: '/admin/jadwaltayang',
    name: 'Schedule',
    component: JadwalTayang
  },
  {
    path: '/admin/movies',
    name: 'Movies',
    component: Movies
  },
  {
    path: '/admin/users',
    name: 'Users',
    component: Users
  },
  {
    path: '/loginpages',
    name: 'Loginpages',
    component: Loginpages
  },
  {
    path: '/regispage',
    name: 'Regispage',
    component: Regispage
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router
