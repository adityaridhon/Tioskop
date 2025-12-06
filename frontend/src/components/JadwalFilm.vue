<template>
    <div class="min-h-screen bg-[#0a1628]">
        <!-- Navbar -->
        <nav class="bg-[#1e2a3a] border-b border-gray-700">
            <div class="max-w-7xl mx-auto px-6">
                <div class="flex items-center justify-between h-14">
                    <!-- Left side - Logo with blue background -->
                    <div class="flex items-center">
                        <div class="bg-blue-600 px-4 py-1.5 rounded-lg">
                            <span class="text-white text-sm font-bold">TIOSKOP</span>
                        </div>
                    </div>
                    
                    <!-- Right side - Menu Items -->
                    <div class="flex items-center gap-6">
                        <a href="/" class="flex items-center gap-2 text-gray-300 hover:text-white transition-colors">
                            <Film class="w-4 h-4" />
                            <span class="text-sm">Cari Film</span>
                        </a>
                        <a href="/jadwal" class="flex items-center gap-2 text-gray-300 hover:text-white transition-colors">
                            <Calendar class="w-4 h-4" />
                            <span class="text-sm">Jadwal Showing</span>
                        </a>
                        <a href="/cinemas" class="flex items-center gap-2 text-gray-300 hover:text-white transition-colors">
                            <MapPin class="w-4 h-4" />
                            <span class="text-sm">Bioskop</span>
                        </a>
                        <a href="#" class="flex items-center gap-2 text-gray-300 hover:text-white transition-colors">
                            <MapPin class="w-4 h-4" />
                            <span class="text-sm">Balikpapan</span>
                        </a>
                    </div>
                </div>
            </div>
        </nav>

        <!-- Main Content -->
        <div class="max-w-7xl mx-auto px-6 py-8">
            <!-- Movie Info Section -->
            <div class="bg-[#1a2942] rounded-2xl p-6 mb-6">
                <div v-if="isLoading" class="text-center py-8">
                    <div class="inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-blue-500"></div>
                    <p class="mt-4 text-gray-400">Memuat data film...</p>
                </div>

                <div v-else-if="error" class="text-center py-8">
                    <p class="text-red-400">{{ error }}</p>
                </div>

                <div v-else class="flex gap-6">
                    <!-- Movie Poster -->
                    <div class="flex-shrink-0">
                        <div class="w-32 h-48 rounded-xl overflow-hidden shadow-lg">
                            <img
                                :src="selectedMovie.poster_url || '/placeholder.jpg'"
                                :alt="selectedMovie.title"
                                class="w-full h-full object-cover"
                                @error="(e: Event) => { const target = e.target as HTMLImageElement; if (target) target.src = '/placeholder.jpg' }"
                            />
                        </div>
                    </div>

                    <!-- Movie Details -->
                    <div class="flex-1">
                        <h1 class="text-white text-2xl font-bold mb-2">{{ selectedMovie.title }}</h1>
                        <div class="flex items-center gap-4 mb-3 text-sm text-gray-300">
                            <span>{{ selectedMovie.duration || 105 }} menit</span>
                            <span>•</span>
                            <span>{{ selectedMovie.genre || 'Comedy, Adventure' }}</span>
                            <span>•</span>
                            <span class="px-2 py-0.5 bg-blue-600 text-white rounded text-xs font-semibold">
                                {{ selectedMovie.rating || '13+' }}
                            </span>
                        </div>
                    </div>
                </div>
            </div>

            <!-- Date Selection -->
            <div class="mb-6">
                <h3 class="text-white text-lg font-semibold mb-3">Pilih Tanggal</h3>
                <div class="flex gap-3 overflow-x-auto pb-2">
                    <button
                        v-for="dateItem in dates"
                        :key="dateItem.full"
                        @click="selectedDate = dateItem.full"
                        class="flex-shrink-0 px-5 py-2.5 rounded-lg transition-all duration-200 min-w-[140px]"
                        :class="
                            selectedDate === dateItem.full
                                ? 'bg-blue-600 text-white shadow-lg'
                                : 'bg-[#1a2942] text-gray-300 hover:bg-[#243a5e]'
                        "
                    >
                        <div class="flex items-center gap-2">
                            <Calendar class="w-4 h-4" />
                            <span class="font-medium">{{ dateItem.formatted }}</span>
                        </div>
                    </button>
                </div>
            </div>

            <!-- Cinema Selection -->
            <div class="mb-6">
                <h3 class="text-white text-lg font-semibold mb-3">Pilih Bioskop</h3>
                <div class="flex gap-3 overflow-x-auto pb-2">
                    <button
                        v-for="cinema in cinemas"
                        :key="cinema.id"
                        @click="selectedCinema = cinema.id"
                        class="flex-shrink-0 px-6 py-3 rounded-lg transition-all duration-200 whitespace-nowrap"
                        :class="
                            selectedCinema === cinema.id
                                ? 'bg-blue-600 text-white shadow-lg'
                                : 'bg-[#1a2942] text-gray-300 hover:bg-[#243a5e]'
                        "
                    >
                        <div class="flex items-center gap-2">
                            <MapPin class="w-4 h-4" />
                            <span>{{ cinema.name }}</span>
                        </div>
                    </button>
                </div>
            </div>

            <!-- Showtimes Grid - New Design -->
            <div v-if="selectedDate !== null && selectedCinema !== null">
                <!-- Header with Jadwal Terdekat Button -->
                <div class="flex items-center justify-between mb-4">
                    <h3 class="text-white text-xl font-bold">Jadwal Tayang</h3>
                    <button 
                        @click="() => findNearestShowtime(true)"
                        class="flex items-center gap-2 px-6 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-all shadow-md"
                    >
                        <Clock class="w-5 h-5" />
                        <span>Jadwal Terdekat</span>
                    </button>
                </div>
                
                <!-- Showtimes Grid -->
                <div v-if="allShowtimes.length > 0" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                    <div
                        v-for="showtime in allShowtimes"
                        :key="showtime.id"
                        class="bg-[#1a2942] rounded-xl p-5 border border-gray-700 hover:border-blue-500 transition-all"
                    >
                        <!-- Time and Price Header -->
                        <div class="flex items-start justify-between mb-4">
                            <div class="bg-blue-600 text-white px-4 py-2 rounded-lg font-bold text-lg">
                                {{ showtime.time }}
                            </div>
                            <div class="text-right">
                                <div class="text-gray-400 text-xs">Harga</div>
                                <div class="text-white font-bold">Rp {{ showtime.price.toLocaleString('id-ID') }}</div>
                            </div>
                        </div>

                        <!-- Date -->
                        <div class="text-gray-400 text-sm mb-3">
                            {{ showtime.dateFormatted }}
                        </div>

                        <!-- Cinema Info -->
                        <div class="space-y-2 mb-4">
                            <div class="flex items-center gap-2 text-white">
                                <MapPin class="w-4 h-4 text-blue-400" />
                                <span class="font-medium">{{ showtime.cinemaName }}</span>
                            </div>
                            <div class="text-gray-400 text-sm pl-6">
                                {{ showtime.location }}
                            </div>
                        </div>

                        <!-- Available Seats -->
                        <div class="flex items-center gap-2 text-gray-300 text-sm mb-2">
                            <svg class="w-4 h-4 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
                            </svg>
                            <span>{{ showtime.availableSeats }} kursi tersedia</span>
                        </div>

                        <!-- Time Remaining -->
                        <div class="flex items-center gap-2 text-gray-400 text-xs mb-4">
                            <Clock class="w-4 h-4" />
                            <span>{{ showtime.timeRemaining }}</span>
                        </div>

                        <!-- Action Button -->
                        <button 
                            @click="() => goToBooking(showtime.id)"
                            class="w-full bg-blue-600 hover:bg-blue-700 text-white font-semibold py-2.5 px-4 rounded-lg transition-all"
                        >
                            Pilih Jadwal Ini
                        </button>
                    </div>
                </div>

                <div v-else class="text-gray-500 text-center py-16">
                    <Film class="w-16 h-16 text-gray-600 mx-auto mb-4" />
                    <p class="text-lg">Tidak ada jadwal tersedia untuk tanggal ini</p>
                </div>
            </div>

            <!-- Empty State -->
            <div v-else class="text-center py-16">
                <Film class="w-16 h-16 text-gray-600 mx-auto mb-4" />
                <p class="text-gray-400 text-lg">Pilih tanggal dan bioskop untuk melihat jadwal tayang</p>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { X, MapPin, Clock, Film, Calendar } from 'lucide-vue-next';

