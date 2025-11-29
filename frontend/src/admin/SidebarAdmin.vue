<script setup>
import { useRouter } from 'vue-router';

const router = useRouter();
</script>

<template>
  <div class="fixed left-0 top-0 w-64 h-full bg-[#f8f4f3] p-4 z-50 sidebar-menu transition-transform">
    <div class="shrink-0 mb-4">
      <div class="relative">
        <!-- Background Shape -->
        <div class="absolute inset-0 bg-gradient-to-br from-blue-900 to-blue-950 rounded-xl transform -skew-x-6 shadow-lg"></div>
        <!-- Text -->
        <h1 class="relative text-lg font-bold px-4 py-1 text-white text-center">TIOS-MIN</h1>
      </div>
    </div>
    
    <ul class="mt-4">
      <span class="text-gray-400 font-bold">ADMIN</span>
      
      <li class="mb-1 group">
        <router-link 
          to="/admin" 
          class="flex font-semibold items-center py-2 px-4 text-gray-900 hover:bg-gray-950 hover:text-gray-100 rounded-md transition-colors"
          active-class="bg-gray-950 text-gray-100"
          exact
        >
          <i class="ri-home-2-line mr-3 text-lg"></i>
          <span class="text-sm">Dashboard</span>
        </router-link>
      </li>
      
      <li class="mb-1 group">
        <router-link 
          to="/admin/users" 
          class="flex font-semibold items-center py-2 px-4 text-gray-900 hover:bg-gray-950 hover:text-gray-100 rounded-md transition-colors"
          active-class="bg-gray-950 text-gray-100"
        >
          <i class='bx bx-user mr-3 text-lg'></i>                
          <span class="text-sm">Users</span>
        </router-link>
      </li>
                
      <li class="mb-1 group">
        <router-link 
          to="/admin/movies" 
          class="flex font-semibold items-center py-2 px-4 text-gray-900 hover:bg-gray-950 hover:text-gray-100 rounded-md transition-colors"
          active-class="bg-gray-950 text-gray-100"
        >
          <i class='bx bx-movie mr-3 text-lg'></i>                 
          <span class="text-sm">Movies</span>
        </router-link>
      </li>
      
      <li class="mb-1 group">
        <router-link 
          to="/admin/jadwaltayang" 
          class="flex font-semibold items-center py-2 px-4 text-gray-900 hover:bg-gray-950 hover:text-gray-100 rounded-md transition-colors"
          active-class="bg-gray-950 text-gray-100"
        >
          <i class='bx bx-calendar mr-3 text-lg'></i>                
          <span class="text-sm">Schedules</span>
        </router-link>
      </li>
      
    </ul>
  </div>
  
  <div class="fixed top-0 left-0 w-full h-full bg-black/50 z-40 md:hidden sidebar-overlay hidden"></div>
</template>


