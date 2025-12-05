<template>
    <div class="min-h-screen bg-gray-50">
        <Navbar />

        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
            <h1 class="text-3xl md:text-4xl font-bold text-gray-900 mb-8">
                Bioskop di Balikpapan
            </h1>

            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                <!-- E-WALK XXI -->
                <div
                    class="bg-white rounded-2xl shadow-md overflow-hidden hover:shadow-xl transition-shadow duration-300"
                >
                    <div
                        class="aspect-video bg-gradient-to-br from-blue-100 to-blue-200 flex items-center justify-center"
                    >
                        <svg
                            class="w-20 h-20 text-blue-300"
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M7 4v16M17 4v16M3 8h4m10 0h4M3 12h18M3 16h4m10 0h4M4 20h16a1 1 0 001-1V5a1 1 0 00-1-1H4a1 1 0 00-1 1v14a1 1 0 001 1z"
                            />
                        </svg>
                    </div>
                    <div class="p-6">
                        <h2 class="text-xl font-bold text-gray-900 mb-2">
                            E-WALK XXI
                        </h2>
                        <p class="text-gray-600 text-sm mb-1">
                            E-Walk Balikpapan Superblock
                        </p>
                        <p class="text-gray-500 text-sm mb-4">Balikpapan</p>
                        <router-link
                            to="/cinema/e-walk-xxi"
                            class="inline-block px-6 py-2 border-2 border-blue-600 text-blue-600 rounded-lg hover:bg-blue-600 hover:text-white transition-colors duration-300 font-medium"
                        >
                            Lihat jadwal
                        </router-link>
                    </div>
                </div>

                <!-- CGV Plaza Balikpapan -->
                <div
                    class="bg-white rounded-2xl shadow-md overflow-hidden hover:shadow-xl transition-shadow duration-300"
                >
                    <div
                        class="aspect-video bg-gradient-to-br from-red-100 to-red-200 flex items-center justify-center"
                    >
                        <svg
                            class="w-20 h-20 text-red-300"
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M7 4v16M17 4v16M3 8h4m10 0h4M3 12h18M3 16h4m10 0h4M4 20h16a1 1 0 001-1V5a1 1 0 00-1-1H4a1 1 0 00-1 1v14a1 1 0 001 1z"
                            />
                        </svg>
                    </div>
                    <div class="p-6">
                        <h2 class="text-xl font-bold text-gray-900 mb-2">
                            CGV Plaza Balikpapan
                        </h2>
                        <p class="text-gray-600 text-sm mb-1">
                            Plaza Balikpapan, Jl. Jenderal Sudirman
                        </p>
                        <p class="text-gray-500 text-sm mb-4">Balikpapan</p>
                        <router-link
                            to="/cinema/cgv-plaza"
                            class="inline-block px-6 py-2 border-2 border-blue-600 text-blue-600 rounded-lg hover:bg-blue-600 hover:text-white transition-colors duration-300 font-medium"
                        >
                            Lihat jadwal
                        </router-link>
                    </div>
                </div>

                <!-- Cinepolis Living Plaza Balikpapan -->
                <div
                    class="bg-white rounded-2xl shadow-md overflow-hidden hover:shadow-xl transition-shadow duration-300"
                >
                    <div
                        class="aspect-video bg-gradient-to-br from-purple-100 to-purple-200 flex items-center justify-center"
                    >
                        <svg
                            class="w-20 h-20 text-purple-300"
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M7 4v16M17 4v16M3 8h4m10 0h4M3 12h18M3 16h4m10 0h4M4 20h16a1 1 0 001-1V5a1 1 0 00-1-1H4a1 1 0 00-1 1v14a1 1 0 001 1z"
                            />
                        </svg>
                    </div>
                    <div class="p-6">
                        <h2 class="text-xl font-bold text-gray-900 mb-2">
                            Cinepolis Living Plaza Balikpapan
                        </h2>
                        <p class="text-gray-600 text-sm mb-1">
                            Living Plaza Balikpapan, Jl. MT Haryono
                        </p>
                        <p class="text-gray-500 text-sm mb-4">Balikpapan</p>
                        <router-link
                            to="/cinema/cinepolis-living"
                            class="inline-block px-6 py-2 border-2 border-blue-600 text-blue-600 rounded-lg hover:bg-blue-600 hover:text-white transition-colors duration-300 font-medium"
                        >
                            Lihat jadwal
                        </router-link>
                    </div>
                </div>

                <!-- PENTACITY XXI -->
                <div
                    class="bg-white rounded-2xl shadow-md overflow-hidden hover:shadow-xl transition-shadow duration-300"
                >
                    <div
                        class="aspect-video bg-gradient-to-br from-green-100 to-green-200 flex items-center justify-center"
                    >
                        <svg
                            class="w-20 h-20 text-green-300"
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M7 4v16M17 4v16M3 8h4m10 0h4M3 12h18M3 16h4m10 0h4M4 20h16a1 1 0 001-1V5a1 1 0 00-1-1H4a1 1 0 00-1 1v14a1 1 0 001 1z"
                            />
                        </svg>
                    </div>
                    <div class="p-6">
                        <h2 class="text-xl font-bold text-gray-900 mb-2">
                            PENTACITY XXI
                        </h2>
                        <p class="text-gray-600 text-sm mb-1">
                            Pentacity Mall Balikpapan
                        </p>
                        <p class="text-gray-500 text-sm mb-4">Balikpapan</p>
                        <router-link
                            to="/cinema/pentacity-xxi"
                            class="inline-block px-6 py-2 border-2 border-blue-600 text-blue-600 rounded-lg hover:bg-blue-600 hover:text-white transition-colors duration-300 font-medium"
                        >
                            Lihat jadwal
                        </router-link>
                    </div>
                </div>

                <!-- STUDIO XXI -->
                <div
                    class="bg-white rounded-2xl shadow-md overflow-hidden hover:shadow-xl transition-shadow duration-300"
                >
                    <div
                        class="aspect-video bg-gradient-to-br from-yellow-100 to-yellow-200 flex items-center justify-center"
                    >
                        <svg
                            class="w-20 h-20 text-yellow-300"
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M7 4v16M17 4v16M3 8h4m10 0h4M3 12h18M3 16h4m10 0h4M4 20h16a1 1 0 001-1V5a1 1 0 00-1-1H4a1 1 0 00-1 1v14a1 1 0 001 1z"
                            />
                        </svg>
                    </div>
                    <div class="p-6">
                        <h2 class="text-xl font-bold text-gray-900 mb-2">
                            STUDIO XXI
                        </h2>
                        <p class="text-gray-600 text-sm mb-1">
                            Jl. Jenderal Sudirman
                        </p>
                        <p class="text-gray-500 text-sm mb-4">Balikpapan</p>
                        <router-link
                            to="/cinema/studio-xxi"
                            class="inline-block px-6 py-2 border-2 border-blue-600 text-blue-600 rounded-lg hover:bg-blue-600 hover:text-white transition-colors duration-300 font-medium"
                        >
                            Lihat jadwal
                        </router-link>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup>
import Navbar from "@/components/Navbar.vue";
</script>
