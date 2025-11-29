<script setup>
import { ref, onMounted } from 'vue';
import { useMovies } from '@/composables/useApi';

// ===== Composables =====
const { movies, loading, error, fetchAll, create, update, remove } = useMovies();

// ===== Modal State =====
const showModal = ref(false);
const modalMode = ref('create'); // 'create' or 'edit'
const formData = ref({
  title: '',
  genre: '',
  rating: '',
  duration: '',
  description: '',
  poster_url: '',
  release_date: ''
});

// ===== Functions =====
const openCreateModal = () => {
  modalMode.value = 'create';
  formData.value = {
    title: '',
    genre: '',
    rating: '',
    duration: '',
    description: '',
    poster_url: '',
    release_date: new Date().toISOString().split('T')[0]
  };
  showModal.value = true;
};

const openEditModal = (movie) => {
  modalMode.value = 'edit';
  formData.value = {
    id: movie.id,
    title: movie.title,
    genre: movie.genre,
    rating: movie.rating,
    duration: movie.duration,
    description: movie.description,
    poster_url: movie.poster_url,
    release_date: movie.release_date
  };
  showModal.value = true;
};

const closeModal = () => {
  showModal.value = false;
  formData.value = {
    title: '',
    genre: '',
    rating: '',
    duration: '',
    description: '',
    poster_url: '',
    release_date: ''
  };
};

const handleSubmit = async () => {
  try {
    if (modalMode.value === 'create') {
      await create(formData.value);
      alert('Film berhasil ditambahkan!');
    } else {
      await update(formData.value.id, formData.value);
      alert('Film berhasil diupdate!');
    }
    closeModal();
    await fetchAll();
  } catch (err) {
    alert(`Error: ${err.message || 'Gagal menyimpan film'}`);
  }
};

const handleDelete = async (id, title) => {
  if (!confirm(`Apakah Anda yakin ingin menghapus film "${title}"?`)) {
    return;
  }

  try {
    await remove(id);
    alert('Film berhasil dihapus!');
    await fetchAll();
  } catch (err) {
    alert(`Error: ${err.message || 'Gagal menghapus film'}`);
  }
};

// ===== Lifecycle =====
onMounted(() => {
  fetchAll();
});
</script>

