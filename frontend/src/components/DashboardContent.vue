<script setup>
import { ref } from 'vue';

const dropdowns = ref({});

const toggleDropdown = (id) => {
  dropdowns.value[id] = !dropdowns.value[id];
};

// Sample data untuk movies
const movies = ref([
  {
    id: 1,
    title: 'Pesugihan Sate Gagak',
    genre: 'Horror',
    duration: '105 min',
    rating: '7.8',
    status: 'Now Showing',
    poster: new URL('../assets/film-1.webp', import.meta.url).href
  },
  {
    id: 2,
    title: 'Pangku',
    genre: 'Drama',
    duration: '98 min',
    rating: '8.2',
    status: 'Now Showing',
    poster: new URL('../assets/film-2.webp', import.meta.url).href
  },
  {
    id: 3,
    title: 'Dopamin',
    genre: 'Romance',
    duration: '110 min',
    rating: '7.5',
    status: 'Now Showing',
    poster: new URL('../assets/film-3.webp', import.meta.url).href
  },
  {
    id: 4,
    title: 'Danyang Wingit Jumat Kliwon',
    genre: 'Horror',
    duration: '95 min',
    rating: '7.3',
    status: 'Now Showing',
    poster: new URL('../assets/film-4.webp', import.meta.url).href
  },
  {
    id: 5,
    title: 'Wicked: For Good',
    genre: 'Musical',
    duration: '160 min',
    rating: '8.8',
    status: 'Now Showing',
    poster: new URL('../assets/film-5.webp', import.meta.url).href
  },
  {
    id: 6,
    title: 'Now You See Me: Now You Dont',
    genre: 'Action',
    duration: '129 min',
    rating: '7.9',
    status: 'Now Showing',
    poster: new URL('../assets/film-6.webp', import.meta.url).href
  },
  {
    id: 7,
    title: 'The Running Man',
    genre: 'Action',
    duration: '123 min',
    rating: '8.1',
    status: 'Now Showing',
    poster: new URL('../assets/film-7.webp', import.meta.url).href
  },
  {
    id: 8,
    title: 'Keeper',
    genre: 'Thriller',
    duration: '115 min',
    rating: '7.6',
    status: 'Now Showing',
    poster: new URL('../assets/film-8.webp', import.meta.url).href
  }
]);

// Sample data untuk schedules
const schedules = ref([
  { id: 1, movie: 'Pesugihan Sate Gagak', theater: 'Studio 1', time: '14:00', date: '2024-11-26', seats: '120/150', price: 'Rp 50.000' },
  { id: 2, movie: 'Pangku', theater: 'Studio 2', time: '16:30', date: '2024-11-26', seats: '80/150', price: 'Rp 50.000' },
  { id: 3, movie: 'Dopamin', theater: 'Studio 1', time: '19:00', date: '2024-11-26', seats: '45/150', price: 'Rp 55.000' },
  { id: 4, movie: 'Keeper', theater: 'Studio 3', time: '21:00', date: '2024-11-26', seats: '90/150', price: 'Rp 50.000' },
]);
</script>

