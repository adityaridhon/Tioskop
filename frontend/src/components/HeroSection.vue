<script setup>
import heroImage from '../assets/hero.png'
</script>

<template>
  <section class="relative h-[400px] sm:h-[500px] md:h-[600px] lg:h-[700px] w-full overflow-hidden">
    <!-- Background Image -->
    <div class="absolute inset-0">
      <img 
        :src="heroImage" 
        alt="Hero" 
        class="w-full h-full object-cover object-center"
      >
      <!-- Overlay gradient -->
      <div class="absolute inset-0 bg-linear-to-r from-black/90 via-black/70 to-transparent"></div>
    </div>

    <!-- Content -->
    <div class="relative h-full max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="flex flex-col justify-center h-full max-w-xl md:max-w-2xl lg:max-w-3xl">
        <!-- Promo Badge -->
        <div class="mb-4 sm:mb-6">
          <span class="inline-flex items-center gap-2 bg-red-600 text-white px-4 py-2 sm:px-5 sm:py-2.5 rounded-full font-bold text-xs sm:text-sm md:text-base">
            <svg class="w-4 h-4 sm:w-5 sm:h-5" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M5 2a1 1 0 011 1v1h1a1 1 0 010 2H6v1a1 1 0 01-2 0V6H3a1 1 0 010-2h1V3a1 1 0 011-1zm0 10a1 1 0 011 1v1h1a1 1 0 110 2H6v1a1 1 0 11-2 0v-1H3a1 1 0 110-2h1v-1a1 1 0 011-1zM12 2a1 1 0 01.967.744L14.146 7.2 17.5 9.134a1 1 0 010 1.732l-3.354 1.935-1.18 4.455a1 1 0 01-1.933 0L9.854 12.8 6.5 10.866a1 1 0 010-1.732l3.354-1.935 1.18-4.455A1 1 0 0112 2z" clip-rule="evenodd"/>
            </svg>
            PROMO SPESIAL
          </span>
        </div>

        <!-- Main Heading -->
        <h1 class="text-2xl sm:text-3xl md:text-5xl lg:text-6xl font-bold text-white mb-2 sm:mb-3 leading-tight">
          Diskon 50%
        </h1>
        <h2 class="text-2xl sm:text-3xl md:text-5xl lg:text-6xl font-bold text-yellow-400 mb-4 sm:mb-6 leading-tight">
          Setiap Rabu
        </h2>

        <!-- Subheading -->
        <p class="text-sm sm:text-base md:text-lg text-gray-200 mb-6 sm:mb-8 max-w-xl lg:max-w-2xl">
          Nikmati film favorit dengan harga spesial! Berlaku untuk semua film dan semua bioskop di Balikpapan.
        </p>

        <!-- CTA Button - Minimalist -->
        <div class="flex flex-col sm:flex-row gap-4">
          <a 
            href="#" 
            class="group inline-flex items-center justify-center sm:justify-start gap-2 text-white font-semibold text-base sm:text-lg hover:gap-3 transition-all duration-300"
          >
            Lihat Bioskop
            <svg class="w-5 h-5 sm:w-6 sm:h-6 transition-transform duration-300 group-hover:translate-x-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8l4 4m0 0l-4 4m4-4H3"/>
            </svg>
          </a>
        </div>
      </div>
    </div>
  </section>
</template>

<style scoped>
</style>