import { ref } from 'vue';
import { moviesAPI, showtimesAPI, studiosAPI, bookingsAPI, seatsAPI, usersAPI } from '@/services/api';

/**
 * Composable untuk Movies
 */
export function useMovies() {
  const movies = ref([]);
  const movie = ref(null);
  const loading = ref(false);
  const error = ref(null);

  const fetchAll = async () => {
    loading.value = true;
    error.value = null;
    try {
      const response = await moviesAPI.getAll();
      if (response.success) {
        movies.value = response.data;
      }
      return response;
    } catch (err) {
      error.value = err.message;
      throw err;
    } finally {
      loading.value = false;
    }
  };

  const search = async (query) => {
    loading.value = true;
    error.value = null;
    try {
      const response = await moviesAPI.search(query);
      if (response.success) {
        movies.value = response.data;
      }
      return response;
    } catch (err) {
      error.value = err.message;
      throw err;
    } finally {
      loading.value = false;
    }
  };

  const getById = async (id) => {
    loading.value = true;
    error.value = null;
    try {
      const response = await moviesAPI.getById(id);
      if (response.success) {
        movie.value = response.data;
      }
      return response;
    } catch (err) {
      error.value = err.message;
      throw err;
    } finally {
      loading.value = false;
    }
  };

  const create = async (movieData) => {
    loading.value = true;
    error.value = null;
    try {
      const response = await moviesAPI.create(movieData);
      return response;
    } catch (err) {
      error.value = err.message;
      throw err;
    } finally {
      loading.value = false;
    }
  };

  const update = async (id, movieData) => {
    loading.value = true;
    error.value = null;
    try {
      const response = await moviesAPI.update(id, movieData);
      return response;
    } catch (err) {
      error.value = err.message;
      throw err;
    } finally {
      loading.value = false;
    }
  };

  const remove = async (id) => {
    loading.value = true;
    error.value = null;
    try {
      const response = await moviesAPI.delete(id);
      return response;
    } catch (err) {
      error.value = err.message;
      throw err;
    } finally {
      loading.value = false;
    }
  };

  return {
    movies,
    movie,
    loading,
    error,
    fetchAll,
    search,
    getById,
    create,
    update,
    remove,
  };
}

/**
 * Composable untuk Showtimes
 */
export function useShowtimes() {
  const showtimes = ref([]);
  const showtime = ref(null);
  const loading = ref(false);
  const error = ref(null);

  const fetchAll = async () => {
    loading.value = true;
    error.value = null;
    try {
      const response = await showtimesAPI.getAll();
      if (response.success) {
        showtimes.value = response.data;
      }
      return response;
    } catch (err) {
      error.value = err.message;
      throw err;
    } finally {
      loading.value = false;
    }
  };

  const getByDate = async (date) => {
    loading.value = true;
    error.value = null;
    try {
      const response = await showtimesAPI.getByDate(date);
      if (response.success) {
        showtimes.value = response.data;
      }
      return response;
    } catch (err) {
      error.value = err.message;
      throw err;
    } finally {
      loading.value = false;
    }
  };

  const getById = async (id) => {
    loading.value = true;
    error.value = null;
    try {
      const response = await showtimesAPI.getById(id);
      if (response.success) {
        showtime.value = response.data;
      }
      return response;
    } catch (err) {
      error.value = err.message;
      throw err;
    } finally {
      loading.value = false;
    }
  };

  const create = async (showtimeData) => {
    loading.value = true;
    error.value = null;
    try {
      const response = await showtimesAPI.create(showtimeData);
      return response;
    } catch (err) {
      error.value = err.message;
      throw err;
    } finally {
      loading.value = false;
    }
  };

  const update = async (id, showtimeData) => {
    loading.value = true;
    error.value = null;
    try {
      const response = await showtimesAPI.update(id, showtimeData);
      return response;
    } catch (err) {
      error.value = err.message;
      throw err;
    } finally {
      loading.value = false;
    }
  };

  const remove = async (id) => {
    loading.value = true;
    error.value = null;
    try {
      const response = await showtimesAPI.delete(id);
      return response;
    } catch (err) {
      error.value = err.message;
      throw err;
    } finally {
      loading.value = false;
    }
  };

  return {
    showtimes,
    showtime,
    loading,
    error,
    fetchAll,
    getByDate,
    getById,
    create,
    update,
    remove,
  };
}

