<template>
    <div class="min-h-screen bg-gray-50">
        <!-- Header Section -->
        <div class="bg-[#143C8C] text-white py-8 px-6 rounded-b-2xl shadow-lg">
            <div class="max-w-7xl mx-auto">
                <button
                    @click="goBack"
                    class="flex items-center gap-2 text-blue-200 hover:text-white mb-6 transition-colors group"
                >
                    <ArrowLeft
                        class="w-5 h-5 group-hover:-translate-x-1 transition-transform"
                    />
                    <span class="font-medium">Kembali</span>
                </button>

                <div class="flex items-center gap-4">
                    <div class="bg-white/10 p-3 rounded-xl">
                        <Film class="w-8 h-8" />
                    </div>
                    <div>
                        <h1 class="text-3xl font-bold mb-1">Detail Film</h1>
                        <p class="text-blue-100">
                            Pilih jadwal tayang yang sesuai
                        </p>
                    </div>
                </div>
            </div>
        </div>

        <!-- Main Content -->
        <div class="max-w-7xl mx-auto px-6 py-8">
            <!-- Loading State -->
            <div v-if="isLoading" class="text-center py-12">
                <div
                    class="inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-[#143C8C]"
                ></div>
                <p class="mt-4 text-gray-600">Memuat data film...</p>
            </div>

            <!-- Error State -->
            <div v-else-if="error" class="text-center py-12">
                <p class="text-red-500">{{ error }}</p>
            </div>

            <!-- Content -->
            <div v-else class="space-y-6">
                <!-- Movie Info Card -->
                <div class="bg-white rounded-xl shadow-md p-6">
                    <h2 class="text-xl font-bold text-[#143C8C] mb-4">
                        Petualuan Sate Daguk
                    </h2>
                    <div class="flex gap-6">
                        <!-- Poster -->
                        <div
                            class="flex-shrink-0 w-48 rounded-lg overflow-hidden shadow-md"
                        >
                            <div class="aspect-[2/3] bg-gray-200">
                                <img
                                    :src="
                                        movie.poster_url || '/placeholder.jpg'
                                    "
                                    :alt="movie.title"
                                    class="w-full h-full object-cover"
                                    @error="
                                        (e) =>
                                            (e.target.src = '/placeholder.jpg')
                                    "
                                />
                            </div>
                        </div>

                        <!-- Info -->
                        <div class="flex-1">
                            <h3 class="text-2xl font-bold text-gray-900 mb-2">
                                {{ movie.title }}
                            </h3>
                            <p class="text-gray-600 mb-3">
                                {{ movie.genre }} • {{ movie.duration }} menit
                            </p>
                            <div class="flex items-center gap-2 mb-4">
                                <Star
                                    class="w-5 h-5 text-yellow-400 fill-yellow-400"
                                />
                                <span class="text-gray-900 font-semibold">
                                    {{ movie.rating || "0" }}
                                </span>
                                <span class="text-gray-500">/10</span>
                                <span
                                    class="ml-4 px-3 py-1 bg-blue-100 text-[#143C8C] rounded-full text-sm font-medium"
                                >
                                    13+
                                </span>
                            </div>
                            <p class="text-gray-600 leading-relaxed">
                                {{ movie.description }}
                            </p>
                        </div>
                    </div>
                </div>

                <!-- Date Selection -->
                <div class="bg-white rounded-xl shadow-md p-6">
                    <div class="flex items-center justify-between mb-4">
                        <div class="flex items-center gap-2">
                            <Calendar class="w-5 h-5 text-[#143C8C]" />
                            <h2 class="text-xl font-bold text-[#143C8C]">
                                Pilih Tanggal
                            </h2>
                        </div>

                    </div>
                    <div class="flex gap-3 overflow-x-auto pb-2">
                        <button
                            v-for="dateItem in dates"
                            :key="dateItem.full"
                            @click="selectedDate = dateItem.full"
                            class="flex-shrink-0 px-6 py-4 rounded-lg transition-all duration-200 border-2"
                            :class="
                                selectedDate === dateItem.full
                                    ? 'bg-[#143C8C] text-white border-[#143C8C] shadow-md'
                                    : 'bg-white text-gray-700 border-gray-200 hover:border-[#143C8C]'
                            "
                        >
                            <div class="text-center">
                                <div class="text-sm font-medium mb-1">
                                    {{ dateItem.day }}
                                </div>
                                <div class="font-bold whitespace-nowrap">
                                    {{ dateItem.date }}
                                </div>
                            </div>
                        </button>
                    </div>


                </div>

                <!-- Showtime Selection -->
                <div
                    v-if="selectedDate"
                    class="bg-white rounded-xl shadow-md p-6"
                >
                    <div class="flex items-center justify-between mb-4">
                        <div class="flex items-center gap-2">
                            <Clock class="w-5 h-5 text-[#143C8C]" />
                            <h2 class="text-xl font-bold text-[#143C8C]">
                                Jadwal Tayang
                            </h2>
                        </div>
                        <!-- Nearest Showtime Button - Moved Here -->
                        <button
                            @click="findNearestShowtime"
                            class="px-4 py-2 bg-[#143C8C] text-white rounded-lg hover:bg-[#0f2d6b] transition-all duration-200 font-medium flex items-center gap-2 shadow-md hover:shadow-lg text-sm"
                        >
                            <Clock class="w-4 h-4" />
                            Jadwal Terdekat
                        </button>
                    </div>

                    <!-- Showtime Info -->
                    <div class="mb-4 p-4 bg-gray-50 rounded-lg">
                        <div class="flex items-center gap-2 text-sm text-gray-600">
                            <MapPin class="w-4 h-4" />
                            <span>{{ selectedCinema ? getSelectedCinemaName() : 'Semua Bioskop' }}</span>
                        </div>
                        <div class="flex items-center gap-2 text-sm text-gray-600 mt-1">
                            <span>{{ formatDate(selectedDate) }}</span>
                        </div>
                    </div>


                    <!-- Showtimes Grouped by Cinema -->
                    <div v-if="filteredShowtimes.length > 0" class="space-y-6">
                        <div 
                            v-for="(group, index) in groupedShowtimes" 
                            :key="index"
                            class="border-2 border-gray-200 rounded-xl p-5"
                        >
                            <!-- Cinema Header -->
                            <div class="mb-4">
                                <h3 class="font-bold text-lg text-gray-900">{{ group.cinemaName }}</h3>
                                <div class="flex items-center gap-1 text-sm text-gray-600 mt-1">
                                    <MapPin class="w-3.5 h-3.5" />
                                    <span>{{ group.cinemaAddress }}</span>
                                </div>
                            </div>

                            <!-- Showtimes for this cinema -->
                            <div class="flex gap-3 overflow-x-auto pb-2">
                                <button
                                    v-for="showtime in group.showtimes"
                                    :key="showtime.id"
                                    @click="selectShowtime(showtime)"
                                    :class="[
                                        'flex-shrink-0 p-4 rounded-lg border-2 transition-all duration-200 min-w-[140px]',
                                        selectedShowtime?.id === showtime.id
                                            ? 'border-[#143C8C] bg-[#143C8C] text-white shadow-md'
                                            : isNearestShowtime(showtime) 
                                                ? 'border-[#143C8C] bg-blue-50 ring-2 ring-blue-300' 
                                                : 'border-gray-200 hover:border-[#143C8C] hover:bg-gray-50'
                                    ]"
                                >
                                    <div class="flex items-center gap-2 mb-2">
                                        <Clock class="w-4 h-4" :class="selectedShowtime?.id === showtime.id ? 'text-white' : 'text-gray-600'" />
                                        <div class="text-lg font-bold" :class="selectedShowtime?.id === showtime.id ? 'text-white' : 'text-gray-900'">
                                            {{ formatTime(showtime.start_time) }}
                                        </div>
                                    </div>
                                    <div class="text-sm" :class="selectedShowtime?.id === showtime.id ? 'text-blue-100' : 'text-gray-600'">
                                        Rp 35.000
                                    </div>
                                    <div class="text-xs mt-1" :class="selectedShowtime?.id === showtime.id ? 'text-blue-100' : 'text-gray-500'">
                                        {{ showtime.available_seats || 48 }} kursi tersedia
                                    </div>
                                    <div v-if="isNearestShowtime(showtime) && selectedShowtime?.id !== showtime.id" class="text-xs text-[#143C8C] font-semibold mt-1">
                                        Terdekat
                                    </div>
                                </button>
                            </div>
                        </div>
                    </div>

                    <!-- Confirmation Section -->
                    <div v-if="selectedShowtime" class="mt-6 p-6 bg-blue-50 border-2 border-[#143C8C] rounded-xl">
                        <div class="flex items-start justify-between mb-4">
                            <div>
                                <h3 class="text-lg font-bold text-[#143C8C] mb-2">Konfirmasi Jadwal</h3>
                                <div class="space-y-1 text-sm text-gray-700">
                                    <p><span class="font-semibold">Film:</span> {{ movie.title }}</p>
                                    <p><span class="font-semibold">Bioskop:</span> {{ getSelectedCinemaName() }}</p>
                                    <p><span class="font-semibold">Tanggal:</span> {{ formatDate(selectedDate) }}</p>
                                    <p><span class="font-semibold">Waktu:</span> {{ formatTime(selectedShowtime.start_time) }}</p>
                                    <p><span class="font-semibold">Harga:</span> Rp 35.000</p>
                                </div>
                            </div>
                        </div>
                        <div class="flex gap-3">
                            <button
                                @click="confirmShowtime"
                                class="flex-1 px-6 py-3 bg-[#143C8C] text-white rounded-lg hover:bg-[#0f2d6b] transition-colors font-medium shadow-md"
                            >
                                Lanjut ke Pemesanan
                            </button>
                            <button
                                @click="cancelShowtime"
                                class="px-6 py-3 bg-gray-200 text-gray-700 rounded-lg hover:bg-gray-300 transition-colors font-medium"
                            >
                                Batal
                            </button>
                        </div>
                    </div>

                    <!-- No Showtimes -->
                    <div v-else-if="filteredShowtimes.length === 0" class="text-center py-8 text-gray-500">
                        <p>Tidak ada jadwal tayang untuk tanggal yang dipilih</p>
                    </div>
                </div>

                <!-- Prompt to select date and cinema -->
                <div
                    v-else
                    class="bg-white rounded-xl shadow-md p-8 text-center"
                >
                    <div class="text-gray-400 mb-2">
                        <Clock class="w-12 h-12 mx-auto mb-3" />
                    </div>
                    <p class="text-gray-600">
                        Pilih tanggal dan bioskop untuk melihat jadwal tayang
                    </p>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup>
