<script setup>
import { useRoute, useRouter } from 'vue-router'
import { getCinemaBySlug } from '../data/cinemas'

const route = useRoute()
const router = useRouter()

const slug = route.params.slug
const cinema = getCinemaBySlug(slug)

// kalau slug tidak ditemukan, balik ke home (sementara)
if (!cinema) {
  router.replace('/')
}
</script>

<template>
  <main class="min-h-screen bg-gray-50">
    <div class="max-w-5xl mx-auto px-4 sm:px-6 lg:px-8 py-8 sm:py-10 lg:py-12">
      <!-- Back -->
      <button
        type="button"
        class="mb-4 inline-flex items-center text-sm text-gray-500 hover:text-gray-800"
        @click="$router.back()"
      >
        ← Kembali
      </button>

      <!-- Header Cinema -->
      <header class="mb-6 sm:mb-8">
        <h1 class="text-xl sm:text-2xl md:text-3xl font-bold text-gray-900">
          {{ cinema.name }}
        </h1>
        <p class="mt-1 text-sm sm:text-base text-gray-600">
          {{ cinema.address }}, {{ cinema.city }}
        </p>
      </header>

      <!-- Jadwal per hari -->
      <section
        v-for="day in cinema.schedules"
        :key="day.date + day.dayLabel"
        class="mb-6 sm:mb-8 lg:mb-10"
      >
        <!-- Header Hari -->
        <div class="mb-3 sm:mb-4">
          <p class="text-xs font-semibold tracking-wide text-gray-500 uppercase">
            {{ day.dayLabel }}
          </p>
          <p class="text-sm sm:text-base text-gray-800">
            {{ day.date }}
          </p>
        </div>

        <!-- List Film -->
        <div class="space-y-4 sm:space-y-5 lg:space-y-6">
          <article
            v-for="movie in day.movies"
            :key="movie.id"
            class="rounded-2xl border border-gray-200 bg-white shadow-sm
                   p-4 sm:p-5 lg:p-6 flex flex-col gap-4 sm:flex-row"
          >
            <!-- Poster -->
            <div class="shrink-0 mx-auto sm:mx-0">
              <div class="w-28 h-40 sm:w-32 sm:h-44 md:w-36 md:h-52 rounded-xl overflow-hidden bg-gray-200">
                <img
                  :src="movie.poster"
                  :alt="movie.title"
                  class="w-full h-full object-cover"
                >
              </div>
            </div>

            <!-- Info + Jam Tayang -->
            <div class="flex-1 flex flex-col gap-3">
              <!-- Info Film -->
              <div>
                <h2 class="text-sm sm:text-base md:text-lg font-bold text-gray-900 mb-1">
                  {{ movie.title }}
                </h2>

                <dl class="grid grid-cols-2 gap-x-4 gap-y-1 text-[11px] sm:text-xs text-gray-600 max-w-md">
                  <div>
                    <dt class="text-gray-400">Genre</dt>
                    <dd>{{ movie.genre }}</dd>
                  </div>
                  <div>
                    <dt class="text-gray-400">Durasi</dt>
                    <dd>{{ movie.duration }}</dd>
                  </div>
                  <div>
                    <dt class="text-gray-400">Sutradara</dt>
                    <dd>{{ movie.director }}</dd>
                  </div>
                  <div class="flex items-center gap-1 mt-1 sm:mt-0">
                    <span class="text-gray-400">Rating Usia</span>
                    <span
                      class="inline-flex items-center rounded-md border border-gray-300 bg-gray-100
                             px-1.5 py-0.5 text-[10px] font-semibold text-gray-700"
                    >
                      {{ movie.rating }}
                    </span>
                  </div>
                </dl>
              </div>

              <!-- Format + Harga -->
              <div class="flex items-center justify-between gap-3 text-xs sm:text-sm mt-1">
                <span class="font-semibold text-gray-800">
                  {{ movie.format }}
                </span>
                <span class="font-semibold text-gray-900">
                  {{ movie.price }}
                </span>
              </div>

              <!-- Jam Tayang -->
              <div class="mt-1">
                <div class="flex flex-wrap gap-1.5 sm:gap-2">
                  <button
                    v-for="show in movie.showtimes"
                    :key="show.time + (show.label || '')"
                    type="button"
                    class="inline-flex flex-col items-center justify-center rounded-lg border border-gray-200
                           bg-white px-3 py-1.5 sm:px-4 sm:py-2 text-xs sm:text-sm font-medium text-gray-900
                           hover:bg-gray-50 transition-colors"
                  >
                    <span>{{ show.time }}</span>
                  </button>
                </div>
              </div>
            </div>
          </article>
        </div>
      </section>

      <p
        v-if="!cinema.schedules || cinema.schedules.length === 0"
        class="text-sm text-gray-500 italic"
      >
        Jadwal belum tersedia untuk bioskop ini.
      </p>
    </div>
  </main>
</template>
