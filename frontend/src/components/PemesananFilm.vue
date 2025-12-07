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
                        <h1 class="text-3xl mb-1">Pemesanan Tiket</h1>
                        <p class="text-blue-100">
                            Pilih jadwal dan tempat duduk Anda
                        </p>
                    </div>
                </div>
            </div>
        </div>

        <!-- Main Content -->
        <div class="max-w-7xl mx-auto px-6 py-8">
            <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
                <!-- Left Column -->
                <div class="lg:col-span-2 space-y-6">
                    <!-- Selected Movie Info -->
                    <div class="bg-white rounded-xl shadow-md p-6">
                        <div class="flex items-center gap-2 mb-4">
                            <Film class="w-5 h-5 text-[#143C8C]" />
                            <h2 class="text-[#143C8C]">Film yang Dipilih</h2>
                        </div>

                        <!-- Loading State -->
                        <div v-if="isLoading" class="text-center py-8">
                            <div
                                class="inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-blue-900"
                            ></div>
                            <p class="mt-4 text-gray-600">
                                Memuat data film...
                            </p>
                        </div>

                        <!-- Error State -->
                        <div v-else-if="error" class="text-center py-8">
                            <p class="text-red-500">{{ error }}</p>
                        </div>

                        <!-- Movie Info -->
                        <div v-else class="flex gap-4">
                            <!-- Movie Poster -->
                            <div
                                class="flex-shrink-0 w-32 rounded-lg overflow-hidden shadow-md"
                            >
                                <div class="aspect-[2/3] bg-gray-200">
                                    <img
                                        :src="
                                            selectedMovie.poster_url ||
                                            '/placeholder.jpg'
                                        "
                                        :alt="selectedMovie.title"
                                        class="w-full h-full object-cover"
                                        @error="
                                            (e) => {
                                                const target = e.target as HTMLImageElement;
                                                if (target) target.src = '/placeholder.jpg';
                                            }
                                        "
                                        loading="lazy"
                                    />
                                </div>
                            </div>

                            <!-- Movie Info -->
                            <div class="flex-1 flex flex-col justify-center">
                                <h3 class="text-gray-900 mb-2">
                                    {{ selectedMovie.title }}
                                </h3>
                                <p class="text-gray-600 mb-3">
                                    {{ selectedMovie.genre }}
                                </p>
                                <div class="flex items-center gap-2 mb-3">
                                    <Star
                                        class="w-5 h-5 text-yellow-400 fill-yellow-400"
                                    />
                                    <span class="text-gray-900">
                                        {{ selectedMovie.rating || "0" }}
                                    </span>
                                    <span class="text-gray-500">/10</span>
                                </div>
                                <p
                                    class="text-gray-600 text-sm leading-relaxed"
                                >
                                    {{ selectedMovie.description }}
                                </p>
                            </div>
                        </div>
                    </div>


                    <!-- Seat Selection -->
                    <div class="bg-white rounded-xl shadow-md p-6">
                        <h2 class="text-[#143C8C] mb-6">Pilih Tempat Duduk</h2>

                        <!-- Screen -->
                        <div class="mb-8">
                            <div
                                class="bg-gradient-to-b from-gray-200 to-gray-100 rounded-t-3xl py-2 text-center text-gray-600 text-sm mb-2"
                            >
                                LAYAR
                            </div>
                            <div
                                class="h-1 bg-gradient-to-b from-gray-300 to-transparent rounded-full"
                            ></div>
                        </div>

                        <!-- Seats -->
                        <div class="flex justify-center mb-6">
                            <div class="space-y-2">
                                <div
                                    v-for="row in rows"
                                    :key="row"
                                    class="flex items-center gap-2"
                                >
                                    <div
                                        class="w-6 text-center text-gray-600 text-sm"
                                    >
                                        {{ row }}
                                    </div>
                                    <div class="flex gap-2">
                                        <button
                                            v-for="i in seatsPerRow"
                                            :key="`${row}${i}`"
                                            :title="`${row}${i}`"
                                            :disabled="
                                                occupiedSeats.includes(
                                                    `${row}${i}`
                                                )
                                            "
                                            :class="
                                                getSeatClasses(`${row}${i}`)
                                            "
                                            @click="toggleSeat(`${row}${i}`)"
                                        >
                                            <Armchair
                                                v-if="
                                                    selectedSeats.includes(
                                                        `${row}${i}`
                                                    )
                                                "
                                                class="w-4 h-4"
                                            />
                                        </button>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <!-- Legend -->
                        <div
                            class="flex flex-wrap gap-4 justify-center pt-4 border-t"
                        >
                            <div class="flex items-center gap-2">
                                <div
                                    class="w-6 h-6 bg-gray-200 rounded-md"
                                ></div>
                                <span class="text-sm text-gray-600"
                                    >Tersedia</span
                                >
                            </div>
                            <div class="flex items-center gap-2">
                                <div
                                    class="w-6 h-6 bg-[#143C8C] rounded-md"
                                ></div>
                                <span class="text-sm text-gray-600"
                                    >Dipilih</span
                                >
                            </div>
                            <div class="flex items-center gap-2">
                                <div
                                    class="w-6 h-6 bg-gray-400 rounded-md"
                                ></div>
                                <span class="text-sm text-gray-600"
                                    >Terisi</span
                                >
                            </div>
                        </div>
                    </div>
                </div>

                <!-- Right Column - Order Summary -->
                <div class="lg:col-span-1">
                    <div class="lg:sticky lg:top-6">
                        <div
                            class="bg-white rounded-xl shadow-md p-6 sticky top-6"
                        >
                            <div class="flex items-center gap-2 mb-6">
                                <Receipt class="w-5 h-5 text-[#143C8C]" />
                                <h2 class="text-[#143C8C]">
                                    Ringkasan Pesanan
                                </h2>
                            </div>

                            <div class="space-y-4">
                                <!-- Movie Info -->
                                <div class="pb-4 border-b">
                                    <h3 class="text-gray-900 mb-2">
                                        {{ selectedMovie.title }}
                                    </h3>
                                    <p class="text-gray-500 text-sm">
                                        {{ selectedMovie.genre }}
                                    </p>
                                </div>





                                <!-- Seats Info -->
                                <div
                                    v-if="selectedSeats.length > 0"
                                    class="flex items-start gap-3 pb-4 border-b"
                                >
                                    <Armchair
                                        class="w-5 h-5 text-[#143C8C] mt-0.5"
                                    />
                                    <div class="flex-1">
                                        <p class="text-gray-600 text-sm mb-2">
                                            Kursi
                                        </p>
                                        <div class="flex flex-wrap gap-2">
                                            <span
                                                v-for="seat in sortedSeats"
                                                :key="seat"
                                                class="px-2 py-1 rounded text-sm bg-gray-100 text-gray-700"
                                            >
                                                {{ seat }}
                                            </span>
                                        </div>
                                    </div>
                                </div>

                                <!-- Price Breakdown -->
                                <div
                                    v-if="selectedSeats.length > 0"
                                    class="space-y-3 pt-2"
                                >
                                    <div class="flex justify-between text-sm">
                                        <span class="text-gray-600">
                                            {{ selectedSeats.length }}
                                            Tiket x
                                            {{ formatCurrency(TICKET_PRICE) }}
                                        </span>
                                        <span class="text-gray-900">
                                            {{ formatCurrency(subtotal) }}
                                        </span>
                                    </div>
                                    <div class="flex justify-between text-sm">
                                        <span class="text-gray-600">
                                            Biaya Admin
                                        </span>
                                        <span class="text-gray-900">
                                            {{ formatCurrency(adminFee) }}
                                        </span>
                                    </div>
                                    <div
                                        class="flex justify-between pt-3 border-t"
                                    >
                                        <span class="text-gray-900">
                                            Total
                                        </span>
                                        <span class="text-[#143C8C] text-xl">
                                            {{ formatCurrency(total) }}
                                        </span>
                                    </div>
                                </div>

                                <!-- Action Button -->
                                <button
                                    @click="handleConfirm"
                                    :disabled="!canConfirm"
                                    class="w-full bg-[#143C8C] text-white py-3 rounded-lg mt-6 transition-all duration-200 disabled:bg-gray-300 disabled:cursor-not-allowed hover:bg-[#0f2d6b] disabled:hover:bg-gray-300 shadow-md hover:shadow-lg flex items-center justify-center gap-2"
                                >
                                    <CreditCard class="w-5 h-5" />
                                    <span>Konfirmasi Pesanan</span>
                                </button>

                                <p
                                    v-if="selectedSeats.length === 0"
                                    class="text-center text-sm text-gray-500 mt-2"
                                >
                                    Pilih kursi untuk melanjutkan
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
                <!-- End Right Column -->
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from "vue";
import { useRoute, useRouter } from "vue-router";
// @ts-ignore - useAuth is a JavaScript file without type declarations
import { useAuth } from "@/composables/useAuth";
import {
    ArrowLeft,
    Film,
    Calendar,
    Clock,
    Armchair,
    Receipt,
    CreditCard,
    Star,
} from "lucide-vue-next"; // atau ganti dengan ikon lain yang kamu pakai

