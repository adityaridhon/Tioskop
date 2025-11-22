<script setup>
import { ref } from 'vue'

const searchQuery = ref('')

const handleSearch = () => {
  console.log('Searching for:', searchQuery.value)
  // Handle search logic here
}
</script>

<template>
  <section class="py-12 sm:py-16 md:py-20 bg-gray-50">
    <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
      <!-- Section Header -->
      <div class="text-center mb-8 sm:mb-10 md:mb-12">
        <h2 class="text-2xl sm:text-3xl md:text-4xl lg:text-5xl font-bold text-gray-900 mb-2 sm:mb-3 md:mb-4">
          Cari Film
        </h2>
        <p class="text-base sm:text-lg md:text-xl text-gray-600">
          Temukan jadwal tayang di semua bioskop Balikpapan
        </p>
      </div>

      <!-- Search Bar -->
      <div class="max-w-3xl mx-auto">
        <div class="relative bg-white rounded-2xl sm:rounded-3xl md:rounded-full shadow-lg sm:shadow-xl p-1.5 sm:p-2">
          <div class="flex flex-col sm:flex-row items-stretch sm:items-center gap-2 sm:gap-2">
            <div class="flex-1 flex items-center px-4 sm:pl-4 md:pl-6">
              <svg class="h-5 w-5 sm:h-6 sm:w-6 text-gray-400 mr-3 sm:mr-4 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
              </svg>
              <input 
                v-model="searchQuery"
                @keyup.enter="handleSearch"
                type="text" 
                placeholder="Masukkan judul film..." 
                class="w-full py-3 sm:py-3.5 md:py-4 text-base sm:text-lg focus:outline-none text-gray-700 placeholder-gray-400"
              >
            </div>
            <button 
              @click="handleSearch"
              class="group relative px-6 sm:px-8 py-3 sm:py-3.5 md:py-4 bg-linear-to-br from-blue-900 to-blue-950 hover:from-blue-800 hover:to-blue-900 text-white font-bold rounded-xl sm:rounded-2xl md:rounded-full transition-all duration-300 hover:scale-105 shadow-lg hover:shadow-2xl overflow-hidden"
            >
              <span class="relative z-10 flex items-center justify-center gap-2 text-sm sm:text-base">
                Cari
                <svg class="w-4 h-4 sm:w-5 sm:h-5 group-hover:translate-x-1 transition-transform duration-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
                </svg>
              </span>
              <!-- Animated glow -->
              <div class="absolute inset-0 opacity-0 group-hover:opacity-100 transition-opacity duration-300">
                <div class="absolute inset-0 bg-linear-to-r from-transparent via-white/30 to-transparent -translate-x-full group-hover:translate-x-full transition-transform duration-1000"></div>
              </div>
            </button>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>

<style scoped>
</style>