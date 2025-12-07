<script setup>
import { ref, onMounted, computed } from "vue";
import { useRouter } from "vue-router";

const router = useRouter();

// ===== State Management =====
const isDragging = ref(false);
const startX = ref(0);
const scrollLeft = ref(0);
const containerWidth = ref(0);
const movies = ref([]);
const isLoading = ref(true);
const error = ref(null);

// Placeholder image as data URL
const placeholderImage =
    'data:image/svg+xml,%3Csvg xmlns="http://www.w3.org/2000/svg" width="300" height="450" viewBox="0 0 300 450"%3E%3Crect fill="%23e5e7eb" width="300" height="450"/%3E%3Cg fill="%239ca3af"%3E%3Cpath d="M150 165c-24.853 0-45 20.147-45 45s20.147 45 45 45 45-20.147 45-45-20.147-45-45-45zm0 75c-16.569 0-30-13.431-30-30s13.431-30 30-30 30 13.431 30 30-13.431 30-30 30z"/%3E%3Cpath d="M240 135H60c-8.284 0-15 6.716-15 15v150c0 8.284 6.716 15 15 15h180c8.284 0 15-6.716 15-15V150c0-8.284-6.716-15-15-15zm0 165H60V150h180v150z"/%3E%3C/g%3E%3C/svg%3E';

// ===== Fetch Movies =====
const fetchMovies = async () => {
    try {
        isLoading.value = true;
        const response = await fetch("http://127.0.0.1:3000/api/movies");
        if (!response.ok) throw new Error("Gagal mengambil data film");

        const result = await response.json();
        // Map API data to component format
        movies.value = (result.data || []).map((movie) => ({
            ...movie,
            image: movie.poster_url || placeholderImage,
        }));
    } catch (err) {
        console.error("Error fetching movies:", err);
        error.value = err.message;
    } finally {
        isLoading.value = false;
    }
};

const infiniteMovies = computed(() => {
    if (movies.value.length === 0) return [];
    // Duplicate movies to create infinite scroll effect
    return [...movies.value, ...movies.value, ...movies.value];
});

// ===== Refs =====
const carouselRef = ref(null);
const containerRef = ref(null);
let scrollTimeout = null;

// ===== Navigation Function =====
const navigateToBooking = (movieId) => {
    if (!isDragging.value) {
        router.push({ name: "PemesananFilm", params: { movieId } });
    }
};

// ===== Computed Properties =====
const cardWidth = computed(() => {
    if (!containerWidth.value) return 160;
    const availableWidth = containerWidth.value;

    // Mobile: 2 cards, Tablet: 3 cards, Desktop: 4 cards
    if (availableWidth < 640) {
        const gap = 16;
        return (availableWidth - gap) / 2 - 8;
    } else if (availableWidth < 1024) {
        const gap = 20;
        return (availableWidth - 2 * gap) / 3;
    } else {
        const gap = 24;
        return (availableWidth - 3 * gap) / 4;
    }
});

const gapSize = computed(() => {
    if (!containerWidth.value) return 16;
    if (containerWidth.value < 640) return 16;
    if (containerWidth.value < 1024) return 20;
    return 24;
});

// ===== Mouse Drag Handlers =====
const handleMouseDown = (e) => {
    isDragging.value = true;
    startX.value = e.pageX - carouselRef.value.offsetLeft;
    scrollLeft.value = carouselRef.value.scrollLeft;
    carouselRef.value.style.cursor = "grabbing";
};

const handleMouseMove = (e) => {
    if (!isDragging.value) return;
    e.preventDefault();
    const x = e.pageX - carouselRef.value.offsetLeft;
    const walk = x - startX.value;
    carouselRef.value.scrollLeft = scrollLeft.value - walk;
};

const handleMouseUp = () => {
    if (!isDragging.value) return;
    isDragging.value = false;
    carouselRef.value.style.cursor = "grab";

    setTimeout(() => {
        snapToNearestCard();
    }, 50);
};

const handleMouseLeave = () => {
    if (isDragging.value) {
        isDragging.value = false;
        carouselRef.value.style.cursor = "grab";

        setTimeout(() => {
            snapToNearestCard();
        }, 50);
    }
};

// ===== Snap & Scroll Functions =====
const snapToNearestCard = () => {
    if (!carouselRef.value || isDragging.value) return;

    const currentScroll = carouselRef.value.scrollLeft;
    const totalCardWidth = cardWidth.value + gapSize.value;

    const nearestCardIndex = Math.round(currentScroll / totalCardWidth);
    const snapPosition = nearestCardIndex * totalCardWidth;

    if (Math.abs(currentScroll - snapPosition) > 1) {
        carouselRef.value.scrollTo({
            left: snapPosition,
            behavior: "smooth",
        });
    }
};

const handleScroll = () => {
    if (scrollTimeout) {
        clearTimeout(scrollTimeout);
    }

    scrollTimeout = setTimeout(() => {
        if (!isDragging.value) {
            snapToNearestCard();
        }
        checkInfiniteScroll();
    }, 150);
};

const checkInfiniteScroll = () => {
    if (!carouselRef.value || isDragging.value || movies.value.length === 0)
        return;

    const currentScroll = carouselRef.value.scrollLeft;
    const totalCardWidth = cardWidth.value + gapSize.value;
    const oneSetWidth = movies.value.length * totalCardWidth;

    if (currentScroll >= oneSetWidth * 2 - totalCardWidth) {
        const offset = currentScroll - oneSetWidth;
        carouselRef.value.scrollLeft = offset;
    }

    if (currentScroll <= totalCardWidth) {
        const offset = currentScroll + oneSetWidth;
        carouselRef.value.scrollLeft = offset;
    }
};