const route = useRoute();
const router = useRouter();

const goBack = () => {
    router.back();
};

const goToBooking = (showtimeId: number) => {
    console.log('goToBooking called with showtimeId:', showtimeId);
    
    // Find the showtime to get its details
    const showtime = showtimeList.value.find(st => st.id === showtimeId);
    console.log('Found showtime:', showtime);
    
    if (!showtime) {
        console.error('Showtime not found for ID:', showtimeId);
        alert('Jadwal tidak ditemukan');
        return;
    }
    
    // Navigate to booking page with movieId and showtimeId
    const targetPath = `/booking/${selectedMovie.value.id}/${showtimeId}`;
    
    console.log('Navigating to:', targetPath);
    
    router.push(targetPath);
};

// Movie data
interface Movie {
    id: number;
    title: string;
    genre: string | null;
    rating: string | null;
    poster_url: string | null;
    description: string | null;
    duration: number | null;
}

const selectedMovie = ref<Movie>({
    id: 0,
    title: 'Loading...',
    genre: '',
    rating: '0',
    poster_url: '',
    description: '',
    duration: 0,
});

const isLoading = ref(true);
const error = ref<string | null>(null);

// Fetch movie data
const fetchMovieData = async () => {
    const movieId = route.params.movieId;

    try {
        isLoading.value = true;
        const response = await fetch(`http://127.0.0.1:3000/api/movies`);

        if (!response.ok) {
            throw new Error('Gagal mengambil data film');
        }

        const result = await response.json();
        const movies = result.data || [];

        const movie = movies.find((m: Movie) => m.id === parseInt(movieId as string));

        if (movie) {
            selectedMovie.value = movie;
        } else {
            error.value = 'Film tidak ditemukan';
        }
    } catch (err) {
        error.value = err instanceof Error ? err.message : 'Terjadi kesalahan';
        console.error('Error fetching movie:', err);
    } finally {
        isLoading.value = false;
    }
};