import { ref, computed, onMounted, watch } from "vue";
import { useRoute, useRouter } from "vue-router";
import {
    ArrowLeft,
    Film,
    Calendar,
    Clock,
    MapPin,
    Star,
} from "lucide-vue-next";
import { cinemas } from "../data/cinemas";

const route = useRoute();
const router = useRouter();

const goBack = () => {
    router.back();
};

// State
const isLoading = ref(true);
const error = ref(null);
const movie = ref({
    id: 0,
    title: "",
    genre: "",
    rating: "0",
    poster_url: "",
    description: "",
    duration: 0,
});
const showtimeList = ref([]);
const selectedDate = ref(null);
const selectedCinema = ref(null);
const nearestShowtimeId = ref(null); // Track nearest showtime for highlighting
const selectedShowtime = ref(null); // Track selected showtime for confirmation

// Fetch movie data
const fetchMovieData = async () => {
    const movieId = route.params.movieId;

    try {
        isLoading.value = true;
        const response = await fetch(`http://127.0.0.1:3000/api/movies`);

        if (!response.ok) {
            throw new Error("Gagal mengambil data film");
        }

        const result = await response.json();
        const movies = result.data || [];

        const foundMovie = movies.find(
            (m) => m.id === parseInt(movieId)
        );

        if (foundMovie) {
            movie.value = foundMovie;
        } else {
            error.value = "Film tidak ditemukan";
        }
    } catch (err) {
        error.value = err instanceof Error ? err.message : "Terjadi kesalahan";
        console.error("Error fetching movie:", err);
    } finally {
        isLoading.value = false;
    }
};

