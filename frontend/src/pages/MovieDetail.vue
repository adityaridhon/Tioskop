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
                        <!-- Toggle Nearest Showtime Button -->
                        <button
                            @click="toggleNearestMode"
                            :class="[
                                'px-4 py-2 rounded-lg transition-all duration-200 font-medium flex items-center gap-2 shadow-md hover:shadow-lg text-sm',
                                isNearestMode
                                    ? 'bg-green-600 text-white hover:bg-green-700'
                                    : 'bg-[#143C8C] text-white hover:bg-[#0f2d6b]',
                            ]"
                        >
                            <Clock class="w-4 h-4" />
                            <span v-if="!isNearestMode"
                                >Aktifkan Jadwal Terdekat</span
                            >
                            <span v-else>Nonaktifkan Filter</span>
                            <svg
                                v-if="isNearestMode"
                                class="w-4 h-4"
                                fill="none"
                                stroke="currentColor"
                                viewBox="0 0 24 24"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M5 13l4 4L19 7"
                                />
                            </svg>
                        </button>
                    </div>

                    <!-- Showtime Info -->
                    <div class="mb-4 p-4 bg-gray-50 rounded-lg">
                        <div class="flex items-center justify-between">
                            <div>
                                <div
                                    class="flex items-center gap-2 text-sm text-gray-600"
                                >
                                    <MapPin class="w-4 h-4" />
                                    <span>{{
                                        selectedCinema
                                            ? getSelectedCinemaName()
                                            : "Semua Bioskop"
                                    }}</span>
                                </div>
                                <div
                                    class="flex items-center gap-2 text-sm text-gray-600 mt-1"
                                >
                                    <span>{{ formatDate(selectedDate) }}</span>
                                </div>
                            </div>
                            <!-- Active Filter Badge -->
                            <div
                                v-if="isNearestMode"
                                class="flex items-center gap-2 px-3 py-1.5 bg-green-100 text-green-700 rounded-lg text-sm font-medium"
                            >
                                <svg
                                    class="w-4 h-4"
                                    fill="currentColor"
                                    viewBox="0 0 20 20"
                                >
                                    <path
                                        fill-rule="evenodd"
                                        d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                                        clip-rule="evenodd"
                                    />
                                </svg>
                                <span>Filter Aktif</span>
                            </div>
                        </div>
                    </div>

                    <!-- Showtimes Grouped by Cinema -->
                    <!-- Mode 1: Normal View - Card per Cinema dengan jadwal horizontal -->
                    <div
                        v-if="!isNearestMode && filteredShowtimes.length > 0"
                        class="space-y-6"
                    >
                        <div
                            v-for="(group, index) in groupedShowtimes"
                            :key="index"
                            class="border-2 border-gray-200 rounded-xl p-5"
                        >
                            <!-- Cinema Header -->
                            <div class="mb-4">
                                <h3 class="font-bold text-lg text-gray-900">
                                    {{ group.cinemaName }}
                                </h3>
                                <div
                                    class="flex items-center gap-1 text-sm text-gray-600 mt-1"
                                >
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
                                            : 'border-gray-200 hover:border-[#143C8C] hover:bg-gray-50',
                                    ]"
                                >
                                    <div class="flex items-center gap-2 mb-2">
                                        <Clock
                                            class="w-4 h-4"
                                            :class="
                                                selectedShowtime?.id ===
                                                showtime.id
                                                    ? 'text-white'
                                                    : 'text-gray-600'
                                            "
                                        />
                                        <div
                                            class="text-lg font-bold"
                                            :class="
                                                selectedShowtime?.id ===
                                                showtime.id
                                                    ? 'text-white'
                                                    : 'text-gray-900'
                                            "
                                        >
                                            {{
                                                formatTime(showtime.start_time)
                                            }}
                                        </div>
                                    </div>
                                    <div
                                        class="text-sm"
                                        :class="
                                            selectedShowtime?.id === showtime.id
                                                ? 'text-blue-100'
                                                : 'text-gray-600'
                                        "
                                    >
                                        {{ formatPrice(showtime.price) }}
                                    </div>
                                    <div
                                        class="text-xs mt-1"
                                        :class="
                                            selectedShowtime?.id === showtime.id
                                                ? 'text-blue-100'
                                                : 'text-gray-500'
                                        "
                                    >
                                        {{
                                            calculateAvailableSeats(
                                                showtime.studio_id
                                            )
                                        }}
                                        kursi tersedia
                                    </div>
                                </button>
                            </div>
                        </div>
                    </div>

                    <!-- Mode 2: Nearest View - Card per Showtime compact style -->
                    <div
                        v-if="isNearestMode && filteredShowtimes.length > 0"
                        class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-3"
                    >
                        <template
                            v-for="(group, index) in groupedShowtimes"
                            :key="index"
                        >
                            <!-- Each Showtime Card -->
                            <div
                                v-for="showtime in group.showtimes"
                                :key="showtime.id"
                                @click="selectShowtime(showtime)"
                                class="bg-white border border-gray-200 rounded-lg p-4 hover:shadow-lg hover:border-[#143C8C] transition-all duration-200 flex flex-col cursor-pointer min-w-[140px]"
                            >
                                <!-- Cinema Name -->
                                <h3
                                    class="font-bold text-sm text-gray-900 mb-2 text-center line-clamp-1"
                                >
                                    {{ group.cinemaName }}
                                </h3>

                                <!-- Time Badge -->
                                <div class="text-center mb-2">
                                    <div
                                        class="text-lg font-bold text-gray-900"
                                    >
                                        {{ formatTime(showtime.start_time) }}
                                    </div>
                                </div>

                                <!-- Price -->
                                <div class="text-center mb-1">
                                    <div class="text-sm text-gray-600">
                                        {{ formatPrice(showtime.price) }}
                                    </div>
                                </div>

                                <!-- Available Seats -->
                                <div class="text-center text-xs text-gray-500">
                                    {{
                                        calculateAvailableSeats(
                                            showtime.studio_id
                                        )
                                    }}
                                    kursi tersedia
                                </div>
                            </div>
                        </template>
                    </div>

                    <!-- Confirmation Section -->
                    <div
                        v-if="selectedShowtime && !isNearestMode"
                        class="mt-6 p-6 bg-gradient-to-r from-blue-50 to-indigo-50 border-2 border-[#143C8C] rounded-xl shadow-lg"
                    >
                        <div class="flex items-start justify-between mb-4">
                            <div class="flex-1">
                                <div class="flex items-center gap-2 mb-3">
                                    <div class="bg-[#143C8C] p-2 rounded-lg">
                                        <svg
                                            class="w-5 h-5 text-white"
                                            fill="none"
                                            stroke="currentColor"
                                            viewBox="0 0 24 24"
                                        >
                                            <path
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                stroke-width="2"
                                                d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
                                            />
                                        </svg>
                                    </div>
                                    <h3
                                        class="text-xl font-bold text-[#143C8C]"
                                    >
                                        Jadwal Terpilih
                                    </h3>
                                </div>
                                <div class="grid grid-cols-2 gap-3 text-sm">
                                    <div>
                                        <p class="text-gray-500 mb-1">Film</p>
                                        <p class="font-semibold text-gray-900">
                                            {{ movie.title }}
                                        </p>
                                    </div>
                                    <div>
                                        <p class="text-gray-500 mb-1">
                                            Bioskop
                                        </p>
                                        <p class="font-semibold text-gray-900">
                                            {{
                                                getCinemaNameFromStudio(
                                                    selectedShowtime.studio_id
                                                )
                                            }}
                                        </p>
                                    </div>
                                    <div>
                                        <p class="text-gray-500 mb-1">
                                            Tanggal & Waktu
                                        </p>
                                        <p class="font-semibold text-gray-900">
                                            {{
                                                formatDateShort(
                                                    selectedShowtime.start_time
                                                )
                                            }}
                                            •
                                            {{
                                                formatTime(
                                                    selectedShowtime.start_time
                                                )
                                            }}
                                        </p>
                                    </div>
                                    <div>
                                        <p class="text-gray-500 mb-1">Harga</p>
                                        <p
                                            class="font-semibold text-[#143C8C] text-lg"
                                        >
                                            {{
                                                formatPrice(
                                                    selectedShowtime.price
                                                )
                                            }}
                                        </p>
                                    </div>
                                </div>
                            </div>
                        </div>
                        <div class="flex gap-3">
                            <button
                                @click="confirmShowtime"
                                class="flex-1 px-6 py-3 bg-[#143C8C] text-white rounded-lg hover:bg-[#0f2d6b] transition-all duration-200 font-semibold shadow-md hover:shadow-lg flex items-center justify-center gap-2"
                            >
                                <svg
                                    class="w-5 h-5"
                                    fill="none"
                                    stroke="currentColor"
                                    viewBox="0 0 24 24"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M13 7l5 5m0 0l-5 5m5-5H6"
                                    />
                                </svg>
                                Lanjut ke Pemilihan Kursi
                            </button>
                            <button
                                @click="cancelShowtime"
                                class="px-6 py-3 bg-white border-2 border-gray-300 text-gray-700 rounded-lg hover:bg-gray-50 transition-colors font-medium"
                            >
                                Batal
                            </button>
                        </div>
                    </div>

                    <!-- No Showtimes -->
                    <div
                        v-else-if="filteredShowtimes.length === 0"
                        class="text-center py-8 text-gray-500"
                    >
                        <p>
                            Tidak ada jadwal tayang untuk tanggal yang dipilih
                        </p>
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
const isNearestMode = ref(false); // Track if in "nearest showtime" mode

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

        const foundMovie = movies.find((m) => m.id === parseInt(movieId));

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
        "ewalk-xxi": [1, 2], // E-WALK XXI has studio 1 and 2
        "cgv-plaza-balikpapan": [3], // CGV Plaza has studio 3
        "cinepolis-living-plaza-balikpapan": [4], // Cinepolis has studio 4
        "pentacity-xxi": [1, 2], // Pentacity XXI (same as XXI)
        "studio-xxi": [1, 2], // Studio XXI (same as XXI)
    };

    // Get studio IDs for selected cinema
    const studioIds = cinemaToStudioMap[selectedCinema.value] || [];

    return showtimeList.value.filter((st) => {
        const dateObj = new Date(st.start_time);
        const fullDate = dateObj.toISOString().split("T")[0];

        // Filter by date AND studio (cinema)
        return (
            fullDate === selectedDate.value && studioIds.includes(st.studio_id)
        );
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
                showtimes: [],
            };
        }
        grouped[cinemaName].showtimes.push(showtime);
    });

    // Convert to array and sort by cinema name
    return Object.values(grouped).sort((a, b) =>
        a.cinemaName.localeCompare(b.cinemaName)
    );
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

