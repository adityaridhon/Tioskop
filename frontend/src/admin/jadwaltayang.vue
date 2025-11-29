<script setup>
import { ref, computed, onMounted } from 'vue';
import { useShowtimes } from '@/composables/useApi';
import { useStudios } from '@/composables/useApi';
import { useMovies } from '@/composables/useApi';

// Use API composables
const { showtimes, loading: showtimesLoading, error: showtimesError, fetchAll: fetchShowtimes, create: createShowtime, update: updateShowtime, remove: deleteShowtime } = useShowtimes();
const { studios: studiosData, loading: studiosLoading, error: studiosError, fetchAll: fetchStudios } = useStudios();
const { movies: moviesData, loading: moviesLoading, error: moviesError, fetchAll: fetchMovies } = useMovies();

// Combined loading and error states
const loading = computed(() => showtimesLoading.value || studiosLoading.value || moviesLoading.value);
const error = computed(() => showtimesError.value || studiosError.value || moviesError.value);

// Studios with "All" option
const studios = computed(() => {
  if (!studiosData.value) return [{ id: 0, name: 'Semua Studio', type: 'All', capacity: 0 }];
  
  const scheduleCount = {};
  schedules.value.forEach(schedule => {
    scheduleCount[schedule.studioId] = (scheduleCount[schedule.studioId] || 0) + 1;
  });

  return [
    { id: 0, name: 'Semua Studio', type: 'All', capacity: 0, schedules: schedules.value.length },
    ...studiosData.value.map(studio => ({
      ...studio,
      schedules: scheduleCount[studio.id] || 0
    }))
  ];
});

// Transform showtimes data to match the component's expected format
const schedules = computed(() => {
  if (!showtimes.value || !moviesData.value || !studiosData.value) return [];
  
  return showtimes.value.map(showtime => {
    const movie = moviesData.value.find(m => m.id === showtime.movie_id);
    const studio = studiosData.value.find(s => s.id === showtime.studio_id);
    
    if (!movie || !studio) return null;

    // Parse start_time (format: "2025-11-27 14:00:00")
    const startTime = new Date(showtime.start_time);
    const hours = startTime.getHours().toString().padStart(2, '0');
    const minutes = startTime.getMinutes().toString().padStart(2, '0');
    const time = `${hours}:${minutes}`;
    
    // Calculate end time based on movie duration
    const endTime = new Date(startTime.getTime() + movie.duration * 60000);
    const endHours = endTime.getHours().toString().padStart(2, '0');
    const endMinutes = endTime.getMinutes().toString().padStart(2, '0');
    const endTimeStr = `${endHours}:${endMinutes}`;
    
    // Format date (YYYY-MM-DD)
    const date = startTime.toISOString().split('T')[0];

    return {
      id: showtime.id,
      movie: movie.title,
      movieId: movie.id,
      studioId: studio.id,
      studio: studio.name,
      studioType: studio.type,
      studioCapacity: studio.capacity,
      time,
      endTime: endTimeStr,
      date,
      duration: movie.duration,
      price: parseFloat(showtime.price),
      soldSeats: 0, // TODO: Calculate from bookings
      poster: movie.poster_url || new URL('../assets/film-1.webp', import.meta.url).href
    };
  }).filter(Boolean); // Remove null entries
});

// Fetch data on mount
onMounted(async () => {
  await Promise.all([
    fetchShowtimes(),
    fetchStudios(),
    fetchMovies()
  ]);
});

// Filter berdasarkan tanggal dan studio
const selectedDate = ref(new Date().toISOString().split('T')[0]); // Default to today
const selectedStudio = ref(0);

// Fungsi untuk memilih tanggal
const selectDate = (direction) => {
  const currentDate = new Date(selectedDate.value);
  currentDate.setDate(currentDate.getDate() + direction);
  selectedDate.value = currentDate.toISOString().split('T')[0];
};

// Filter jadwal berdasarkan studio dan tanggal yang dipilih
const filteredSchedules = computed(() => {
  if (selectedStudio.value === 0) {
    // Show all studios
    return schedules.value.filter(schedule => schedule.date === selectedDate.value);
  }
  return schedules.value.filter(
    schedule => schedule.studioId === selectedStudio.value && schedule.date === selectedDate.value
  );
});