// Date selection
const selectedDate = ref<string | null>(null);
const selectedCinema = ref<number | null>(null);

// Generate dates (next 7 days)
const dates = computed(() => {
    const result: any[] = [];
    for (let i = 0; i < 7; i++) {
        const date = new Date();
        date.setDate(date.getDate() + i);
        
        const dayName = date.toLocaleDateString('id-ID', { weekday: 'long' });
        const day = date.getDate();
        const monthName = date.toLocaleDateString('id-ID', { month: 'short' });
        
        // Use local date format instead of ISO to avoid timezone issues
        const year = date.getFullYear();
        const month = String(date.getMonth() + 1).padStart(2, '0');
        const dayStr = String(date.getDate()).padStart(2, '0');
        const fullDate = `${year}-${month}-${dayStr}`;

        result.push({
            formatted: `${dayName}, ${day} ${monthName}`,
            full: fullDate,
        });
    }
    return result;
});

// Cinema data
interface Cinema {
    id: number;
    name: string;
    address: string;
    showtimes?: Showtime[];
}

interface Showtime {
    id: number;
    time: string;
    price: number;
    availableSeats: number;
}

const cinemas = ref<Cinema[]>([
    { id: 0, name: 'Semua Bioskop', address: 'Semua Lokasi' },
    { id: 1, name: 'Tioskop E-Walk BPN', address: 'E-Walk Balikpapan, Jl. Jenderal Sudirman' },
    { id: 2, name: 'Tioskop Mall Balikpapan', address: 'Mall Balikpapan Lantai 3, Jl. Jenderal Sudirman No. 1' },
    { id: 3, name: 'Tioskop Plaza Klandasan', address: 'Plaza Klandasan, Jl. Marsma R. Iswahyudi No. 88' },
]);

