<script setup>
import { ref, onMounted, computed } from 'vue';
import { useRoute } from 'vue-router';
import axios from 'axios';

const route = useRoute();
const API_BASE_URL = 'http://127.0.0.1:3000/api';

const showtimeId = ref(route.params.showtimeId);
const seats = ref([]);
const selectedSeats = ref([]);
const showtime = ref(null);
const loading = ref(false);

// Fetch available seats
const fetchSeats = async () => {
  loading.value = true;
  try {
    const response = await axios.get(
      `${API_BASE_URL}/seats/showtime/${showtimeId.value}/available`
    );
    if (response.data.success) {
      seats.value = response.data.data;
    }
  } catch (err) {
    console.error('Error fetching seats:', err);
  } finally {
    loading.value = false;
  }
};

// Toggle seat selection
const toggleSeat = (seatId) => {
  const index = selectedSeats.value.indexOf(seatId);
  if (index > -1) {
    selectedSeats.value.splice(index, 1);
  } else {
    selectedSeats.value.push(seatId);
  }
};

// Calculate total price
const totalPrice = computed(() => {
  if (!showtime.value) return 0;
  return selectedSeats.value.length * parseFloat(showtime.value.price);
});

// Create booking
const createBooking = async () => {
  if (selectedSeats.value.length === 0) {
    alert('Pilih minimal 1 kursi');
    return;
  }

  try {
    const bookingData = {
      user_id: 1, // TODO: Get from auth
      showtime_id: showtimeId.value,
      seat_ids: selectedSeats.value
    };

    const response = await axios.post(`${API_BASE_URL}/bookings`, bookingData);
    
    if (response.data.success) {
      alert(`Booking berhasil! Kode booking: ${response.data.data.booking_code}`);
      // Redirect ke halaman konfirmasi
    }
  } catch (err) {
    console.error('Error creating booking:', err);
    alert('Booking gagal, silakan coba lagi');
  }
};

onMounted(() => {
  fetchSeats();
});
</script>

<template>
  <div class="container mx-auto p-6">
    <h1 class="text-3xl font-bold mb-6">Pilih Kursi</h1>

    <!-- Seat Layout -->
    <div class="bg-white rounded-lg shadow p-6 mb-6">
      <div class="text-center mb-8">
        <div class="bg-gray-800 text-white py-2 rounded">LAYAR</div>
      </div>

      <div v-if="loading" class="text-center">Loading...</div>
      
      <div v-else class="grid grid-cols-10 gap-2">
        <button
          v-for="seat in seats"
          :key="seat.id"
          @click="toggleSeat(seat.id)"
          :class="[
            'p-3 rounded text-sm font-medium transition',
            selectedSeats.includes(seat.id)
              ? 'bg-blue-500 text-white'
              : 'bg-gray-200 hover:bg-gray-300'
          ]"
        >
          {{ seat.seat_code }}
        </button>
      </div>
    </div>

    <!-- Summary -->
    <div class="bg-white rounded-lg shadow p-6">
      <h2 class="text-xl font-bold mb-4">Ringkasan Booking</h2>
      <div class="space-y-2 mb-4">
        <p>Kursi dipilih: {{ selectedSeats.length }}</p>
        <p class="text-2xl font-bold text-blue-600">
          Total: Rp {{ totalPrice.toLocaleString('id-ID') }}
        </p>
      </div>
      
      <button
        @click="createBooking"
        :disabled="selectedSeats.length === 0"
        class="w-full bg-blue-600 text-white py-3 rounded-lg font-semibold hover:bg-blue-700 disabled:bg-gray-300 disabled:cursor-not-allowed"
      >
        Konfirmasi Booking
      </button>
    </div>
  </div>
</template>