<template>
  <div class="p-6">
    <!-- Quick Actions Section -->
    <div class="mb-6">
      <h2 class="text-xl font-semibold text-gray-800 mb-4">Quick Actions</h2>
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <!-- Add Film Button -->
        <button class="bg-blue-900 hover:bg-blue-800 text-white rounded-lg p-5 shadow-md shadow-blue-900/20 hover:shadow-lg hover:shadow-blue-800/30 transition-all duration-300 group">
          <div class="flex items-center justify-between">
            <div class="flex items-center gap-3">
              <div class="bg-white/20 p-3 rounded-lg group-hover:bg-white/30 transition-colors">
                <i class="bx bx-movie text-3xl"></i>
              </div>
              <div class="text-left">
                <h3 class="text-lg font-bold mb-0.5">Add New Film</h3>
                <p class="text-blue-100 text-xs">Create a new movie entry</p>
              </div>
            </div>
            <i class="bx bx-plus-circle text-3xl opacity-70 group-hover:opacity-100 transition-opacity"></i>
          </div>
        </button>

        <!-- Add Schedule Button -->
        <button class="bg-blue-900 hover:bg-blue-800 text-white rounded-lg p-5 shadow-md shadow-blue-900/20 hover:shadow-lg hover:shadow-blue-800/30 transition-all duration-300 group">
          <div class="flex items-center justify-between">
            <div class="flex items-center gap-3">
              <div class="bg-white/20 p-3 rounded-lg group-hover:bg-white/30 transition-colors">
                <i class="bx bx-calendar-plus text-3xl"></i>
              </div>
              <div class="text-left">
                <h3 class="text-lg font-bold mb-0.5">Add Schedule</h3>
                <p class="text-blue-100 text-xs">Create a new showtime</p>
              </div>
            </div>
            <i class="bx bx-plus-circle text-3xl opacity-70 group-hover:opacity-100 transition-opacity"></i>
          </div>
        </button>
      </div>
    </div>

    <!-- Tables Section -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-6">
      <!-- Now Showing Movies -->
      <div class="bg-white border border-gray-100 shadow-md shadow-black/5 p-6 rounded-lg">
        <div class="flex justify-between items-center mb-4">
          <h3 class="font-semibold text-lg text-gray-900">Now Showing Movies</h3>
          <button class="text-blue-600 hover:text-blue-700 text-sm font-medium flex items-center gap-1">
            <span>View All</span>
            <i class="bx bx-chevron-right"></i>
          </button>
        </div>
        <div class="space-y-4">
          <div v-for="movie in movies" :key="movie.id" class="flex items-center gap-4 p-3 hover:bg-gray-50 rounded-lg transition-colors">
            <img :src="movie.poster" :alt="movie.title" class="w-16 h-24 rounded object-cover">
            <div class="flex-1">
              <h4 class="font-semibold text-gray-900">{{ movie.title }}</h4>
              <div class="flex items-center gap-3 mt-1 text-sm text-gray-600">
                <span class="flex items-center gap-1">
                  <i class="bx bx-category"></i>
                  {{ movie.genre }}
                </span>
                <span class="flex items-center gap-1">
                  <i class="bx bx-time"></i>
                  {{ movie.duration }}
                </span>
                <span class="flex items-center gap-1">
                  <i class="bx bx-star text-yellow-500"></i>
                  {{ movie.rating }}
                </span>
              </div>
              <span :class="[
                'inline-block px-2 py-1 mt-2 text-xs font-medium rounded-full',
                movie.status === 'Now Showing' ? 'bg-green-100 text-green-700' : 'bg-blue-100 text-blue-700'
              ]">
                {{ movie.status }}
              </span>
            </div>
            <div class="flex gap-2">
              <button class="text-blue-600 hover:bg-blue-50 p-2 rounded transition-colors">
                <i class="bx bx-edit text-xl"></i>
              </button>
              <button class="text-red-600 hover:bg-red-50 p-2 rounded transition-colors">
                <i class="bx bx-trash text-xl"></i>
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Today's Schedules -->
      <div class="bg-white border border-gray-100 shadow-md shadow-black/5 p-6 rounded-lg">
        <div class="flex justify-between items-center mb-4">
          <h3 class="font-semibold text-lg text-gray-900">Today's Schedules</h3>
          <button class="text-blue-600 hover:text-blue-700 text-sm font-medium flex items-center gap-1">
            <span>View All</span>
            <i class="bx bx-chevron-right"></i>
          </button>
        </div>
        <div class="space-y-3">
          <div v-for="schedule in schedules" :key="schedule.id" class="p-4 border border-gray-200 rounded-lg hover:border-blue-300 transition-colors">
            <div class="flex justify-between items-start mb-2">
              <div>
                <h4 class="font-semibold text-gray-900">{{ schedule.movie }}</h4>
                <p class="text-sm text-gray-600 mt-1">{{ schedule.theater }}</p>
              </div>
              <span class="bg-blue-100 text-blue-700 px-3 py-1 rounded-full text-sm font-medium">
                {{ schedule.time }}
              </span>
            </div>
            <div class="flex justify-between items-center text-sm mt-3 pt-3 border-t border-gray-100">
              <div class="flex items-center gap-4">
                <span class="text-gray-600">
                  <i class="bx bx-chair"></i> {{ schedule.seats }}
                </span>
                <span class="text-green-600 font-medium">{{ schedule.price }}</span>
              </div>
              <button class="text-blue-600 hover:text-blue-700 font-medium">
                Edit
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>

  </div>
</template>