// Group schedules by studio for "All Studios" view
const groupedSchedules = computed(() => {
  if (selectedStudio.value !== 0) return null;
  
  const schedulesByDate = schedules.value.filter(schedule => schedule.date === selectedDate.value);
  const grouped = {};
  
  schedulesByDate.forEach(schedule => {
    if (!grouped[schedule.studioId]) {
      const studioData = studiosData.value?.find(s => s.id === schedule.studioId);
      if (studioData) {
        grouped[schedule.studioId] = {
          studio: studioData,
          schedules: []
        };
      }
    }
    if (grouped[schedule.studioId]) {
      grouped[schedule.studioId].schedules.push(schedule);
    }
  });
  
  return Object.values(grouped).sort((a, b) => a.studio.id - b.studio.id);
});

// Hitung persentase terisi untuk studio
const getStudioOccupancy = (studioId) => {
  if (studioId === 0) {
    // Calculate overall occupancy for all studios
    const allSchedules = schedules.value.filter(s => s.date === selectedDate.value);
    if (allSchedules.length === 0) return 0;
    const totalSeats = allSchedules.reduce((sum, s) => sum + s.studioCapacity, 0);
    const totalSold = allSchedules.reduce((sum, s) => sum + s.soldSeats, 0);
    return Math.round((totalSold / totalSeats) * 100);
  }
  const studioSchedules = schedules.value.filter(s => s.studioId === studioId && s.date === selectedDate.value);
  if (studioSchedules.length === 0) return 0;
  const totalSeats = studioSchedules[0].studioCapacity * studioSchedules.length;
  const totalSold = studioSchedules.reduce((sum, s) => sum + s.soldSeats, 0);
  return Math.round((totalSold / totalSeats) * 100);
};

// Hitung jumlah jadwal per studio untuk tanggal dipilih
const getStudioScheduleCount = (studioId) => {
  if (studioId === 0) {
    // Count all schedules for all studios
    return schedules.value.filter(s => s.date === selectedDate.value).length;
  }
  return schedules.value.filter(s => s.studioId === studioId && s.date === selectedDate.value).length;
};

// Format harga ke Rupiah
const formatPrice = (price) => {
  return `Rp ${price.toLocaleString('id-ID')}`;
};

// Pilih warna background berdasarkan index
const getScheduleColor = (index) => {
  const colors = [
    'bg-orange-50 border-orange-200',
    'bg-yellow-50 border-yellow-200',
    'bg-pink-50 border-pink-200',
    'bg-purple-50 border-purple-200'
  ];
  return colors[index % colors.length];
};

// Modal state for CRUD operations
const showModal = ref(false);
const modalMode = ref('create'); // 'create' or 'edit'
const formData = ref({
  movie_id: '',
  studio_id: '',
  start_time: '',
  price: ''
});

// Open modal for creating new schedule
const openCreateModal = () => {
  modalMode.value = 'create';
  formData.value = {
    movie_id: '',
    studio_id: '',
    start_time: '',
    price: ''
  };
  showModal.value = true;
};

// Open modal for editing schedule
const openEditModal = (schedule) => {
  modalMode.value = 'edit';
  // Find the original showtime data
  const showtime = showtimes.value.find(s => s.id === schedule.id);
  if (showtime) {
    formData.value = {
      id: showtime.id,
      movie_id: showtime.movie_id,
      studio_id: showtime.studio_id,
      start_time: showtime.start_time.replace(' ', 'T').substring(0, 16), // Format for datetime-local input
      price: showtime.price
    };
  }
  showModal.value = true;
};

// Close modal
const closeModal = () => {
  showModal.value = false;
  formData.value = {
    movie_id: '',
    studio_id: '',
    start_time: '',
    price: ''
  };
};

// Handle form submission
const handleSubmit = async () => {
  try {
    // Convert datetime-local format to MySQL datetime format
    const startTime = formData.value.start_time.replace('T', ' ') + ':00';
    
    const payload = {
      movie_id: parseInt(formData.value.movie_id),
      studio_id: parseInt(formData.value.studio_id),
      start_time: startTime,
      price: formData.value.price.toString()
    };

    if (modalMode.value === 'create') {
      await createShowtime(payload);
    } else {
      await updateShowtime(formData.value.id, payload);
    }

    await fetchShowtimes();
    closeModal();
  } catch (err) {
    console.error('Error saving schedule:', err);
    alert('Gagal menyimpan jadwal: ' + (err.message || 'Terjadi kesalahan'));
  }
};

// Handle delete schedule
const handleDelete = async (scheduleId, movieTitle) => {
  if (!confirm(`Apakah Anda yakin ingin menghapus jadwal "${movieTitle}"?`)) {
    return;
  }

  try {
    await deleteShowtime(scheduleId);
    await fetchShowtimes();
  } catch (err) {
    console.error('Error deleting schedule:', err);
    alert('Gagal menghapus jadwal: ' + (err.message || 'Terjadi kesalahan'));
  }
};