const showtimeList = ref<any[]>([]);

// Fetch showtimes from API
const fetchShowtimes = async (movieId: number) => {
    try {
        const response = await fetch(`http://127.0.0.1:3000/api/showtimes/movie/${movieId}`);
        const result = await response.json();
        if (result.success) {
            showtimeList.value = result.data;
            await enrichShowtimesWithSeats();
        }
    } catch (err) {
        console.error('Error fetching showtimes:', err);
    }
};

// Fetch seat availability for each showtime
const enrichShowtimesWithSeats = async () => {
    for (const showtime of showtimeList.value) {
        try {
            const response = await fetch(`http://127.0.0.1:3000/api/seats/showtime/${showtime.id}`);
            const result = await response.json();
            if (result.success) {
                const availableSeats = result.data.filter((seat: any) => !seat.is_booked).length;
                showtime.availableSeats = availableSeats;
            }
        } catch (err) {
            console.error(`Error fetching seats for showtime ${showtime.id}:`, err);
            showtime.availableSeats = 0;
        }
    }
};

// Filter cinemas based on selection
const filteredCinemas = computed(() => {
    if (!selectedDate.value || !selectedCinema.value) return [];

    // Filter showtimes by date
    const filteredShowtimes = showtimeList.value.filter((st) => {
        const dateObj = new Date(st.start_time);
        const fullDate = dateObj.toISOString().split('T')[0];
        return fullDate === selectedDate.value;
    });

    // Group by studio/cinema
    const cinemaMap = new Map<number, Cinema>();

    filteredShowtimes.forEach((st) => {
        const cinemaId = st.studio_id || 2; // Default to cinema 2 if not specified
        
        if (!cinemaMap.has(cinemaId)) {
            const cinema = cinemas.value.find(c => c.id === cinemaId) || {
                id: cinemaId,
                name: `Cinema ${cinemaId}`,
                address: 'Alamat tidak tersedia'
            };
            cinemaMap.set(cinemaId, { ...cinema, showtimes: [] });
        }

        const cinema = cinemaMap.get(cinemaId)!;
        const dateObj = new Date(st.start_time);
        const timeStr = dateObj.toLocaleTimeString('id-ID', {
            hour: '2-digit',
            minute: '2-digit',
        }).replace('.', ':');

        cinema.showtimes!.push({
            id: st.id,
            time: timeStr,
            price: 45000, // Default price, can be from API
            availableSeats: st.availableSeats || 0,
        });
    });

    // If "Semua Bioskop" is selected, show all
    if (selectedCinema.value === 0) {
        return Array.from(cinemaMap.values());
    }

    // Otherwise, filter by selected cinema
    const selected = cinemaMap.get(selectedCinema.value);
    return selected ? [selected] : [];
});