/**
 * Composable untuk Studios
 */
export function useStudios() {
  const studios = ref([]);
  const studio = ref(null);
  const loading = ref(false);
  const error = ref(null);

  const fetchAll = async () => {
    loading.value = true;
    error.value = null;
    try {
      const response = await studiosAPI.getAll();
      if (response.success) {
        studios.value = response.data;
      }
      return response;
    } catch (err) {
      error.value = err.message;
      throw err;
    } finally {
      loading.value = false;
    }
  };

  const getById = async (id) => {
    loading.value = true;
    error.value = null;
    try {
      const response = await studiosAPI.getById(id);
      if (response.success) {
        studio.value = response.data;
      }
      return response;
    } catch (err) {
      error.value = err.message;
      throw err;
    } finally {
      loading.value = false;
    }
  };

  const create = async (studioData) => {
    loading.value = true;
    error.value = null;
    try {
      const response = await studiosAPI.create(studioData);
      return response;
    } catch (err) {
      error.value = err.message;
      throw err;
    } finally {
      loading.value = false;
    }
  };

  const update = async (id, studioData) => {
    loading.value = true;
    error.value = null;
    try {
      const response = await studiosAPI.update(id, studioData);
      return response;
    } catch (err) {
      error.value = err.message;
      throw err;
    } finally {
      loading.value = false;
    }
  };

  const remove = async (id) => {
    loading.value = true;
    error.value = null;
    try {
      const response = await studiosAPI.delete(id);
      return response;
    } catch (err) {
      error.value = err.message;
      throw err;
    } finally {
      loading.value = false;
    }
  };

  return {
    studios,
    studio,
    loading,
    error,
    fetchAll,
    getById,
    create,
    update,
    remove,
  };
}

/**
 * Composable untuk Bookings
 */
export function useBookings() {
  const bookings = ref([]);
  const booking = ref(null);
  const loading = ref(false);
  const error = ref(null);

  const fetchAll = async () => {
    loading.value = true;
    error.value = null;
    try {
      const response = await bookingsAPI.getAll();
      if (response.success) {
        bookings.value = response.data;
      }
      return response;
    } catch (err) {
      error.value = err.message;
      throw err;
    } finally {
      loading.value = false;
    }
  };

  const search = async (customerName) => {
    loading.value = true;
    error.value = null;
    try {
      const response = await bookingsAPI.search(customerName);
      if (response.success) {
        bookings.value = response.data;
      }
      return response;
    } catch (err) {
      error.value = err.message;
      throw err;
    } finally {
      loading.value = false;
    }
  };

  const getById = async (id) => {
    loading.value = true;
    error.value = null;
    try {
      const response = await bookingsAPI.getById(id);
      if (response.success) {
        booking.value = response.data;
      }
      return response;
    } catch (err) {
      error.value = err.message;
      throw err;
    } finally {
      loading.value = false;
    }
  };

  const create = async (bookingData) => {
    loading.value = true;
    error.value = null;
    try {
      const response = await bookingsAPI.create(bookingData);
      return response;
    } catch (err) {
      error.value = err.message;
      throw err;
    } finally {
      loading.value = false;
    }
  };

  const updateStatus = async (id, status) => {
    loading.value = true;
    error.value = null;
    try {
      const response = await bookingsAPI.updateStatus(id, status);
      return response;
    } catch (err) {
      error.value = err.message;
      throw err;
    } finally {
      loading.value = false;
    }
  };

  const remove = async (id) => {
    loading.value = true;
    error.value = null;
    try {
      const response = await bookingsAPI.delete(id);
      return response;
    } catch (err) {
      error.value = err.message;
      throw err;
    } finally {
      loading.value = false;
    }
  };

  return {
    bookings,
    booking,
    loading,
    error,
    fetchAll,
    search,
    getById,
    create,
    updateStatus,
    remove,
  };
}

