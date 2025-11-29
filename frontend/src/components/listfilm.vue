<script setup>
import { ref } from 'vue';

// ===== Movie Data =====
const movies = ref([
  {
    id: 1,
    title: 'Pesugihan Sate Gagak',
    genre: 'Action, Adventure',
    description: 'Setelah peristiwa dahsyat di Infinity War, alam semesta hancur. Para...',
    rating: 4.8,
    duration: 181,
    status: 'Sedang Tayang',
    image: new URL('../assets/film-1.webp', import.meta.url).href
  },
  {
    id: 2,
    title: 'Pangku',
    genre: 'Action, Crime, Drama',
    description: 'Ketika seorang pembunuh sadis mulai membunuh tokoh politik...',
    rating: 4.6,
    duration: 176,
    status: 'Sedang Tayang',
    image: new URL('../assets/film-2.webp', import.meta.url).href
  },
  {
    id: 3,
    title: 'Dopamin',
    genre: 'Sci-Fi, Adventure',
    description: 'Paul Atreides bersatu dengan Chani dan Fremen sambil mencar...',
    rating: 4.9,
    duration: 166,
    status: 'Sedang Tayang',
    image: new URL('../assets/film-3.webp', import.meta.url).href
  },
  {
    id: 4,
    title: 'Danyang Wingit Jumat Kliwon',
    genre: 'Biography, Drama, History',
    description: 'Kisah J. Robert Oppenheimer dan perannnya dalam pengembangan...',
    rating: 4.7,
    duration: 180,
    status: 'Sedang Tayang',
    image: new URL('../assets/film-4.webp', import.meta.url).href
  },
  {
    id: 5,
    title: 'Wicked: For Good',
    genre: 'Musical, Fantasy',
    description: 'Sebuah kisah tentang persahabatan yang tak terduga antara dua penyihir...',
    rating: 4.5,
    duration: 160,
    status: 'Sedang Tayang',
    image: new URL('../assets/film-5.webp', import.meta.url).href
  },
  {
    id: 6,
    title: 'Now You See Me: Now You Dont',
    genre: 'Action, Thriller',
    description: 'Sekumpulan pesulap jalanan melakukan serangkaian perampokan bank...',
    rating: 4.3,
    duration: 129,
    status: 'Sedang Tayang',
    image: new URL('../assets/film-6.webp', import.meta.url).href
  },
  {
    id: 7,
    title: 'The Running Man',
    genre: 'Action, Sci-Fi',
    description: 'Dalam dunia distopia masa depan, seorang pria terpaksa mengikuti...',
    rating: 4.4,
    duration: 123,
    status: 'Sedang Tayang',
    image: new URL('../assets/film-7.webp', import.meta.url).href
  },
  {
    id: 8,
    title: 'Keeper',
    genre: 'Thriller, Mystery',
    description: 'Seorang penjaga museum menemukan rahasia kelam di balik koleksi...',
    rating: 4.2,
    duration: 115,
    status: 'Sedang Tayang',
    image: new URL('../assets/film-8.webp', import.meta.url).href
  }
]);
</script>

<template>
  <div class="p-6">
    <div class="max-w-7xl mx-auto">
      
      <!-- Header Section -->
      <div class="flex items-center justify-between mb-8">
        <div>
          <h1 class="text-3xl font-bold text-gray-900">Manajemen Film</h1>
        </div>
        
        <!-- Add Movie Button -->
        <button class="bg-blue-900 hover:bg-blue-800 text-white font-semibold px-6 py-3 rounded-lg flex items-center gap-2 transition shadow-md hover:shadow-lg">
          <i class="bx bx-plus text-2xl"></i>
          <span>Tambah Film</span>
        </button>
      </div>

      <!-- Movies Grid -->
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <div
          v-for="movie in movies"
          :key="movie.id"
          class="bg-white rounded-xl shadow-lg overflow-hidden hover:shadow-2xl transition-all duration-300 relative group"
        >
          <!-- Status Badge -->
          <div class="absolute top-3 left-3 z-10">
            <span class="bg-green-500 text-white text-xs font-semibold px-3 py-1 rounded-full shadow-md">
              {{ movie.status }}
            </span>
          </div>

          <!-- Movie Poster -->
          <div class="relative overflow-hidden h-64">
            <img
              :src="movie.image"
              :alt="movie.title"
              class="w-full h-full object-cover transform transition-transform duration-500 group-hover:scale-110"
            >
          </div>

          <!-- Movie Info -->
          <div class="p-4">
            <h3 class="text-lg font-bold text-gray-900 mb-1">{{ movie.title }}</h3>
            <p class="text-sm text-gray-600 mb-2">{{ movie.genre }}</p>
            <p class="text-sm text-gray-500 mb-3 line-clamp-2">{{ movie.description }}</p>
            
            <!-- Rating & Duration -->
            <div class="flex items-center gap-4 text-sm text-gray-600">
              <span class="flex items-center gap-1">
                <i class="bx bxs-star text-yellow-500"></i>
                {{ movie.rating }}
              </span>
              <span class="flex items-center gap-1">
                <i class="bx bx-time-five"></i>
                {{ movie.duration }}m
              </span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>