// Fetch showtimes
const fetchShowtimes = async (movieId) => {
    try {
        const response = await fetch(
            `http://127.0.0.1:3000/api/showtimes/movie/${movieId}`
        );
        const result = await response.json();
        if (result.success) {
            showtimeList.value = result.data;
        }
    } catch (err) {
        console.error("Error fetching showtimes:", err);
    }
};

// Computed: Available dates from showtimes
const dates = computed(() => {
    if (!showtimeList.value.length) return [];

    const uniqueDates = new Set();
    const result = [];

    showtimeList.value.forEach((st) => {
        const dateObj = new Date(st.start_time);
        const fullDate = dateObj.toISOString().split("T")[0];

        if (!uniqueDates.has(fullDate)) {
            uniqueDates.add(fullDate);
            const dayName = dateObj.toLocaleDateString("id-ID", {
                weekday: "short",
            });
            const dateStr = dateObj.toLocaleDateString("id-ID", {
                day: "numeric",
                month: "short",
            });

            result.push({
                date: dateStr,
                day: dayName,
                full: fullDate,
            });
        }
    });

    return result.sort((a, b) => a.full.localeCompare(b.full));
});

// Computed: Filtered showtimes based on selected date and cinema
const filteredShowtimes = computed(() => {
    if (!selectedDate.value) return [];

    // If no cinema selected, show ALL showtimes for the selected date (all cinemas)
    if (!selectedCinema.value) {
        return showtimeList.value.filter((st) => {
            const dateObj = new Date(st.start_time);
            const fullDate = dateObj.toISOString().split("T")[0];
            return fullDate === selectedDate.value;
        });
    }

    // Mapping cinema slug to studio IDs
    const cinemaToStudioMap = {
        'ewalk-xxi': [1, 2],                                    // E-WALK XXI has studio 1 and 2
        'cgv-plaza-balikpapan': [3],                           // CGV Plaza has studio 3
        'cinepolis-living-plaza-balikpapan': [4],              // Cinepolis has studio 4
        'pentacity-xxi': [1, 2],                               // Pentacity XXI (same as XXI)
        'studio-xxi': [1, 2],                                  // Studio XXI (same as XXI)
    };

    // Get studio IDs for selected cinema
    const studioIds = cinemaToStudioMap[selectedCinema.value] || [];

    return showtimeList.value.filter((st) => {
        const dateObj = new Date(st.start_time);
        const fullDate = dateObj.toISOString().split("T")[0];
        
        // Filter by date AND studio (cinema)
        return fullDate === selectedDate.value && studioIds.includes(st.studio_id);
    });
});