const route = useRoute();
const router = useRouter();
const { token, user } = useAuth();

const goBack = () => {
    router.back();
};

// Data film (akan diambil dari API)
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
    title: "Loading...",
    genre: "",
    rating: "0",
    poster_url: "",
    description: "",
    duration: 0,
});

const isLoading = ref(true);
const error = ref<string | null>(null);

// Fetch movie data dari API
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

        // Cari film berdasarkan movieId
        const movie = movies.find(
            (m: Movie) => m.id === parseInt(movieId as string)
        );

        if (movie) {
            selectedMovie.value = movie;
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

// Load movie data saat component mounted
onMounted(async () => {
    await fetchMovieData();
    await fetchShowtimes(parseInt(route.params.movieId as string));
    
    // Get showtimeId from query params and fetch seats
    const showtimeId = route.query.showtimeId;
    if (showtimeId) {
        const id = parseInt(showtimeId as string);
        // Find showtime from list
        const showtime = showtimeList.value.find(st => st.id === id);
        if (showtime) {
            selectedShowtime.value = showtime;
            await fetchSeats(id);
        }
    }
});

const selectedDate = ref<string | null>(null);

const selectedSeats = ref<string[]>([]);

// Data untuk schedule
// Data untuk schedule
const dates = computed(() => {
    if (!showtimeList.value.length) return [];

    const uniqueDates = new Set();
    const result: any[] = [];

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





// Data kursi
const rows = ["A", "B", "C", "D", "E", "F", "G", "H"];
const seatsPerRow = 10;
const occupiedSeats = ref<string[]>([]);
const allSeats = ref<any[]>([]);
const selectedShowtime = ref<any>(null);

// Fetch seats for showtime
const fetchSeats = async (showtimeId: number) => {
    try {
        const response = await fetch(
            `http://127.0.0.1:3000/api/seats/showtime/${showtimeId}`
        );
        const result = await response.json();
        console.log("=== Fetched Seats ===");
        console.log("Total seats:", result.data?.length);
        console.log("First 5 seats:", result.data?.slice(0, 5));
        console.log("Last 5 seats:", result.data?.slice(-5));
        console.log(
            "All seat codes:",
            result.data?.map((s: any) => s.seat_code).join(", ")
        );

        if (result.success) {
            allSeats.value = result.data;
            // Update occupied seats
            occupiedSeats.value = result.data
                .filter((seat: any) => seat.is_booked)
                .map((seat: any) => seat.seat_code);
        }
    } catch (err) {
        console.error("Error fetching seats:", err);
    }
};

// Fetch showtimes
const showtimeList = ref<any[]>([]);
const fetchShowtimes = async (movieId: number) => {
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

// Pricing
const TICKET_PRICE = 45000;
const adminFee = 5000;

const subtotal = computed(() => selectedSeats.value.length * TICKET_PRICE);
const total = computed(() => subtotal.value + adminFee);

const sortedSeats = computed(() => [...selectedSeats.value].sort());

const toggleSeat = (seatId: string) => {
    if (occupiedSeats.value.includes(seatId)) return;

    if (selectedSeats.value.includes(seatId)) {
        selectedSeats.value = selectedSeats.value.filter((s) => s !== seatId);
    } else {
        selectedSeats.value = [...selectedSeats.value, seatId];
    }
};

const getSeatStatus = (seatId: string) => {
    if (occupiedSeats.value.includes(seatId)) return "occupied";
    if (selectedSeats.value.includes(seatId)) return "selected";
    return "available";
};

const getSeatClasses = (seatId: string) => {
    const status = getSeatStatus(seatId);
    const baseClasses =
        "w-8 h-8 rounded-md transition-all duration-200 flex items-center justify-center";

    switch (status) {
        case "occupied":
            return `${baseClasses} bg-gray-400 cursor-not-allowed`;
        case "selected":
            return `${baseClasses} bg-[#143C8C] text-white shadow-md scale-110`;
        default:
            return `${baseClasses} bg-gray-200 hover:bg-gray-300 cursor-pointer`;
    }
};

const formatDate = (dateStr: string | null) => {
    if (!dateStr) return "-";
    const d = new Date(dateStr);
    const options: Intl.DateTimeFormatOptions = {
        weekday: "long",
        year: "numeric",
        month: "long",
        day: "numeric",
    };
    return d.toLocaleDateString("id-ID", options);
};

const formatCurrency = (amount: number) =>
    new Intl.NumberFormat("id-ID", {
        style: "currency",
        currency: "IDR",
        minimumFractionDigits: 0,
    }).format(amount);

const handleConfirm = async () => {
    if (!canConfirm.value || !selectedShowtime.value) return;

    // Check if user is logged in
    if (!token.value || !user.value) {
        const confirmLogin = confirm(
            "Anda harus login terlebih dahulu untuk melakukan pemesanan. Login sekarang?"
        );
        if (confirmLogin) {
            // Save booking data to localStorage for later
            localStorage.setItem(
                "pendingBooking",
                JSON.stringify({
                    movieId: route.params.movieId,
                    showtimeId: selectedShowtime.value.id,
                    selectedSeats: selectedSeats.value,
                })
            );
            router.push("/login");
        }
        return;
    }

    // Debug
    console.log("=== DEBUG handleConfirm ===");
    console.log("selectedSeats:", selectedSeats.value);
    console.log("allSeats length:", allSeats.value.length);
    console.log("allSeats sample:", allSeats.value.slice(0, 3));
    console.log("token:", token.value ? "exists" : "missing");
    console.log("user:", user.value);

    if (allSeats.value.length === 0) {
        alert(
            `Debug: Data kursi kosong. Showtime ID: ${selectedShowtime.value.id}`
        );
        // Try to fetch again?
        await fetchSeats(selectedShowtime.value.id);
        if (allSeats.value.length === 0) {
            alert("Gagal memuat data kursi. Silakan refresh halaman.");
            return;
        }
    }

    // Map selected seat codes to IDs
    const seatIds = selectedSeats.value
        .map((code) => {
            const seat = allSeats.value.find((s) => s.seat_code === code);
            console.log(
                `Mapping ${code}:`,
                seat ? `found ID ${seat.id}` : "NOT FOUND"
            );
            return seat ? seat.id : null;
        })
        .filter((id) => id !== null);

    console.log("Mapped seatIds:", seatIds);

    if (seatIds.length === 0) {
        alert(
            `Gagal mendapatkan ID kursi. Selected: ${selectedSeats.value.join(
                ", "
            )}. AllSeats: ${
                allSeats.value.length
            }\n\nPeriksa console (F12) untuk detail.`
        );
        return;
    }

    const payload = {
        user_id: user.value?.id || 1, // Use logged-in user ID or fallback to 1
        showtime_id: selectedShowtime.value.id,
        seat_ids: seatIds,
    };

    console.log("Payload:", payload);

    try {
        isLoading.value = true;

        // Prepare headers
        const headers: Record<string, string> = {
            "Content-Type": "application/json",
        };

        // Add Authorization header if token exists
        if (token.value) {
            headers["Authorization"] = `Bearer ${token.value}`;
        }

        const response = await fetch("http://127.0.0.1:3000/api/bookings", {
            method: "POST",
            headers: headers,
            body: JSON.stringify(payload),
        });

        const result = await response.json();
        console.log("Response:", result);

        if (response.ok && result.success) {
            alert("Pemesanan berhasil!");
            // Reset selection or navigate away
            selectedSeats.value = [];
            router.push("/");
        } else {
            alert(result.message || "Gagal membuat pemesanan");
        }
    } catch (err) {
        console.error("Error creating booking:", err);
        alert("Terjadi kesalahan saat memproses pemesanan");
    } finally {
        isLoading.value = false;
    }
};

const canConfirm = computed(
    () =>
        !!selectedMovie.value.id &&
        !!selectedShowtime.value &&
        selectedSeats.value.length > 0
);
</script>
