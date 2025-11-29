import { createRouter, createWebHistory } from 'vue-router'
import Home from '../pages/Home.vue'
import Admin from '../pages/Admin.vue'
import JadwalTayang from '../pages/JadwalTayang.vue'
import Movies from '../pages/Movies.vue'
import Users from '../pages/Users.vue'
import PemesananFilm from '../components/PemesananFilm.vue'
import Loginpages from '../pages/loginpages.vue'
import Regispage from '../pages/regispages.vue'
import CinemaSchedule from '../pages/CinemaSchedule.vue'

const routes = [
  {
    path: '/',
    name: 'Home',
    component: Home
  },
  {
    path: '/pemesanan/:movieId',
    name: 'PemesananFilm',
    component: PemesananFilm
  },
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
    path: '/cinema/:slug',
    name: 'CinemaSchedule',
    component: CinemaSchedule
  },
  {
    path: '/login',
    name: 'Login',
    component: Loginpages
  },
  {
    path: '/register',
    name: 'Register',
    component: Regispage
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router