// Computed: Group showtimes by cinema (for TIX-like display)
const groupedShowtimes = computed(() => {
    if (!selectedDate.value) return [];

    const showtimes = filteredShowtimes.value;
    const grouped = {};

    // Group showtimes by cinema name (not studio_id)
    showtimes.forEach((showtime) => {
        const cinemaName = getCinemaNameFromStudio(showtime.studio_id);
        
        if (!grouped[cinemaName]) {
            grouped[cinemaName] = {
                cinemaName: cinemaName,
                cinemaAddress: getCinemaAddressFromStudio(showtime.studio_id),
                showtimes: []
            };
        }
        grouped[cinemaName].showtimes.push(showtime);
    });

    // Convert to array and sort by cinema name
    return Object.values(grouped).sort((a, b) => a.cinemaName.localeCompare(b.cinemaName));
});

// Helper functions
const formatDate = (dateStr) => {
    if (!dateStr) return "-";
    const d = new Date(dateStr);
    return d.toLocaleDateString("id-ID", {
        weekday: "long",
        year: "numeric",
        month: "long",
        day: "numeric",
    });
};

const formatTime = (dateTimeStr) => {
    const dateObj = new Date(dateTimeStr);
    return dateObj
        .toLocaleTimeString("id-ID", {
            hour: "2-digit",
            minute: "2-digit",
        })
        .replace(".", ":");
};

const getCinemaNameFromStudio = (studioId) => {
    // Map studio ID to cinema name
    const studioToCinemaMap = {
        1: 'E-WALK XXI',
        2: 'E-WALK XXI',
        3: 'CGV Plaza Balikpapan',
        4: 'Cinepolis Living Plaza',
        5: 'PENTACITY XXI',
        6: 'STUDIO XXI',
    };
    return studioToCinemaMap[studioId] || 'Unknown Cinema';
};

