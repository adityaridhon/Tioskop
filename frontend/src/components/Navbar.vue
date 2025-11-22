<script setup>
import { ref } from 'vue'

const isMenuOpen = ref(false)

const toggleMenu = () => {
  isMenuOpen.value = !isMenuOpen.value
}
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
        <div class="hidden md:flex items-center space-x-8">
          <a href="#" class="text-gray-800 hover:text-blue-600 font-medium transition duration-300">Cari Film</a>
          <a href="#" class="text-gray-800 hover:text-blue-600 font-medium transition duration-300">Now Showing</a>
          <a href="#" class="text-gray-800 hover:text-blue-600 font-medium transition duration-300">Bioskop</a>
          <a href="#" class="flex items-center gap-2 px-4 py-2 bg-blue-50 hover:bg-blue-100 text-blue-700 font-medium rounded-lg transition duration-300">
            <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M5.05 4.05a7 7 0 119.9 9.9L10 18.9l-4.95-4.95a7 7 0 010-9.9zM10 11a2 2 0 100-4 2 2 0 000 4z" clip-rule="evenodd"/>
            </svg>
            <span>Balikpapan</span>
          </a>
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
          <a href="#" class="text-gray-800 hover:text-blue-600 font-medium transition duration-300">Cari Film</a>
          <a href="#" class="text-gray-800 hover:text-blue-600 font-medium transition duration-300">Now Showing</a>
          <a href="#" class="text-gray-800 hover:text-blue-600 font-medium transition duration-300">Bioskop</a>
          <a href="#" class="flex items-center gap-2 px-4 py-2 bg-blue-50 hover:bg-blue-100 text-blue-700 font-medium rounded-lg transition duration-300">
            <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M5.05 4.05a7 7 0 119.9 9.9L10 18.9l-4.95-4.95a7 7 0 010-9.9zM10 11a2 2 0 100-4 2 2 0 000 4z" clip-rule="evenodd"/>
            </svg>
            <span>Balikpapan</span>
          </a>
        </div>
      </div>
    </div>
  </nav>
</template>

<style scoped>
</style>