// All showtimes with detailed information for new card design
const allShowtimes = computed(() => {
    if (!selectedDate.value || selectedCinema.value === null) return [];

    // Filter showtimes by date
    const filteredShowtimes = showtimeList.value.filter((st) => {
        const dateObj = new Date(st.start_time);
        const fullDate = dateObj.toISOString().split('T')[0];
        return fullDate === selectedDate.value;
    });

    // Filter by cinema if not "Semua Bioskop"
    const cinemFilteredShowtimes = selectedCinema.value === 0 
        ? filteredShowtimes 
        : filteredShowtimes.filter(st => st.studio_id === selectedCinema.value);

    // Map to detailed showtime objects
    return cinemFilteredShowtimes.map((st) => {
        const dateObj = new Date(st.start_time);
        const timeStr = dateObj.toLocaleTimeString('id-ID', {
            hour: '2-digit',
            minute: '2-digit',
        }).replace('.', ':');

        // Format date
        const dateFormatted = dateObj.toLocaleDateString('id-ID', {
            year: 'numeric',
            month: '2-digit',
            day: '2-digit',
        });

        // Calculate time remaining
        const now = new Date();
        const diff = dateObj.getTime() - now.getTime();
        let timeRemaining = '';
        
        if (diff > 0) {
            const hours = Math.floor(diff / (1000 * 60 * 60));
            const minutes = Math.floor((diff % (1000 * 60 * 60)) / (1000 * 60));
            timeRemaining = `${hours} jam ${minutes} menit lagi`;
        } else {
            timeRemaining = 'Sudah berlalu';
        }

        // Get cinema info
        const cinema = cinemas.value.find(c => c.id === st.studio_id) || {
            name: 'Tioskop E-Walk BPN',
            address: 'E-Walk Balikpapan, Jl. Jenderal Sudirman'
        };

        return {
            id: st.id,
            time: timeStr,
            price: 45000,
            dateFormatted: dateFormatted,
            cinemaName: cinema.name,
            location: cinema.address.includes(',') ? cinema.address.split(',')[0] : 'Balikpapan',
            availableSeats: st.availableSeats || 0,
            timeRemaining: timeRemaining,
        };
    });
});

// Find nearest showtime function
const findNearestShowtime = (showAlert: boolean = true) => {
    if (showtimeList.value.length === 0) {
        if (showAlert) alert('Tidak ada jadwal tersedia');
        return;
    }

    const now = new Date();
    let nearestShowtime: any = null;
    let minDiff = Infinity;

    // Find the showtime closest to current time
    showtimeList.value.forEach((st) => {
        // Parse the datetime string properly
        const showtimeDate = new Date(st.start_time);
        const diff = showtimeDate.getTime() - now.getTime();
        
        // Only consider future showtimes
        if (diff > 0 && diff < minDiff) {
            minDiff = diff;
            nearestShowtime = st;
        }
    });

    if (nearestShowtime) {
        // Auto-select the date and cinema for nearest showtime
        const showtimeDate = new Date(nearestShowtime.start_time);
        const dateStr = showtimeDate.toISOString().split('T')[0];
        
        selectedDate.value = dateStr;
        // Always select "Semua Bioskop" to show all showtimes for this date
        selectedCinema.value = 0;

        // Scroll to showtimes section
        setTimeout(() => {
            const showtimesSection = document.querySelector('.bg-\\[\\#1a2942\\]');
            if (showtimesSection) {
                showtimesSection.scrollIntoView({ behavior: 'smooth', block: 'start' });
            }
        }, 100);

        if (showAlert) {
            // Show notification
            const timeStr = showtimeDate.toLocaleTimeString('id-ID', {
                hour: '2-digit',
                minute: '2-digit',
            }).replace('.', ':');
            const dateFormatted = showtimeDate.toLocaleDateString('id-ID', {
                weekday: 'long',
                day: 'numeric',
                month: 'long',
            });
            
            alert(`Jadwal terdekat ditemukan!\n${dateFormatted} pukul ${timeStr}`);
        }
    } else {
        if (showAlert) alert('Tidak ada jadwal yang akan datang');
    }
};

onMounted(async () => {
    await fetchMovieData();
    await fetchShowtimes(parseInt(route.params.movieId as string));
    
    // Wait a bit for seat data to be enriched
    setTimeout(() => {
        // Automatically find and select nearest showtime
        if (showtimeList.value.length > 0) {
            findNearestShowtime(false); // Silent mode - no alerts
        } else {
            // Fallback: Auto-select first date and "Semua Bioskop"
            if (dates.value.length > 0) {
                selectedDate.value = dates.value[0].full;
            }
            selectedCinema.value = 0;
        }
    }, 500);
});
</script>

<style scoped>
/* Custom scrollbar for horizontal scroll */
::-webkit-scrollbar {
    height: 6px;
}

::-webkit-scrollbar-track {
    background: #1a2942;
    border-radius: 10px;
}

::-webkit-scrollbar-thumb {
    background: #3b5998;
    border-radius: 10px;
}

::-webkit-scrollbar-thumb:hover {
    background: #4a6bb8;
}
</style>
