<script setup>
import { ref, computed } from 'vue';

// Data studio theaters
const studios = ref([
  { id: 0, name: 'Semua Studio', type: 'All', capacity: 0, schedules: 0 },
  { id: 1, name: 'Studio 1', type: 'Regular', capacity: 120, schedules: 4 },
  { id: 2, name: 'Studio 2', type: 'Premium', capacity: 150, schedules: 0 },
  { id: 3, name: 'Studio 3', type: 'IMAX', capacity: 180, schedules: 0 },
  { id: 4, name: 'Studio 4', type: 'Regular', capacity: 100, schedules: 0 },
  { id: 5, name: 'Studio 5', type: 'Premiere', capacity: 200, schedules: 0 }
]);

// Sample data untuk jadwal tayang
const schedules = ref([
  {
    id: 1,
    movie: 'Pesugihan Sate Gagak',
    studioId: 1,
    studio: 'Studio 1',
    studioType: 'Regular',
    studioCapacity: 120,
    time: '10:00',
    endTime: '13:01',
    date: '2025-11-26',
    duration: 181,
    price: 35000,
    soldSeats: 85,
    poster: new URL('../assets/film-1.webp', import.meta.url).href
  },
  {
    id: 2,
    movie: 'Pangku',
    studioId: 1,
    studio: 'Studio 1',
    studioType: 'Regular',
    studioCapacity: 120,
    time: '13:30',
    endTime: '16:26',
    date: '2025-11-26',
    duration: 176,
    price: 35000,
    soldSeats: 72,
    poster: new URL('../assets/film-2.webp', import.meta.url).href
  },
  {
    id: 3,
    movie: 'Dopamin',
    studioId: 1,
    studio: 'Studio 1',
    studioType: 'Regular',
    studioCapacity: 120,
    time: '17:00',
    endTime: '19:28',
    date: '2025-11-26',
    duration: 148,
    price: 40000,
    soldSeats: 115,
    poster: new URL('../assets/film-3.webp', import.meta.url).href
  },
  {
    id: 4,
    movie: 'Keeper',
    studioId: 1,
    studio: 'Studio 1',
    studioType: 'Regular',
    studioCapacity: 120,
    time: '20:00',
    endTime: '22:15',
    date: '2025-11-26',
    duration: 135,
    price: 40000,
    soldSeats: 98,
    poster: new URL('../assets/film-8.webp', import.meta.url).href
  },
  {
    id: 5,
    movie: 'Wicked: For Good',
    studioId: 2,
    studio: 'Studio 2',
    studioType: 'Premium',
    studioCapacity: 150,
    time: '13:00',
    endTime: '15:40',
    date: '2025-11-27',
    duration: 160,
    price: 60000,
    soldSeats: 45,
    poster: new URL('../assets/film-5.webp', import.meta.url).href
  },
  {
    id: 6,
    movie: 'Now You See Me: Now You Dont',
    studioId: 2,
    studio: 'Studio 2',
    studioType: 'Premium',
    studioCapacity: 150,
    time: '16:00',
    endTime: '18:09',
    date: '2025-11-27',
    duration: 129,
    price: 55000,
    soldSeats: 78,
    poster: new URL('../assets/film-6.webp', import.meta.url).href
  },
  {
    id: 7,
    movie: 'The Running Man',
    studioId: 3,
    studio: 'Studio 3',
    studioType: 'IMAX',
    studioCapacity: 180,
    time: '11:30',
    endTime: '13:45',
    date: '2025-11-26',
    duration: 135,
    price: 45000,
    soldSeats: 120,
    poster: new URL('../assets/film-7.webp', import.meta.url).href
  },
  {
    id: 8,
    movie: 'Keepers of the Flame',
    studioId: 3,
    studio: 'Studio 3',
    studioType: 'IMAX',
    studioCapacity: 180,
    time: '14:00',
    endTime: '16:21',
    date: '2025-11-26',
    duration: 141,
    price: 45000,
    soldSeats: 102,
    poster: new URL('../assets/film-7.webp', import.meta.url).href
  },
  {
    id: 9,
    movie: 'Midnight Sun',
    studioId: 4,
    studio: 'Studio 4',
    studioType: 'Regular',
    studioCapacity: 100,
    time: '15:15',
    endTime: '17:40',
    date: '2025-11-26',
    duration: 145,
    price: 40000,
    soldSeats: 60,
    poster: new URL('../assets/film-3.webp', import.meta.url).href
  },
  {
    id: 10,
    movie: 'Premiere Night',
    studioId: 5,
    studio: 'Studio 5',
    studioType: 'Premiere',
    studioCapacity: 200,
    time: '18:30',
    endTime: '21:10',
    date: '2025-11-26',
    duration: 160,
    price: 65000,
    soldSeats: 150,
    poster: new URL('../assets/film-5.webp', import.meta.url).href
  }
]);

// Filter berdasarkan tanggal dan studio
const selectedDate = ref('2025-11-26');
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
      grouped[schedule.studioId] = {
        studio: studios.value.find(s => s.id === schedule.studioId),
        schedules: []
      };
    }
    grouped[schedule.studioId].schedules.push(schedule);
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
</script>

<template>
  <div class="p-6">
    <div class="max-w-7xl mx-auto">
      
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
            <button class="bg-blue-900 hover:bg-blue-800 text-white font-semibold px-6 py-3 rounded-lg flex items-center gap-2 transition shadow-md hover:shadow-lg">
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
              'rounded-lg border-2 p-4 transition-all hover:shadow-md',
              getScheduleColor(index)
            ]"
          >
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
                  'rounded-lg border-2 p-4 transition-all hover:shadow-md',
                  getScheduleColor(index)
                ]"
              >
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
</template>

<style scoped>
/* Custom scrollbar for date selector if needed */
</style>
