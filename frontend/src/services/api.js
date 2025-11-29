// Base URL untuk API backend
const API_BASE_URL = 'http://127.0.0.1:3000/api';

// Helper function untuk handle response
const handleResponse = async (response) => {
  try {
    // Check if response is JSON
    const contentType = response.headers.get('content-type');
    if (!contentType || !contentType.includes('application/json')) {
      const text = await response.text();
      console.error('Non-JSON response:', text);

      // Check if it's a common error message
      if (text.includes('Invalid URL') || text.includes('Cannot') || text.includes('Error')) {
        throw new Error(`Server error: ${text.substring(0, 100)}`);
      }

      throw new Error(`Server mengembalikan response bukan JSON. Status: ${response.status} ${response.statusText}`);
    }

    const data = await response.json();

    if (!response.ok) {
      throw new Error(data.message || 'Terjadi kesalahan pada server');
    }

    return data;
  } catch (err) {
    // If it's already an Error we threw, rethrow it
    if (err instanceof Error && err.message.includes('Server')) {
      throw err;
    }

    // Check if it's a network error
    if (err.name === 'TypeError' && err.message.includes('fetch')) {
      throw new Error('Tidak dapat terhubung ke server. Pastikan backend sedang berjalan di http://127.0.0.1:3000');
    }

    // Otherwise it's a JSON parsing error or other error
    console.error('Error parsing response:', err);
    throw new Error('Format response dari server tidak valid. Pastikan backend sedang berjalan dengan benar.');
  }
};

// Movies API
export const moviesAPI = {
  // Get all movies
  getAll: async () => {
    try {
      const response = await fetch(`${API_BASE_URL}/movies/all`);
      return handleResponse(response);
    } catch (err) {
      if (err.message.includes('fetch')) {
        throw new Error('Tidak dapat terhubung ke server. Pastikan backend sedang berjalan.');
      }
      throw err;
    }
  },

  // Search movies
  search: async (query) => {
    try {
      const response = await fetch(`${API_BASE_URL}/movies?q=${encodeURIComponent(query)}`);
      return handleResponse(response);
    } catch (err) {
      if (err.message.includes('fetch')) {
        throw new Error('Tidak dapat terhubung ke server. Pastikan backend sedang berjalan.');
      }
      throw err;
    }
  },

  // Get movie by ID
  getById: async (id) => {
    try {
      const response = await fetch(`${API_BASE_URL}/movies/${id}`);
      return handleResponse(response);
    } catch (err) {
      if (err.message.includes('fetch')) {
        throw new Error('Tidak dapat terhubung ke server. Pastikan backend sedang berjalan.');
      }
      throw err;
    }
  },

  // Create new movie
  create: async (movieData) => {
    try {
      const response = await fetch(`${API_BASE_URL}/movies`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(movieData),
      });
      return handleResponse(response);
    } catch (err) {
      if (err.message.includes('fetch')) {
        throw new Error('Tidak dapat terhubung ke server. Pastikan backend sedang berjalan.');
      }
      throw err;
    }
  },

  // Update movie
  update: async (id, movieData) => {
    try {
      const response = await fetch(`${API_BASE_URL}/movies/${id}`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(movieData),
      });
      return handleResponse(response);
    } catch (err) {
      if (err.message.includes('fetch')) {
        throw new Error('Tidak dapat terhubung ke server. Pastikan backend sedang berjalan.');
      }
      throw err;
    }
  },

  // Delete movie
  delete: async (id) => {
    try {
      const response = await fetch(`${API_BASE_URL}/movies/${id}`, {
        method: 'DELETE',
      });
      return handleResponse(response);
    } catch (err) {
      if (err.message.includes('fetch')) {
        throw new Error('Tidak dapat terhubung ke server. Pastikan backend sedang berjalan.');
      }
      throw err;
    }
  },
};