<template>
  <div class="p-6">
    <div class="max-w-7xl mx-auto">
      
      <!-- Header Section -->
      <div class="flex items-center justify-between mb-8">
        <div>
          <h1 class="text-3xl font-bold text-gray-900 mb-2">Manajemen Film</h1>
          <p class="text-gray-600">Kelola koleksi film bioskop Anda</p>
        </div>
        
        <!-- Add Movie Button -->
        <button 
          @click="openCreateModal"
          class="bg-gradient-to-r from-blue-600 to-blue-700 hover:from-blue-700 hover:to-blue-800 text-white font-semibold px-6 py-3 rounded-lg flex items-center gap-2 transition shadow-lg hover:shadow-xl"
        >
          <i class="bx bx-plus text-2xl"></i>
          <span>Tambah Film</span>
        </button>
      </div>

      <!-- Loading State -->
      <div v-if="loading" class="text-center py-12">
        <div class="inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
        <p class="mt-4 text-gray-600">Memuat data film...</p>
      </div>

      <!-- Error State -->
      <div v-if="error" class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
        <strong class="font-bold">Error!</strong>
        <span class="block sm:inline"> {{ error }}</span>
      </div>

      <!-- Movies Grid -->
      <div v-if="!loading && !error" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <div
          v-for="movie in movies"
          :key="movie.id"
          class="bg-white rounded-xl shadow-lg overflow-hidden hover:shadow-2xl transition-all duration-300 relative group"
        >
          <!-- Movie Poster -->
          <div class="relative overflow-hidden h-64">
            <img
              :src="movie.poster_url || 'https://via.placeholder.com/300x400?text=No+Image'"
              :alt="movie.title"
              class="w-full h-full object-cover transform transition-transform duration-500 group-hover:scale-110"
            >
            <!-- Action Buttons Overlay -->
            <div class="absolute top-3 right-3 flex gap-2 opacity-0 group-hover:opacity-100 transition-opacity">
              <button 
                @click="openEditModal(movie)"
                class="w-8 h-8 bg-blue-500 hover:bg-blue-600 text-white rounded-full flex items-center justify-center shadow-lg transition"
                title="Edit Film"
              >
                <i class="bx bx-edit text-lg"></i>
              </button>
              <button 
                @click="handleDelete(movie.id, movie.title)"
                class="w-8 h-8 bg-red-500 hover:bg-red-600 text-white rounded-full flex items-center justify-center shadow-lg transition"
                title="Hapus Film"
              >
                <i class="bx bx-trash text-lg"></i>
              </button>
            </div>
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

      <!-- Empty State -->
      <div v-if="!loading && !error && movies.length === 0" class="text-center py-12">
        <i class="bx bx-movie text-6xl text-gray-300 mb-4"></i>
        <h3 class="text-xl font-semibold text-gray-600 mb-2">Belum ada film</h3>
        <p class="text-gray-500 mb-4">Mulai tambahkan film pertama Anda</p>
        <button 
          @click="openCreateModal"
          class="bg-blue-600 hover:bg-blue-700 text-white font-medium px-6 py-2 rounded-lg inline-flex items-center gap-2"
        >
          <i class="bx bx-plus text-xl"></i>
          <span>Tambah Film</span>
        </button>
      </div>
    </div>

    <!-- Modal Form -->
    <div 
      v-if="showModal" 
      @click="closeModal"
      class="fixed inset-0 bg-black/20 backdrop-blur-sm flex items-center justify-center z-50 p-4"
    >
      <div @click.stop class="bg-white rounded-xl shadow-2xl max-w-2xl w-full max-h-[90vh] overflow-y-auto animate-slideUp">
        <!-- Modal Header -->
        <div class="sticky top-0 bg-gradient-to-r from-blue-600 to-blue-700 px-6 py-4 flex justify-between items-center rounded-t-xl">
          <h2 class="text-xl font-bold text-white flex items-center gap-2">
            <i class="bx bx-movie-play text-2xl"></i>
            {{ modalMode === 'create' ? 'Tambah Film Baru' : 'Edit Film' }}
          </h2>
          <button 
            @click="closeModal"
            class="text-white/80 hover:text-white transition-colors"
          >
            <i class="bx bx-x text-3xl"></i>
          </button>
        </div>

        <!-- Modal Body -->
        <form @submit.prevent="handleSubmit" class="p-6 space-y-4">
          <!-- Title -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">
              Judul Film <span class="text-red-500">*</span>
            </label>
            <input 
              v-model="formData.title"
              type="text" 
              required
              placeholder="Masukkan judul film"
              class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
            >
          </div>

          <!-- Genre -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">
              Genre <span class="text-red-500">*</span>
            </label>
            <input 
              v-model="formData.genre"
              type="text" 
              required
              placeholder="Contoh: Action, Drama, Comedy"
              class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
            >
          </div>

          <!-- Rating & Duration -->
          <div class="grid grid-cols-2 gap-4">
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-2">
                Rating <span class="text-red-500">*</span>
              </label>
              <input 
                v-model="formData.rating"
                type="text" 
                required
                placeholder="8.5"
                class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-2">
                Durasi (menit) <span class="text-red-500">*</span>
              </label>
              <input 
                v-model="formData.duration"
                type="number" 
                required
                placeholder="120"
                class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
            </div>
          </div>

          <!-- Description -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">
              Deskripsi <span class="text-red-500">*</span>
            </label>
            <textarea 
              v-model="formData.description"
              required
              rows="3"
              placeholder="Masukkan deskripsi film"
              class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
            ></textarea>
          </div>

          <!-- Poster URL -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">
              URL Poster <span class="text-red-500">*</span>
            </label>
            <input 
              v-model="formData.poster_url"
              type="url" 
              required
              placeholder="https://example.com/poster.jpg"
              class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
            >
          </div>

          <!-- Release Date -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">
              Tanggal Rilis <span class="text-red-500">*</span>
            </label>
            <input 
              v-model="formData.release_date"
              type="date" 
              required
              class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
            >
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
              class="flex-1 px-4 py-2 bg-blue-900 hover:bg-blue-800 text-white rounded-lg transition flex items-center justify-center gap-2"
            >
              <i class="bx bx-save"></i>
              {{ modalMode === 'create' ? 'Tambah Film' : 'Simpan Perubahan' }}
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

.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>