const getCinemaAddressFromStudio = (studioId) => {
    // Map studio ID to cinema address
    const studioToAddressMap = {
        1: 'E-Walk Balikpapan Superblock',
        2: 'E-Walk Balikpapan Superblock',
        3: 'Plaza Balikpapan, Jl. Jenderal Sudirman',
        4: 'Living Plaza Balikpapan, Jl. MT Haryono',
        5: 'Pentacity Mall Balikpapan',
        6: 'Jl. Jenderal Sudirman',
    };
    return studioToAddressMap[studioId] || '';
};

const getSelectedCinemaName = () => {
    const cinema = cinemas.find((c) => c.slug === selectedCinema.value);
    return cinema ? cinema.name : "";
};

const isNearestShowtime = (showtime) => {
    return nearestShowtimeId.value === showtime.id;
};

const selectShowtime = (showtime) => {
    selectedShowtime.value = showtime;
    // Scroll to confirmation section
    setTimeout(() => {
        const confirmSection = document.querySelector('.bg-blue-50.border-2');
        if (confirmSection) {
            confirmSection.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
        }
    }, 100);
};

const confirmShowtime = () => {
    if (selectedShowtime.value) {
        router.push({
            name: "PemesananFilm",
            params: { movieId: route.params.movieId },
            query: { showtimeId: selectedShowtime.value.id },
        });
    }
};

const cancelShowtime = () => {
    selectedShowtime.value = null;
};

// Find nearest showtime function (multiprocessing concept)
const findNearestShowtime = () => {
    if (showtimeList.value.length === 0) {
        return;
    }

    const now = new Date();
    
    // Filter only future showtimes (yang belum lewat) - ACROSS ALL DATES AND CINEMAS
    const futureShowtimes = showtimeList.value.filter((st) => {
        const showtimeDate = new Date(st.start_time);
        return showtimeDate > now;
    });

    if (futureShowtimes.length === 0) {
        alert('Tidak ada jadwal tayang yang tersedia untuk waktu mendatang.');
        return;
    }

    // Multiprocessing concept: Process all showtimes in parallel to find the nearest
    // Calculate time difference for each showtime
    const showtimesWithDiff = futureShowtimes.map((st) => {
        const showtimeDate = new Date(st.start_time);
        const timeDiff = showtimeDate - now; // difference in milliseconds
        return {
            showtime: st,
            diff: timeDiff,
            date: showtimeDate.toISOString().split("T")[0],
        };
    });

    // Sort by time difference (ascending) to get the nearest
    showtimesWithDiff.sort((a, b) => a.diff - b.diff);

    // Get the nearest showtime
    const nearest = showtimesWithDiff[0];
    
    // Set nearest showtime ID for highlighting
    nearestShowtimeId.value = nearest.showtime.id;
    
    // Auto-select the date of nearest showtime
    selectedDate.value = nearest.date;
    
    // Reset cinema selection to show ALL cinemas for that date
    selectedCinema.value = null;
    
    // Scroll to showtime section
    setTimeout(() => {
        const showtimeSection = document.querySelector('.grid.grid-cols-2');
        if (showtimeSection) {
            showtimeSection.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
        }
    }, 300);
};

// Watch for date changes and reset cinema selection
watch(selectedDate, () => {
    selectedCinema.value = null;
});

// Lifecycle
onMounted(async () => {
    await fetchMovieData();
    await fetchShowtimes(parseInt(route.params.movieId));
});
</script>

<style scoped>
/* Custom scrollbar for date selection */
.overflow-x-auto::-webkit-scrollbar {
    height: 6px;
}

.overflow-x-auto::-webkit-scrollbar-track {
    background: #f1f1f1;
    border-radius: 10px;
}

.overflow-x-auto::-webkit-scrollbar-thumb {
    background: #143c8c;
    border-radius: 10px;
}

.overflow-x-auto::-webkit-scrollbar-thumb:hover {
    background: #0f2d6b;
}

/* Pulse animation for nearest showtime */
@keyframes pulse-slow {
    0%, 100% {
        opacity: 1;
    }
    50% {
        opacity: 0.8;
    }
}

.animate-pulse-slow {
    animation: pulse-slow 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
}
</style>