const formatDateShort = (dateTimeStr) => {
    const dateObj = new Date(dateTimeStr);
    return dateObj.toLocaleDateString("id-ID", {
        year: "numeric",
        month: "2-digit",
        day: "2-digit",
    });
};

const formatPrice = (price) => {
    if (!price) return "Rp 0";
    return `Rp ${parseInt(price).toLocaleString("id-ID")}`;
};

const calculateTimeRemaining = (dateTimeStr) => {
    const now = new Date();
    const showtimeDate = new Date(dateTimeStr);
    const diff = showtimeDate - now;

    if (diff < 0) return "Sudah lewat";

    const hours = Math.floor(diff / (1000 * 60 * 60));
    const minutes = Math.floor((diff % (1000 * 60 * 60)) / (1000 * 60));

    if (hours > 24) {
        const days = Math.floor(hours / 24);
        return `${days} hari lagi`;
    }

    return `${hours} jam ${minutes} menit lagi`;
};

const calculateAvailableSeats = (studioId) => {
    // Default capacity per studio (you can adjust this based on your actual data)
    const studioCapacities = {
        1: 45,
        2: 40,
        3: 42,
        4: 50,
        5: 38,
        6: 45,
    };

    // For now, return random available seats (you can later integrate with actual booking data)
    const capacity = studioCapacities[studioId] || 50;
    const bookedSeats = Math.floor(Math.random() * 10); // Simulate some booked seats
    return capacity - bookedSeats;
};

