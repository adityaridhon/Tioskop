// Contoh penggunaan API service di komponen Vue

import { ref, onMounted } from 'vue';
import { moviesAPI } from '@/services/api';

export default {
  setup() {
    const movies = ref([]);
    const loading = ref(false);
    const error = ref(null);

    // Fetch all movies saat komponen di-mount
    const fetchMovies = async () => {
      loading.value = true;
      error.value = null;
      
      try {
        const response = await moviesAPI.getAll();
        
        if (response.success) {
          movies.value = response.data;
        }
      } catch (err) {
        error.value = err.message;
        console.error('Error fetching movies:', err);
      } finally {
        loading.value = false;
      }
    };

    // Search movies
    const searchMovies = async (query) => {
      if (!query.trim()) {
        fetchMovies();
        return;
      }

      loading.value = true;
      error.value = null;
      
      try {
        const response = await moviesAPI.search(query);
        
        if (response.success) {
          movies.value = response.data;
        }
      } catch (err) {
        error.value = err.message;
        console.error('Error searching movies:', err);
      } finally {
        loading.value = false;
      }
    };

    // Delete movie
    const deleteMovie = async (id) => {
      if (!confirm('Apakah Anda yakin ingin menghapus film ini?')) {
        return;
      }

      try {
        const response = await moviesAPI.delete(id);
        
        if (response.success) {
          // Refresh list setelah delete
          await fetchMovies();
          alert('Film berhasil dihapus');
        }
      } catch (err) {
        error.value = err.message;
        console.error('Error deleting movie:', err);
        alert('Gagal menghapus film');
      }
    };

    // Call fetchMovies saat komponen di-mount
    onMounted(() => {
      fetchMovies();
    });

    return {
      movies,
      loading,
      error,
      fetchMovies,
      searchMovies,
      deleteMovie,
    };
  }
};
