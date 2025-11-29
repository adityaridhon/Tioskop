<script setup>
import { ref } from 'vue'

const searchQuery = ref('')
const movies = ref([])
const isLoading = ref(false)
const error = ref(null)
const hasSearched = ref(false)

const handleSearch = async () => {
  if (!searchQuery.value.trim()) {
    return
  }
  
  isLoading.value = true
  error.value = null
  hasSearched.value = true
  
  try {
    const query = `?q=${encodeURIComponent(searchQuery.value)}`
    const response = await fetch(`http://127.0.0.1:3000/api/movies${query}`)
    
    if (!response.ok) {
      throw new Error('Gagal mengambil data film')
    }
    
    movies.value = await response.json()
  } catch (err) {
    error.value = err.message
    console.error('Error fetching movies:', err)
  } finally {
    isLoading.value = false
  }
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

      <!-- Results Section -->
      <div class="mt-8 sm:mt-10 md:mt-12">
        <!-- Loading State -->
        <div v-if="isLoading" class="text-center py-12">
          <div class="inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-blue-900"></div>
          <p class="mt-4 text-gray-600">Mencari film...</p>
        </div>

        <!-- Error State -->
        <div v-else-if="error" class="text-center py-12">
          <div class="text-red-500 text-lg">{{ error }}</div>
        </div>

        <!-- No Results -->
        <div v-else-if="hasSearched && movies.length === 0" class="text-center py-12">
          <svg class="mx-auto h-16 w-16 text-gray-400 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 4v16M17 4v16M3 8h4m10 0h4M3 12h18M3 16h4m10 0h4M4 20h16a1 1 0 001-1V5a1 1 0 00-1-1H4a1 1 0 00-1 1v14a1 1 0 001 1z"/>
          </svg>
          <p class="text-gray-600 text-lg">Tidak ada film ditemukan</p>
        </div>

        <!-- Movie Results Grid -->
        <div v-else class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
          <div 
            v-for="movie in movies" 
            :key="movie.id"
            class="bg-white rounded-xl shadow-lg overflow-hidden hover:shadow-2xl transition-shadow duration-300"
          >
            <!-- Movie Poster -->
            <div class="h-64 bg-linear-to-br from-blue-900 to-blue-950 flex items-center justify-center overflow-hidden">
              <img 
                v-if="movie.poster_url" 
                :src="movie.poster_url" 
                :alt="movie.title"
                class="w-full h-full object-cover"
              />
              <svg v-else class="h-20 w-20 text-white/50" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 4v16M17 4v16M3 8h4m10 0h4M3 12h18M3 16h4m10 0h4M4 20h16a1 1 0 001-1V5a1 1 0 00-1-1H4a1 1 0 00-1 1v14a1 1 0 001 1z"/>
              </svg>
            </div>
            
            <!-- Movie Info -->
            <div class="p-5">
              <h3 class="text-xl font-bold text-gray-900 mb-2">{{ movie.title }}</h3>
              
              <div class="flex items-center gap-3 mb-3">
                <div v-if="movie.rating" class="flex items-center gap-1">
                  <svg class="h-5 w-5 text-yellow-400" fill="currentColor" viewBox="0 0 20 20">
                    <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z"/>
                  </svg>
                  <span class="text-gray-700 font-semibold">{{ movie.rating }}</span>
                </div>
                <span v-if="movie.year" class="text-gray-500">{{ movie.year }}</span>
              </div>
              
              <p v-if="movie.description" class="text-gray-600 text-sm line-clamp-3">
                {{ movie.description }}
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>

<style scoped>
</style>