// Showtimes API
export const showtimesAPI = {
  // Get all showtimes
  getAll: async () => {
    try {
      const response = await fetch(`${API_BASE_URL}/showtimes`);
      return handleResponse(response);
    } catch (err) {
      if (err.message.includes('fetch')) {
        throw new Error('Tidak dapat terhubung ke server. Pastikan backend sedang berjalan.');
      }
      throw err;
    }
  },

  // Get showtimes by date
  getByDate: async (date) => {
    try {
      const response = await fetch(`${API_BASE_URL}/showtimes?date=${date}`);
      return handleResponse(response);
    } catch (err) {
      if (err.message.includes('fetch')) {
        throw new Error('Tidak dapat terhubung ke server. Pastikan backend sedang berjalan.');
      }
      throw err;
    }
  },

  // Get showtime by ID
  getById: async (id) => {
    try {
      const response = await fetch(`${API_BASE_URL}/showtimes/${id}`);
      return handleResponse(response);
    } catch (err) {
      if (err.message.includes('fetch')) {
        throw new Error('Tidak dapat terhubung ke server. Pastikan backend sedang berjalan.');
      }
      throw err;
    }
  },

  // Create new showtime
  create: async (showtimeData) => {
    try {
      const response = await fetch(`${API_BASE_URL}/showtimes`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(showtimeData),
      });
      return handleResponse(response);
    } catch (err) {
      if (err.message.includes('fetch')) {
        throw new Error('Tidak dapat terhubung ke server. Pastikan backend sedang berjalan.');
      }
      throw err;
    }
  },

  // Update showtime
  update: async (id, showtimeData) => {
    try {
      const response = await fetch(`${API_BASE_URL}/showtimes/${id}`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(showtimeData),
      });
      return handleResponse(response);
    } catch (err) {
      if (err.message.includes('fetch')) {
        throw new Error('Tidak dapat terhubung ke server. Pastikan backend sedang berjalan.');
      }
      throw err;
    }
  },

  // Delete showtime
  delete: async (id) => {
    try {
      const response = await fetch(`${API_BASE_URL}/showtimes/${id}`, {
        method: 'DELETE',
      });
      return handleResponse(response);
    } catch (err) {
      if (err.message.includes('fetch')) {
        throw new Error('Tidak dapat terhubung ke server. Pastikan backend sedang berjalan.');
      }
      throw err;
    }
  },
};

// Studios API
export const studiosAPI = {
  // Get all studios
  getAll: async () => {
    try {
      const response = await fetch(`${API_BASE_URL}/studios`);
      return handleResponse(response);
    } catch (err) {
      if (err.message.includes('fetch')) {
        throw new Error('Tidak dapat terhubung ke server. Pastikan backend sedang berjalan.');
      }
      throw err;
    }
  },

  // Get studio by ID
  getById: async (id) => {
    try {
      const response = await fetch(`${API_BASE_URL}/studios/${id}`);
      return handleResponse(response);
    } catch (err) {
      if (err.message.includes('fetch')) {
        throw new Error('Tidak dapat terhubung ke server. Pastikan backend sedang berjalan.');
      }
      throw err;
    }
  },

  // Create new studio
  create: async (studioData) => {
    try {
      const response = await fetch(`${API_BASE_URL}/studios`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(studioData),
      });
      return handleResponse(response);
    } catch (err) {
      if (err.message.includes('fetch')) {
        throw new Error('Tidak dapat terhubung ke server. Pastikan backend sedang berjalan.');
      }
      throw err;
    }
  },

  // Update studio
  update: async (id, studioData) => {
    try {
      const response = await fetch(`${API_BASE_URL}/studios/${id}`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(studioData),
      });
      return handleResponse(response);
    } catch (err) {
      if (err.message.includes('fetch')) {
        throw new Error('Tidak dapat terhubung ke server. Pastikan backend sedang berjalan.');
      }
      throw err;
    }
  },

  // Delete studio
  delete: async (id) => {
    try {
      const response = await fetch(`${API_BASE_URL}/studios/${id}`, {
        method: 'DELETE',
      });
      return handleResponse(response);
    } catch (err) {
      if (err.message.includes('fetch')) {
        throw new Error('Tidak dapat terhubung ke server. Pastikan backend sedang berjalan.');
      }
      throw err;
    }
  },
};