const getCinemaNameFromStudio = (studioId) => {
    // Map studio ID to cinema name
    const studioToCinemaMap = {
        1: "E-WALK XXI",
        2: "E-WALK XXI",
        3: "CGV Plaza Balikpapan",
        4: "Cinepolis Living Plaza",
        5: "PENTACITY XXI",
        6: "STUDIO XXI",
    };
    return studioToCinemaMap[studioId] || "Unknown Cinema";
};

const getCinemaAddressFromStudio = (studioId) => {
    // Map studio ID to cinema address
    const studioToAddressMap = {
        1: "E-Walk Balikpapan Superblock",
        2: "E-Walk Balikpapan Superblock",
        3: "Plaza Balikpapan, Jl. Jenderal Sudirman",
        4: "Living Plaza Balikpapan, Jl. MT Haryono",
        5: "Pentacity Mall Balikpapan",
        6: "Jl. Jenderal Sudirman",
    };
    return studioToAddressMap[studioId] || "";
};

const getSelectedCinemaName = () => {
    const cinema = cinemas.find((c) => c.slug === selectedCinema.value);
    return cinema ? cinema.name : "";
};

const isNearestShowtime = (showtime) => {
    return nearestShowtimeId.value === showtime.id;
};

const selectShowtime = (showtime) => {
    // If in nearest mode, go directly to booking page
    if (isNearestMode.value) {
        router.push({
            name: "PemesananFilm",
            params: { movieId: route.params.movieId },
            query: { showtimeId: showtime.id },
        });
        return;
    }

    // Normal mode: show confirmation section
    selectedShowtime.value = showtime;
    // Scroll to confirmation section
    setTimeout(() => {
        const confirmSection = document.querySelector(".bg-blue-50.border-2");
        if (confirmSection) {
            confirmSection.scrollIntoView({
                behavior: "smooth",
                block: "nearest",
            });
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

// Toggle nearest mode on/off
const toggleNearestMode = () => {
    if (isNearestMode.value) {
        // Turn OFF: Reset to normal mode
        isNearestMode.value = false;
        nearestShowtimeId.value = null;
    } else {
        // Turn ON: Find and show nearest showtimes
        findNearestShowtime();
    }
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
        alert("Tidak ada jadwal tayang yang tersedia untuk waktu mendatang.");
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

    // Enable nearest mode
    isNearestMode.value = true;

    // Auto-select the date of nearest showtime
    selectedDate.value = nearest.date;

    // Reset cinema selection to show ALL cinemas for that date
    selectedCinema.value = null;

    // Scroll to showtime section
    setTimeout(() => {
        const showtimeSection = document.querySelector(".space-y-4");
        if (showtimeSection) {
            showtimeSection.scrollIntoView({
                behavior: "smooth",
                block: "nearest",
            });
        }
    }, 300);
};

// Watch for dates to auto-select first date
watch(dates, (newDates) => {
    if (newDates.length > 0 && !selectedDate.value) {
        selectedDate.value = newDates[0].full;
    }
});

// Watch for date changes and reset cinema selection + disable nearest mode
watch(selectedDate, () => {
    selectedCinema.value = null;
    // Only disable nearest mode if user manually changes date
    if (!nearestShowtimeId.value) {
        isNearestMode.value = false;
    }
});

// Watch for cinema filter changes - disable nearest mode
watch(selectedCinema, () => {
    if (selectedCinema.value !== null) {
        isNearestMode.value = false;
        nearestShowtimeId.value = null;
    }
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
    0%,
    100% {
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
