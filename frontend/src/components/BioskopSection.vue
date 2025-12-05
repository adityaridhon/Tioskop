<script setup>
import { cinemas } from "../data/cinemas";
</script>

<template>
    <section id="bioskop-section" class="py-12 sm:py-16 md:py-20 bg-gray-50">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="mb-8 sm:mb-10 md:mb-12">
                <h2
                    class="text-2xl sm:text-3xl md:text-4xl font-bold text-gray-900"
                >
                    Bioskop di Balikpapan
                </h2>
            </div>

            <div
                class="grid gap-5 sm:gap-6 md:gap-7 grid-cols-1 sm:grid-cols-2 lg:grid-cols-3"
            >
                <article
                    v-for="cinema in cinemas"
                    :key="cinema.slug"
                    class="group relative overflow-hidden rounded-2xl bg-white border border-gray-200 shadow-sm hover:shadow-lg hover:border-blue-500/60 transition-all duration-200"
                >
                    <div class="relative h-32 sm:h-36 md:h-40 overflow-hidden">
                        <img
                            :src="cinema.image"
                            :alt="cinema.name"
                            class="h-full w-full object-cover transform group-hover:scale-[1.03] transition-transform duration-300"
                        />
                        <div
                            class="pointer-events-none absolute inset-0 bg-linear-to-t from-black/25 via-black/0"
                        />
                    </div>

                    <div class="p-4 sm:p-5 flex flex-col gap-3">
                        <div>
                            <h3
                                class="text-base sm:text-lg font-semibold text-gray-900 leading-snug"
                            >
                                {{ cinema.name }}
                            </h3>
                            <p class="mt-1 text-xs sm:text-sm text-gray-600">
                                {{ cinema.address }}
                            </p>
                            <p class="text-[11px] sm:text-xs text-gray-500">
                                {{ cinema.city }}
                            </p>
                        </div>

                        <div class="mt-2">
                            <RouterLink
                                :to="{
                                    name: 'CinemaSchedule',
                                    params: { slug: cinema.slug },
                                }"
                                class="inline-flex items-center justify-center px-3 sm:px-4 py-1.5 rounded-full text-xs sm:text-sm font-medium border border-blue-500 text-blue-600 bg-white hover:bg-blue-50 transition-colors duration-200"
                            >
                                Lihat jadwal
                            </RouterLink>
                        </div>
                    </div>
                </article>
            </div>
        </div>
    </section>
</template>
