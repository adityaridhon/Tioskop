<script setup>
import { ref, computed, onMounted } from 'vue'
import { RouterLink } from 'vue-router'
import { useAuth } from '@/composables/useAuth'

const isMenuOpen = ref(false)
const profileDropdownOpen = ref(false)
const showLogoutModal = ref(false)

const { user, fetchProfile, logout } = useAuth()

const isAuthenticated = computed(() => !!user.value)
const displayName = computed(() => user.value?.name || user.value?.email || 'Pengguna')
const displayRole = computed(() => {
  const role = user.value?.role || 'Customer'
  return role.charAt(0).toUpperCase() + role.slice(1).toLowerCase()
})

const toggleMenu = () => {
  isMenuOpen.value = !isMenuOpen.value
}

const scrollToSearch = () => {
  const searchSection = document.getElementById('search-section')
  if (searchSection) {
    searchSection.scrollIntoView({ 
      behavior: 'smooth',
      block: 'start'
    })
    // Close mobile menu if open
    isMenuOpen.value = false
  }
}

const toggleProfileDropdown = () => {
  profileDropdownOpen.value = !profileDropdownOpen.value
}

const openLogoutModal = () => {
  showLogoutModal.value = true
  profileDropdownOpen.value = false
}

const closeLogoutModal = () => {
  showLogoutModal.value = false
}

const confirmLogout = () => {
  logout()
  showLogoutModal.value = false
  isMenuOpen.value = false
}

onMounted(() => {
  if (!user.value) {
    fetchProfile().catch(() => {})
  }
})
</script>

<template>
  <nav class="bg-white shadow-lg">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="flex justify-between items-center h-16">
        <!-- Logo Text with Custom Shape -->
        <div class="shrink-0">
          <div class="relative">
            <!-- Background Shape -->
            <div class="absolute inset-0 bg-linear-to-br from-blue-900 to-blue-950 rounded-2xl transform -skew-x-6 shadow-lg"></div>
            <!-- Text -->
            <h1 class="relative text-2xl font-bold px-6 py-2 text-white">
              TIOSKOP
            </h1>
          </div>
        </div>

        <!-- Desktop Menu -->
        <div class="hidden md:flex items-center space-x-6">
          <a @click.prevent="scrollToSearch" href="#search-section" class="text-gray-800 hover:text-blue-600 font-medium transition duration-300 cursor-pointer">Cari Film</a>
          <a href="#" class="text-gray-800 hover:text-blue-600 font-medium transition duration-300">Now Showing</a>
          <a href="#" class="text-gray-800 hover:text-blue-600 font-medium transition duration-300">Bioskop</a>
          <a href="#" class="flex items-center gap-2 px-4 py-2 bg-blue-50 hover:bg-blue-100 text-blue-700 font-medium rounded-lg transition duration-300">
            <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M5.05 4.05a7 7 0 119.9 9.9L10 18.9l-4.95-4.95a7 7 0 010-9.9zM10 11a2 2 0 100-4 2 2 0 000 4z" clip-rule="evenodd"/>
            </svg>
            <span>Balikpapan</span>
          </a>

          <template v-if="isAuthenticated">
            <div class="relative">
              <button type="button" @click="toggleProfileDropdown" class="flex flex-col items-start px-4 py-2 bg-gray-100 hover:bg-gray-200 rounded-xl transition">
                <span class="text-sm font-semibold text-gray-800">{{ displayName }}</span>
                <span class="text-xs text-gray-500">{{ displayRole }}</span>
              </button>
              <ul v-if="profileDropdownOpen" class="absolute right-0 mt-2 w-40 bg-white rounded-lg shadow-lg border border-gray-100 py-2 z-40">
                <li>
                  <span class="block px-4 py-2 text-xs text-gray-500">Sedang masuk</span>
                </li>
                <li>
                  <button type="button" @click="openLogoutModal" class="w-full text-left px-4 py-2 text-sm text-red-600 hover:bg-red-50">Logout</button>
                </li>
              </ul>
            </div>
          </template>
          <template v-else>
            <RouterLink to="/login" class="text-gray-800 hover:text-blue-600 font-medium transition duration-300">Masuk</RouterLink>
            <RouterLink to="/register" class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 font-semibold transition">Daftar</RouterLink>
          </template>
        </div>

        <!-- Mobile menu button -->
        <div class="md:hidden">
          <button @click="toggleMenu" class="text-gray-700 hover:text-blue-600 focus:outline-none">
            <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path v-if="!isMenuOpen" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"/>
              <path v-else stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
            </svg>
          </button>
        </div>
      </div>

      <!-- Mobile Menu -->
      <div v-if="isMenuOpen" class="md:hidden pb-4">
        <div class="flex flex-col space-y-3">
          <a @click.prevent="scrollToSearch" href="#search-section" class="text-gray-800 hover:text-blue-600 font-medium transition duration-300 cursor-pointer">Cari Film</a>
          <a href="#" class="text-gray-800 hover:text-blue-600 font-medium transition duration-300">Now Showing</a>
          <a href="#" class="text-gray-800 hover:text-blue-600 font-medium transition duration-300">Bioskop</a>
          <a href="#" class="flex items-center gap-2 px-4 py-2 bg-blue-50 hover:bg-blue-100 text-blue-700 font-medium rounded-lg transition duration-300">
            <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M5.05 4.05a7 7 0 119.9 9.9L10 18.9l-4.95-4.95a7 7 0 010-9.9zM10 11a2 2 0 100-4 2 2 0 000 4z" clip-rule="evenodd"/>
            </svg>
            <span>Balikpapan</span>
          </a>
          <div class="border-t border-gray-200 pt-3">
            <template v-if="isAuthenticated">
              <div class="flex flex-col space-y-1">
                <span class="text-sm font-semibold text-gray-800">{{ displayName }}</span>
                <span class="text-xs text-gray-500">{{ displayRole }}</span>
                <button type="button" @click="openLogoutModal" class="mt-2 w-full px-4 py-2 bg-red-100 text-red-600 rounded-lg hover:bg-red-200">Logout</button>
              </div>
            </template>
            <template v-else>
              <div class="flex gap-3">
                <RouterLink to="/login" class="flex-1 text-center px-4 py-2 border border-blue-600 text-blue-600 rounded-lg">Masuk</RouterLink>
                <RouterLink to="/register" class="flex-1 text-center px-4 py-2 bg-blue-600 text-white rounded-lg">Daftar</RouterLink>
              </div>
            </template>
          </div>
        </div>
      </div>
    </div>
  </nav>

  <transition name="fade">
    <div v-if="showLogoutModal" class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 px-4">
      <div class="bg-white rounded-lg shadow-xl w-full max-w-sm p-6 text-center">
        <h3 class="text-lg font-semibold text-gray-900 mb-2">Keluar dari akun?</h3>
        <p class="text-sm text-gray-600 mb-6">Anda akan diarahkan ke halaman login setelah logout.</p>
        <div class="flex gap-3">
          <button type="button" @click="closeLogoutModal" class="flex-1 py-2 rounded-lg border border-gray-300 text-gray-700 hover:bg-gray-50">Batal</button>
          <button type="button" @click="confirmLogout" class="flex-1 py-2 rounded-lg bg-red-600 text-white hover:bg-red-700">Logout</button>
        </div>
      </div>
    </div>
  </transition>
</template>

<style scoped>
</style>