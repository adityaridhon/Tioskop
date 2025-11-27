<script setup>
import { ref, onMounted } from 'vue';
import axios from 'axios';

const API_BASE_URL = 'http://127.0.0.1:3000/api';
const movies = ref([]);
const loading = ref(false);
const error = ref(null);

// Fetch movies dari backend
const fetchMovies = async () => {
  loading.value = true;
  error.value = null;
  try {
    const response = await axios.get(`${API_BASE_URL}/movies/all`);
    if (response.data.success) {
      movies.value = response.data.data;
    }
  } catch (err) {
    error.value = 'Gagal memuat data film';
    console.error(err);
  } finally {
    loading.value = false;
  }
};

// Create movie
const createMovie = async (movieData) => {
  try {
    const response = await axios.post(`${API_BASE_URL}/movies`, movieData);
    if (response.data.success) {
      await fetchMovies(); // Refresh list
      return response.data;
    }
  } catch (err) {
    console.error('Error creating movie:', err);
    throw err;
  }
};

// Update movie
const updateMovie = async (id, movieData) => {
  try {
    const response = await axios.put(`${API_BASE_URL}/movies/${id}`, movieData);
    if (response.data.success) {
      await fetchMovies();
      return response.data;
    }
  } catch (err) {
    console.error('Error updating movie:', err);
    throw err;
  }
};

// Delete movie
const deleteMovie = async (id) => {
  try {
    const response = await axios.delete(`${API_BASE_URL}/movies/${id}`);
    if (response.data.success) {
      await fetchMovies();
      return response.data;
    }
  } catch (err) {
    console.error('Error deleting movie:', err);
    throw err;
  }
};

onMounted(() => {
  fetchMovies();
});
</script>