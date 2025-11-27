<script setup>
import { ref, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { useMovies } from '@/composables/useApi';
import { useShowtimes } from '@/composables/useApi';
import { useStudios } from '@/composables/useApi';

const router = useRouter();
const dropdowns = ref({});

const toggleDropdown = (id) => {
  dropdowns.value[id] = !dropdowns.value[id];
};

// Use API composables
const { movies: moviesData, loading: moviesLoading, fetchAll: fetchMovies, create: createMovie } = useMovies();
const { showtimes, loading: showtimesLoading, fetchAll: fetchShowtimes, create: createShowtime } = useShowtimes();
const { studios, loading: studiosLoading, fetchAll: fetchStudios } = useStudios();

// Transform movies data
const movies = ref([]);
const schedules = ref([]);

// Modal states
const showMovieModal = ref(false);
const showScheduleModal = ref(false);

// Form data
const movieForm = ref({
  title: '',
  genre: '',
  rating: '',
  duration: '',
  description: '',
  poster_url: '',
  release_date: ''
});

const scheduleForm = ref({
  movie_id: '',
  studio_id: '',
  start_time: '',
  price: ''
});

// Quick actions - Open modals instead of navigation
const goToAddMovie = () => {
  movieForm.value = {
    title: '',
    genre: '',
    rating: '',
    duration: '',
    description: '',
    poster_url: '',
    release_date: new Date().toISOString().split('T')[0]
  };
  showMovieModal.value = true;
};

const goToAddSchedule = () => {
  scheduleForm.value = {
    movie_id: '',
    studio_id: '',
    start_time: '',
    price: ''
  };
  showScheduleModal.value = true;
};

const goToAllMovies = () => {
  router.push('/admin/movies');
};

const goToAllSchedules = () => {
  router.push('/admin/jadwal-tayang');
};

// Modal close functions
const closeMovieModal = () => {
  showMovieModal.value = false;
};

const closeScheduleModal = () => {
  showScheduleModal.value = false;
};

// Submit handlers
const handleMovieSubmit = async () => {
  try {
    await createMovie(movieForm.value);
    alert('Film berhasil ditambahkan!');
    closeMovieModal();
    await fetchMovies();
    // Refresh movies display
    if (moviesData.value) {
      movies.value = moviesData.value.slice(0, 5).map(movie => ({
        id: movie.id,
        title: movie.title,
        genre: movie.genre,
        duration: `${movie.duration} min`,
        rating: movie.rating || 'N/A',
        status: 'Now Showing',
        poster: movie.poster_url || new URL('../assets/film-1.webp', import.meta.url).href
      }));
    }
  } catch (err) {
    alert(`Error: ${err.message || 'Gagal menambahkan film'}`);
  }
};

const handleScheduleSubmit = async () => {
  try {
    // Format the datetime for backend (combine date and time)
    const payload = {
      movie_id: parseInt(scheduleForm.value.movie_id),
      studio_id: parseInt(scheduleForm.value.studio_id),
      start_time: scheduleForm.value.start_time.replace('T', ' ') + ':00',
      price: scheduleForm.value.price.toString()
    };
    
    await createShowtime(payload);
    alert('Jadwal berhasil ditambahkan!');
    closeScheduleModal();
    await fetchShowtimes();
    // Refresh schedules display
    if (showtimes.value && moviesData.value && studios.value) {
      schedules.value = showtimes.value.slice(0, 4).map(showtime => {
        const movie = moviesData.value.find(m => m.id === showtime.movie_id);
        const studio = studios.value.find(s => s.id === showtime.studio_id);
        const startTime = new Date(showtime.start_time);
        
        return {
          id: showtime.id,
          movie: movie?.title || 'Unknown',
          theater: studio?.name || 'Unknown',
          time: startTime.toLocaleTimeString('id-ID', { hour: '2-digit', minute: '2-digit' }),
          date: startTime.toLocaleDateString('id-ID'),
          seats: `0/${studio?.capacity || 0}`,
          price: `Rp ${parseFloat(showtime.price).toLocaleString('id-ID')}`
        };
      });
    }
  } catch (err) {
    alert(`Error: ${err.message || 'Gagal menambahkan jadwal'}`);
  }
};

// Fetch data on mount
onMounted(async () => {
  await Promise.all([fetchMovies(), fetchShowtimes(), fetchStudios()]);
  
  // Transform movies
  if (moviesData.value) {
    movies.value = moviesData.value.slice(0, 5).map(movie => ({
      id: movie.id,
      title: movie.title,
      genre: movie.genre,
      duration: `${movie.duration} min`,
      rating: movie.rating || 'N/A',
      status: 'Now Showing',
      poster: movie.poster_url || new URL('../assets/film-1.webp', import.meta.url).href
    }));
  }
  
  // Transform schedules
  if (showtimes.value && moviesData.value && studios.value) {
    schedules.value = showtimes.value.slice(0, 4).map(showtime => {
      const movie = moviesData.value.find(m => m.id === showtime.movie_id);
      const studio = studios.value.find(s => s.id === showtime.studio_id);
      const startTime = new Date(showtime.start_time);
      
      return {
        id: showtime.id,
        movie: movie?.title || 'Unknown',
        theater: studio?.name || 'Unknown',
        time: startTime.toLocaleTimeString('id-ID', { hour: '2-digit', minute: '2-digit' }),
        date: startTime.toLocaleDateString('id-ID'),
        seats: `0/${studio?.capacity || 0}`,
        price: `Rp ${parseFloat(showtime.price).toLocaleString('id-ID')}`
      };
    });
  }
});
</script>

<template>
  <div class="p-6">
    <!-- Quick Actions Section -->
    <div class="mb-6">
      <h2 class="text-xl font-semibold text-gray-800 mb-4">Quick Actions</h2>
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <!-- Add Film Button -->
        <button @click="goToAddMovie" class="bg-blue-900 hover:bg-blue-800 text-white rounded-lg p-5 shadow-md shadow-blue-900/20 hover:shadow-lg hover:shadow-blue-800/30 transition-all duration-300 group">
          <div class="flex items-center justify-between">
            <div class="flex items-center gap-3">
              <div class="bg-white/20 p-3 rounded-lg group-hover:bg-white/30 transition-colors">
                <i class="bx bx-movie text-3xl"></i>
              </div>
              <div class="text-left">
                <h3 class="text-lg font-bold mb-0.5">Tambah Film</h3>
                <p class="text-blue-100 text-xs">Buat entri film baru</p>
              </div>
            </div>
            <i class="bx bx-plus-circle text-3xl opacity-70 group-hover:opacity-100 transition-opacity"></i>
          </div>
        </button>

        <!-- Add Schedule Button -->
        <button @click="goToAddSchedule" class="bg-blue-900 hover:bg-blue-800 text-white rounded-lg p-5 shadow-md shadow-blue-900/20 hover:shadow-lg hover:shadow-blue-800/30 transition-all duration-300 group">
          <div class="flex items-center justify-between">
            <div class="flex items-center gap-3">
              <div class="bg-white/20 p-3 rounded-lg group-hover:bg-white/30 transition-colors">
                <i class="bx bx-calendar-plus text-3xl"></i>
              </div>
              <div class="text-left">
                <h3 class="text-lg font-bold mb-0.5">Tambah Jadwal</h3>
                <p class="text-blue-100 text-xs">Buat jadwal tayang baru</p>
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
          <button @click="goToAllMovies" class="text-blue-600 hover:text-blue-700 text-sm font-medium flex items-center gap-1">
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
          <button @click="goToAllSchedules" class="text-blue-600 hover:text-blue-700 text-sm font-medium flex items-center gap-1">
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

    <!-- Modal Form Tambah Film -->
    <div v-if="showMovieModal" @click="closeMovieModal" class="fixed inset-0 bg-black/20 backdrop-blur-sm flex items-center justify-center z-50 p-4">
      <div @click.stop class="bg-white rounded-xl shadow-2xl max-w-2xl w-full max-h-[90vh] overflow-y-auto animate-slideUp">
        <div class="sticky top-0 bg-gradient-to-r from-blue-600 to-blue-700 px-6 py-4 flex justify-between items-center rounded-t-xl">
          <h3 class="text-xl font-bold text-white flex items-center gap-2">
            <i class="bx bx-movie-play text-2xl"></i>
            Tambah Film Baru
          </h3>
          <button @click="closeMovieModal" class="text-white/80 hover:text-white transition-colors">
            <i class="bx bx-x text-3xl"></i>
          </button>
        </div>
        
        <form @submit.prevent="handleMovieSubmit" class="p-6 space-y-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">Judul Film</label>
            <input v-model="movieForm.title" type="text" required
              class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent">
          </div>

          <div class="grid grid-cols-2 gap-4">
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-2">Genre</label>
              <input v-model="movieForm.genre" type="text" required
                class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent">
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-2">Rating</label>
              <input v-model="movieForm.rating" type="number" step="0.1" min="0" max="10" required
                class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent">
            </div>
          </div>

          <div class="grid grid-cols-2 gap-4">
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-2">Durasi (menit)</label>
              <input v-model="movieForm.duration" type="number" min="1" required
                class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent">
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-2">Tanggal Rilis</label>
              <input v-model="movieForm.release_date" type="date" required
                class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent">
            </div>
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">URL Poster</label>
            <input v-model="movieForm.poster_url" type="url" required
              class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
              placeholder="https://example.com/poster.jpg">
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">Deskripsi</label>
            <textarea v-model="movieForm.description" rows="4" required
              class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"></textarea>
          </div>

          <div class="flex justify-end gap-3 pt-4">
            <button type="button" @click="closeMovieModal"
              class="px-6 py-2 border border-gray-300 rounded-lg text-gray-700 hover:bg-gray-50 transition-colors">
              Batal
            </button>
            <button type="submit"
              class="px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors flex items-center gap-2">
              <i class="bx bx-save"></i>
              Simpan Film
            </button>
          </div>
        </form>
      </div>
    </div>

    <!-- Modal Form Tambah Jadwal -->
    <div v-if="showScheduleModal" @click="closeScheduleModal" class="fixed inset-0 bg-black/20 backdrop-blur-sm flex items-center justify-center z-50 p-4">
      <div @click.stop class="bg-white rounded-xl shadow-2xl max-w-lg w-full animate-slideUp">
        <div class="bg-gradient-to-r from-blue-600 to-blue-700 px-6 py-4 flex justify-between items-center rounded-t-xl">
          <h3 class="text-xl font-bold text-white flex items-center gap-2">
            <i class="bx bx-calendar-star text-2xl"></i>
            Tambah Jadwal Tayang
          </h3>
          <button @click="closeScheduleModal" class="text-white/80 hover:text-white transition-colors">
            <i class="bx bx-x text-3xl"></i>
          </button>
        </div>
        
        <form @submit.prevent="handleScheduleSubmit" class="p-6 space-y-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">Film</label>
            <select v-model="scheduleForm.movie_id" required
              class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent">
              <option value="" disabled>Pilih Film</option>
              <option v-for="movie in moviesData" :key="movie.id" :value="movie.id">
                {{ movie.title }}
              </option>
            </select>
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">Studio</label>
            <select v-model="scheduleForm.studio_id" required
              class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent">
              <option value="" disabled>Pilih Studio</option>
              <option v-for="studio in studios" :key="studio.id" :value="studio.id">
                {{ studio.name }} ({{ studio.type }})
              </option>
            </select>
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">Waktu Tayang</label>
            <input v-model="scheduleForm.start_time" type="datetime-local" required
              class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent">
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">Harga Tiket (Rp)</label>
            <input v-model="scheduleForm.price" type="number" min="0" step="1000" required
          <div class="flex justify-end gap-3 pt-4">
            <button type="button" @click="closeScheduleModal"
              class="px-6 py-2 border border-gray-300 rounded-lg text-gray-700 hover:bg-gray-50 transition-colors">
              Batal
            </button>
            <button type="submit"
              class="px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors flex items-center gap-2">
              <i class="bx bx-save"></i>
              Simpan Jadwal
            </button>
          </div>
        </form>
      </div>
    </div>

  </div>
</template>

<style scoped>
@keyframes slideUp {
  from {
    opacity: 0;
    transform: translateY(20px) scale(0.95);
  }
  to {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
}

.animate-slideUp {
  animation: slideUp 0.3s ease-out;
}
</style>