/**
 * Composable untuk Seats
 */
export function useSeats() {
  const seats = ref([]);
  const loading = ref(false);
  const error = ref(null);

  const getByShowtime = async (showtimeId) => {
    loading.value = true;
    error.value = null;
    try {
      const response = await seatsAPI.getByShowtime(showtimeId);
      if (response.success) {
        seats.value = response.data;
      }
      return response;
    } catch (err) {
      error.value = err.message;
      throw err;
    } finally {
      loading.value = false;
    }
  };

  const getAvailable = async (showtimeId) => {
    loading.value = true;
    error.value = null;
    try {
      const response = await seatsAPI.getAvailable(showtimeId);
      if (response.success) {
        seats.value = response.data;
      }
      return response;
    } catch (err) {
      error.value = err.message;
      throw err;
    } finally {
      loading.value = false;
    }
  };

  const book = async (seatIds) => {
    loading.value = true;
    error.value = null;
    try {
      const response = await seatsAPI.book(seatIds);
      return response;
    } catch (err) {
      error.value = err.message;
      throw err;
    } finally {
      loading.value = false;
    }
  };

  const release = async (seatIds) => {
    loading.value = true;
    error.value = null;
    try {
      const response = await seatsAPI.release(seatIds);
      return response;
    } catch (err) {
      error.value = err.message;
      throw err;
    } finally {
      loading.value = false;
    }
  };

  return {
    seats,
    loading,
    error,
    getByShowtime,
    getAvailable,
    book,
    release,
  };
}

/**
 * Composable untuk Users
 */
export function useUsers() {
  const users = ref([]);
  const user = ref(null);
  const loading = ref(false);
  const error = ref(null);

  const fetchAll = async () => {
    loading.value = true;
    error.value = null;
    try {
      const response = await usersAPI.getAll();
      if (response.success) {
        users.value = response.data;
      }
      return response;
    } catch (err) {
      error.value = err.message;
      throw err;
    } finally {
      loading.value = false;
    }
  };

  const login = async (loginData) => {
    loading.value = true;
    error.value = null;
    try {
      const response = await usersAPI.login(loginData);
      if (response.success) {
        user.value = response.data;
      }
      return response;
    } catch (err) {
      error.value = err.message;
      throw err;
    } finally {
      loading.value = false;
    }
  };

  const getById = async (id) => {
    loading.value = true;
    error.value = null;
    try {
      const response = await usersAPI.getById(id);
      if (response.success) {
        user.value = response.data;
      }
      return response;
    } catch (err) {
      error.value = err.message;
      throw err;
    } finally {
      loading.value = false;
    }
  };

  const create = async (userData) => {
    loading.value = true;
    error.value = null;
    try {
      const response = await usersAPI.create(userData);
      return response;
    } catch (err) {
      error.value = err.message;
      throw err;
    } finally {
      loading.value = false;
    }
  };

  const update = async (id, userData) => {
    loading.value = true;
    error.value = null;
    try {
      const response = await usersAPI.update(id, userData);
      return response;
    } catch (err) {
      error.value = err.message;
      throw err;
    } finally {
      loading.value = false;
    }
  };

  const remove = async (id) => {
    loading.value = true;
    error.value = null;
    try {
      const response = await usersAPI.delete(id);
      return response;
    } catch (err) {
      error.value = err.message;
      throw err;
    } finally {
      loading.value = false;
    }
  };

  return {
    users,
    user,
    loading,
    error,
    fetchAll,
    login,
    getById,
    create,
    update,
    remove,
  };
}