// ===== Navigation Handlers =====
const nextSlide = () => {
    if (carouselRef.value) {
        const currentScroll = carouselRef.value.scrollLeft;
        const totalCardWidth = cardWidth.value + gapSize.value;
        carouselRef.value.scrollTo({
            left: currentScroll + totalCardWidth,
            behavior: "smooth",
        });

        setTimeout(() => {
            checkInfiniteScroll();
        }, 300);
    }
};

const prevSlide = () => {
    if (carouselRef.value) {
        const currentScroll = carouselRef.value.scrollLeft;
        const totalCardWidth = cardWidth.value + gapSize.value;
        carouselRef.value.scrollTo({
            left: currentScroll - totalCardWidth,
            behavior: "smooth",
        });

        setTimeout(() => {
            checkInfiniteScroll();
        }, 300);
    }
};

// ===== Utility Functions =====
const updateContainerWidth = () => {
    if (containerRef.value) {
        containerWidth.value = containerRef.value.offsetWidth;
    }
};

// ===== Lifecycle Hooks =====
onMounted(async () => {
    await fetchMovies();
    updateContainerWidth();
    window.addEventListener("resize", updateContainerWidth);

    if (carouselRef.value && movies.value.length > 0) {
        const totalCardWidth = cardWidth.value + gapSize.value;
        const oneSetWidth = movies.value.length * totalCardWidth;
        carouselRef.value.scrollLeft = oneSetWidth;

        carouselRef.value.addEventListener("scroll", handleScroll, {
            passive: true,
        });
    }
});
</script>

<template>
    <!-- ===== Now Showing Section ===== -->
    <section id="now-showing-section" class="py-12 sm:py-16 md:py-20 bg-white">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <!-- ===== Section Header ===== -->
            <div class="mb-8 sm:mb-10 md:mb-12">
                <h2
                    class="text-2xl sm:text-3xl md:text-4xl font-bold text-gray-900"
                >
                    Now Showing
                </h2>
            </div>

            <!-- ===== Loading State ===== -->
            <div v-if="isLoading" class="text-center py-12">
                <div
                    class="inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-blue-900"
                ></div>
                <p class="mt-4 text-gray-600">Memuat film...</p>
            </div>

            <!-- ===== Error State ===== -->
            <div v-else-if="error" class="text-center py-12 text-red-500">
                {{ error }}
            </div>

            <!-- ===== Carousel Container ===== -->
            <div v-else ref="containerRef" class="relative">
                <!-- ===== Scrollable Movie Grid ===== -->
                <div
                    ref="carouselRef"
                    @mousedown="handleMouseDown"
                    @mousemove="handleMouseMove"
                    @mouseup="handleMouseUp"
                    @mouseleave="handleMouseLeave"
                    class="overflow-x-auto scrollbar-hide"
                    style="
                        scrollbar-width: none;
                        -ms-overflow-style: none;
                        cursor: grab;
                    "
                >
                    <div class="flex gap-4 sm:gap-5 lg:gap-6 pb-4">
                        <div
                            v-for="(movie, index) in infiniteMovies"
                            :key="`movie-${index}`"
                            class="shrink-0"
                            :style="{ width: `${cardWidth}px` }"
                        >
                            <div
                                class="group cursor-pointer select-none"
                                @click="navigateToBooking(movie.id)"
                            >
                                <!-- ===== Movie Poster ===== -->
                                <div
                                    class="relative overflow-hidden rounded-xl sm:rounded-2xl mb-2 sm:mb-3 aspect-2/3 bg-gray-200"
                                >
                                    <img
                                        :src="movie.image"
                                        :alt="movie.title"
                                        class="w-full h-full object-cover transform transition-transform duration-500 group-hover:scale-110"
                                        draggable="false"
                                        @error="
                                            (e) =>
                                                (e.target.src =
                                                    '/placeholder.jpg')
                                        "
                                        loading="lazy"
                                    />
                                </div>

                                <!-- ===== Movie Title ===== -->
                                <h3
                                    class="font-bold text-sm sm:text-base text-gray-900 group-hover:text-blue-600 transition-colors duration-300 line-clamp-2"
                                >
                                    {{ movie.title }}
                                </h3>
                            </div>
                        </div>
                    </div>
                </div>

                <!-- ===== Navigation Buttons ===== -->
                <button
                    @click="prevSlide"
                    class="hidden lg:flex group absolute -left-6 top-1/2 -translate-y-1/2 w-12 h-12 bg-white/15 backdrop-blur-xl rounded-full border border-white/20 items-center justify-center transition-all duration-300 hover:bg-white/25 hover:scale-110 z-10 shadow-xl"
                >
                    <svg
                        class="w-6 h-6 text-gray-900 transform transition-transform duration-300 group-hover:-translate-x-0.5"
                        fill="none"
                        stroke="currentColor"
                        viewBox="0 0 24 24"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M15 19l-7-7 7-7"
                        />
                    </svg>
                </button>

                <button
                    @click="nextSlide"
                    class="hidden lg:flex group absolute -right-6 top-1/2 -translate-y-1/2 w-12 h-12 bg-white/15 backdrop-blur-xl rounded-full border border-white/20 items-center justify-center transition-all duration-300 hover:bg-white/25 hover:scale-110 z-10 shadow-xl"
                >
                    <svg
                        class="w-6 h-6 text-gray-900 transform transition-transform duration-300 group-hover:translate-x-0.5"
                        fill="none"
                        stroke="currentColor"
                        viewBox="0 0 24 24"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M9 5l7 7-7 7"
                        />
                    </svg>
                </button>
            </div>
        </div>
    </section>
</template>

<style scoped>
.scrollbar-hide::-webkit-scrollbar {
    display: none;
}

.line-clamp-2 {
    display: -webkit-box;
    -webkit-line-clamp: 2;
    line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
}
</style>