// Bookings API
export const bookingsAPI = {
  // Get all bookings
  getAll: async () => {
    try {
      const response = await fetch(`${API_BASE_URL}/bookings`);
      return handleResponse(response);
    } catch (err) {
      if (err.message.includes('fetch')) {
        throw new Error('Tidak dapat terhubung ke server. Pastikan backend sedang berjalan.');
      }
      throw err;
    }
  },

  // Search bookings by customer name
  search: async (customerName) => {
    try {
      const response = await fetch(`${API_BASE_URL}/bookings?customer=${encodeURIComponent(customerName)}`);
      return handleResponse(response);
    } catch (err) {
      if (err.message.includes('fetch')) {
        throw new Error('Tidak dapat terhubung ke server. Pastikan backend sedang berjalan.');
      }
      throw err;
    }
  },

  // Get booking by ID
  getById: async (id) => {
    try {
      const response = await fetch(`${API_BASE_URL}/bookings/${id}`);
      return handleResponse(response);
    } catch (err) {
      if (err.message.includes('fetch')) {
        throw new Error('Tidak dapat terhubung ke server. Pastikan backend sedang berjalan.');
      }
      throw err;
    }
  },

  // Create new booking
  create: async (bookingData) => {
    try {
      const response = await fetch(`${API_BASE_URL}/bookings`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(bookingData),
      });
      return handleResponse(response);
    } catch (err) {
      if (err.message.includes('fetch')) {
        throw new Error('Tidak dapat terhubung ke server. Pastikan backend sedang berjalan.');
      }
      throw err;
    }
  },

  // Update booking status
  updateStatus: async (id, status) => {
    try {
      const response = await fetch(`${API_BASE_URL}/bookings/${id}/status`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ status }),
      });
      return handleResponse(response);
    } catch (err) {
      if (err.message.includes('fetch')) {
        throw new Error('Tidak dapat terhubung ke server. Pastikan backend sedang berjalan.');
      }
      throw err;
    }
  },

  // Delete booking
  delete: async (id) => {
    try {
      const response = await fetch(`${API_BASE_URL}/bookings/${id}`, {
        method: 'DELETE',
      });
      return handleResponse(response);
    } catch (err) {
      if (err.message.includes('fetch')) {
        throw new Error('Tidak dapat terhubung ke server. Pastikan backend sedang berjalan.');
      }
      throw err;
    }
  },
};

// Seats API
export const seatsAPI = {
  // Get seats by showtime
  getByShowtime: async (showtimeId) => {
    try {
      const response = await fetch(`${API_BASE_URL}/seats/showtime/${showtimeId}`);
      return handleResponse(response);
    } catch (err) {
      if (err.message.includes('fetch')) {
        throw new Error('Tidak dapat terhubung ke server. Pastikan backend sedang berjalan.');
      }
      throw err;
    }
  },

  // Get available seats
  getAvailable: async (showtimeId) => {
    try {
      const response = await fetch(`${API_BASE_URL}/seats/available/${showtimeId}`);
      return handleResponse(response);
    } catch (err) {
      if (err.message.includes('fetch')) {
        throw new Error('Tidak dapat terhubung ke server. Pastikan backend sedang berjalan.');
      }
      throw err;
    }
  },

  // Book seats
  book: async (seatIds) => {
    try {
      const response = await fetch(`${API_BASE_URL}/seats/book`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ seat_ids: seatIds }),
      });
      return handleResponse(response);
    } catch (err) {
      if (err.message.includes('fetch')) {
        throw new Error('Tidak dapat terhubung ke server. Pastikan backend sedang berjalan.');
      }
      throw err;
    }
  },

  // Release seats
  release: async (seatIds) => {
    try {
      const response = await fetch(`${API_BASE_URL}/seats/release`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ seat_ids: seatIds }),
      });
      return handleResponse(response);
    } catch (err) {
      if (err.message.includes('fetch')) {
        throw new Error('Tidak dapat terhubung ke server. Pastikan backend sedang berjalan.');
      }
      throw err;
    }
  },
};