</script>

<template>
  <div class="p-6">
    <!-- Loading State -->
    <div v-if="loading" class="flex justify-center items-center min-h-screen">
      <div class="text-center">
        <div class="animate-spin rounded-full h-16 w-16 border-b-2 border-blue-900 mx-auto mb-4"></div>
        <p class="text-gray-600">Memuat data jadwal...</p>
      </div>
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="max-w-2xl mx-auto mt-8">
      <div class="bg-red-50 border border-red-200 rounded-lg p-6 text-center">
        <i class="bx bx-error-circle text-5xl text-red-500 mb-3"></i>
        <h3 class="text-lg font-semibold text-red-800 mb-2">Gagal Memuat Data</h3>
        <p class="text-red-600 mb-4">{{ error }}</p>
        <button 
          @click="() => { fetchShowtimes(); fetchStudios(); fetchMovies(); }"
          class="bg-red-600 hover:bg-red-700 text-white px-6 py-2 rounded-lg transition"
        >
          Coba Lagi
        </button>
      </div>
    </div>

    <!-- Main Content -->
    <div v-else class="max-w-7xl mx-auto">
      
      <!-- Header with Date Selector and Studio Dropdown -->
      <div class="bg-white rounded-lg shadow-sm p-6 mb-6">
        <div class="flex items-center justify-between">
          <!-- Date Navigator -->
          <div class="flex items-center gap-4">
            <button @click="selectDate(-1)" class="p-2 hover:bg-gray-100 rounded-full transition">
              <i class="bx bx-chevron-left text-2xl text-gray-600"></i>
            </button>
            
            <div class="flex items-center gap-3">
              <i class="bx bx-calendar text-gray-400 text-xl"></i>
              <div>
                <div class="text-sm text-gray-500">Tanggal Dipilih</div>
                <div class="text-lg font-semibold text-gray-900">
                  {{ new Date(selectedDate).toLocaleDateString('id-ID', { weekday: 'long', day: 'numeric', month: 'long', year: 'numeric' }) }}
                </div>
              </div>
            </div>
            
            <button @click="selectDate(1)" class="p-2 hover:bg-gray-100 rounded-full transition">
              <i class="bx bx-chevron-right text-2xl text-gray-600"></i>
            </button>
          </div>

          <!-- Add Schedule Button -->
          <div>
            <button @click="openCreateModal" class="bg-blue-900 hover:bg-blue-800 text-white font-semibold px-6 py-3 rounded-lg flex items-center gap-2 transition shadow-md hover:shadow-lg">
              <i class="bx bx-plus text-xl"></i>
              <span>Tambah Jadwal</span>
            </button>
          </div>
        </div>
      </div>

      <!-- Studio Cards -->
      <div class="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-6 gap-2 mb-6">
        <button
          v-for="studio in studios"
          :key="studio.id"
          @click="selectedStudio = studio.id"
          :class="[
            'bg-white rounded-lg p-2 border-2 transition-all text-left',
            selectedStudio === studio.id 
              ? 'border-blue-500 shadow-md' 
              : 'border-gray-200 hover:border-gray-300'
          ]"
        >
          <div class="flex items-center justify-between mb-1">
            <h3 class="font-semibold text-gray-900 text-sm">{{ studio.name }}</h3>
          </div>
          <div class="text-xs text-gray-600 mb-1.5">Kapasitas: {{ studio.capacity }} kursi</div>
          <div class="flex items-center justify-between text-xs">
            <span class="text-gray-600">{{ getStudioScheduleCount(studio.id) }} jadwal</span>
          </div>
        </button>
      </div>

      <!-- Timeline Jadwal -->
      <div class="bg-white rounded-lg shadow-sm p-6">
        <div class="flex items-center justify-between mb-6">
          <h2 class="text-xl font-semibold text-gray-900">Timeline Jadwal</h2>
          <span class="text-sm text-gray-500">{{ filteredSchedules.length }} jadwal hari ini</span>
        </div>

        <!-- Studio Header -->
        <div v-if="selectedStudio !== 0" class="mb-4 pb-4 border-b border-gray-200">
          <div class="flex items-center gap-3">
            <div class="w-10 h-10 bg-blue-100 rounded-lg flex items-center justify-center">
              <i class="bx bx-video text-blue-600 text-xl"></i>
            </div>
            <div>
              <h3 class="font-semibold text-gray-900">
                {{ studios.find(s => s.id === selectedStudio)?.name }}
              </h3>
              <p class="text-sm text-gray-500">
                {{ studios.find(s => s.id === selectedStudio)?.type }} - {{ studios.find(s => s.id === selectedStudio)?.capacity }} kursi
              </p>
            </div>
          </div>
        </div>

        <!-- Schedule Timeline for Single Studio -->
        <div v-if="selectedStudio !== 0" class="space-y-3">
          <div
            v-for="(schedule, index) in filteredSchedules"
            :key="schedule.id"
            :class="[
              'rounded-lg border-2 p-4 transition-all hover:shadow-md relative group',
              getScheduleColor(index)
            ]"
          >
            <!-- Edit/Delete Buttons (appear on hover) -->
            <div class="absolute top-2 right-2 opacity-0 group-hover:opacity-100 transition-opacity flex gap-2">
              <button 
                @click="openEditModal(schedule)"
                class="bg-blue-600 hover:bg-blue-700 text-white p-2 rounded-lg transition shadow-md"
                title="Edit Jadwal"
              >
                <i class="bx bx-edit text-lg"></i>
              </button>
              <button 
                @click="handleDelete(schedule.id, schedule.movie)"
                class="bg-red-600 hover:bg-red-700 text-white p-2 rounded-lg transition shadow-md"
                title="Hapus Jadwal"
              >
                <i class="bx bx-trash text-lg"></i>
              </button>
            </div>

            <div class="flex items-center justify-between">
              <!-- Left: Time & Movie Info -->
              <div class="flex items-start gap-4">
                <div class="text-center">
                  <div class="text-xs text-gray-500 mb-1">Mulai</div>
                  <div class="text-2xl font-bold text-gray-900">{{ schedule.time }}</div>
                  <div class="text-xs text-gray-500 mt-1">{{ schedule.endTime }}</div>
                </div>
                
                <div class="flex-1">
                  <h4 class="text-lg font-semibold text-gray-900 mb-2">{{ schedule.movie }}</h4>
                  <div class="flex items-center gap-4 text-sm text-gray-600">
                    <span class="flex items-center gap-1">
                      <i class="bx bx-time-five"></i>
                      {{ schedule.duration }} menit
                    </span>
                    <span class="flex items-center gap-1">
                      {{ formatPrice(schedule.price) }}
                    </span>
                  </div>
                </div>
              </div>

              <!-- Right: Seat Availability -->
              <div class="text-right">
                <div class="text-xs text-gray-500 mb-1">Terjual</div>
                <div class="text-2xl font-bold text-gray-900">{{ schedule.soldSeats }}/{{ schedule.studioCapacity }}</div>
                <div class="text-sm text-gray-600 mt-1">
                  {{ Math.round((schedule.soldSeats / schedule.studioCapacity) * 100) }}% penuh
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- Schedule Timeline for All Studios (Grouped by Studio) -->
        <div v-else class="space-y-8">
          <div v-for="(group, groupIndex) in groupedSchedules" :key="group.studio.id">
            <!-- Studio Header for Each Group -->
            <div class="mb-4 pb-3 border-b border-gray-200">
              <div class="flex items-center gap-3">
                <div class="w-10 h-10 bg-blue-100 rounded-lg flex items-center justify-center">
                  <i class="bx bx-video text-blue-600 text-xl"></i>
                </div>
                <div>
                  <h3 class="font-semibold text-gray-900">{{ group.studio.name }}</h3>
                  <p class="text-sm text-gray-500">
                    {{ group.studio.type }} - {{ group.studio.capacity }} kursi
                  </p>
                </div>
                <span class="ml-auto text-sm text-gray-600">{{ group.schedules.length }} jadwal hari ini</span>
              </div>
            </div>

            <!-- Schedules for This Studio -->
            <div class="space-y-3 mb-8">
              <div
                v-for="(schedule, index) in group.schedules"
                :key="schedule.id"
                :class="[
                  'rounded-lg border-2 p-4 transition-all hover:shadow-md relative group',
                  getScheduleColor(index)
                ]"
              >
                <!-- Edit/Delete Buttons (appear on hover) -->
                <div class="absolute top-2 right-2 opacity-0 group-hover:opacity-100 transition-opacity flex gap-2">
                  <button 
                    @click="openEditModal(schedule)"
                    class="bg-blue-600 hover:bg-blue-700 text-white p-2 rounded-lg transition shadow-md"
                    title="Edit Jadwal"
                  >
                    <i class="bx bx-edit text-lg"></i>
                  </button>
                  <button 
                    @click="handleDelete(schedule.id, schedule.movie)"
                    class="bg-red-600 hover:bg-red-700 text-white p-2 rounded-lg transition shadow-md"
                    title="Hapus Jadwal"
                  >
                    <i class="bx bx-trash text-lg"></i>
                  </button>
                </div>

                <div class="flex items-center justify-between">
                  <!-- Left: Time & Movie Info -->
                  <div class="flex items-start gap-4">
                    <div class="text-center">
                      <div class="text-xs text-gray-500 mb-1">Mulai</div>
                      <div class="text-2xl font-bold text-gray-900">{{ schedule.time }}</div>
                      <div class="text-xs text-gray-500 mt-1">{{ schedule.endTime }}</div>
                    </div>
                    
                    <div class="flex-1">
                      <h4 class="text-lg font-semibold text-gray-900 mb-2">{{ schedule.movie }}</h4>
                      <div class="flex items-center gap-4 text-sm text-gray-600">
                        <span class="flex items-center gap-1">
                          <i class="bx bx-time-five"></i>
                          {{ schedule.duration }} menit
                        </span>
                        <span class="flex items-center gap-1">
                          {{ formatPrice(schedule.price) }}
                        </span>
                      </div>
                    </div>
                  </div>

                  <!-- Right: Seat Availability -->
                  <div class="text-right">
                    <div class="text-xs text-gray-500 mb-1">Terjual</div>
                    <div class="text-2xl font-bold text-gray-900">{{ schedule.soldSeats }}/{{ schedule.studioCapacity }}</div>
                    <div class="text-sm text-gray-600 mt-1">
                      {{ Math.round((schedule.soldSeats / schedule.studioCapacity) * 100) }}% penuh
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- Empty State -->
        <div v-if="filteredSchedules.length === 0" class="text-center py-12">
          <i class="bx bx-calendar-x text-6xl text-gray-300 mb-4"></i>
          <h3 class="text-xl font-semibold text-gray-600 mb-2">Tidak ada jadwal tayang</h3>
          <p class="text-gray-500">Belum ada jadwal untuk studio dan tanggal yang dipilih.</p>
        </div>
      </div>
    </div>
    </div>

    <!-- Modal Form -->
    <div v-if="showModal" @click="closeModal" class="fixed inset-0 bg-black/20 backdrop-blur-sm flex items-center justify-center z-50 p-4">
      <div @click.stop class="bg-white rounded-xl shadow-2xl max-w-md w-full max-h-[90vh] overflow-y-auto animate-slideUp">
        <!-- Modal Header -->
        <div class="bg-gradient-to-r from-blue-600 to-blue-700 px-6 py-4 flex justify-between items-center rounded-t-xl">
          <h3 class="text-xl font-bold text-white flex items-center gap-2">
            <i class="bx bx-calendar-star text-2xl"></i>
            {{ modalMode === 'create' ? 'Tambah Jadwal Baru' : 'Edit Jadwal' }}
          </h3>
          <button @click="closeModal" class="text-white/80 hover:text-white transition-colors">
            <i class="bx bx-x text-3xl"></i>
          </button>
        </div>

        <!-- Modal Body -->
        <form @submit.prevent="handleSubmit" class="p-6 space-y-4">
          <!-- Movie Selection -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">Film</label>
            <select 
              v-model="formData.movie_id" 
              required
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            >
              <option value="">Pilih Film</option>
              <option v-for="movie in moviesData" :key="movie.id" :value="movie.id">
                {{ movie.title }} ({{ movie.duration }} min)
              </option>
            </select>
          </div>

          <!-- Studio Selection -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">Studio</label>
            <select 
              v-model="formData.studio_id" 
              required
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            >
              <option value="">Pilih Studio</option>
              <option v-for="studio in studiosData" :key="studio.id" :value="studio.id">
                {{ studio.name }} ({{ studio.type }})
              </option>
            </select>
          </div>

          <!-- Start Time -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">Waktu Mulai</label>
            <input 
              type="datetime-local" 
              v-model="formData.start_time" 
              required
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            />
          </div>

          <!-- Price -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">Harga (Rp)</label>
            <input 
              type="number" 
              v-model="formData.price" 
              required
              min="0"
              step="1000"
              placeholder="50000"
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            />
          </div>

          <!-- Modal Footer -->
          <div class="flex gap-3 pt-4">
            <button
              type="button"
              @click="closeModal"
              class="flex-1 px-4 py-2 border border-gray-300 text-gray-700 rounded-lg hover:bg-gray-50 transition"
            >
              Batal
            </button>
            <button
              type="submit"
              class="flex-1 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition flex items-center justify-center gap-2"
            >
              <i class="bx bx-save"></i>
              {{ modalMode === 'create' ? 'Tambah' : 'Simpan' }}
            </button>
          </div>
        </form>
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

/* Custom scrollbar for date selector if needed */
</style>
