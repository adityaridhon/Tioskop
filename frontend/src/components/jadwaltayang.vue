<script setup>
import { ref, onMounted } from 'vue';
import axios from 'axios';

const API_BASE_URL = 'http://127.0.0.1:3000/api';
const showtimes = ref([]);
const movies = ref([]);
const studios = ref([]);

// Fetch showtimes
const fetchShowtimes = async () => {
  try {
    const response = await axios.get(`${API_BASE_URL}/showtimes`);
    if (response.data.success) {
      showtimes.value = response.data.data;
    }
  } catch (err) {
    console.error('Error fetching showtimes:', err);
  }
};

// Fetch movies untuk dropdown
const fetchMovies = async () => {
  try {
    const response = await axios.get(`${API_BASE_URL}/movies/all`);
    if (response.data.success) {
      movies.value = response.data.data;
    }
  } catch (err) {
    console.error('Error fetching movies:', err);
  }
};

// Fetch studios untuk dropdown
const fetchStudios = async () => {
  try {
    const response = await axios.get(`${API_BASE_URL}/studios`);
    if (response.data.success) {
      studios.value = response.data.data;
    }
  } catch (err) {
    console.error('Error fetching studios:', err);
  }
};

// Create showtime
const createShowtime = async (showtimeData) => {
  try {
    const response = await axios.post(`${API_BASE_URL}/showtimes`, showtimeData);
    if (response.data.success) {
      await fetchShowtimes();
      return response.data;
    }
  } catch (err) {
    console.error('Error creating showtime:', err);
    throw err;
  }
};

onMounted(async () => {
  await Promise.all([fetchShowtimes(), fetchMovies(), fetchStudios()]);
});
</script>