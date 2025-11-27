<script setup>
import { ref } from 'vue';

const dropdowns = ref({
  search: false,
  notifications: false,
  profile: false
});

const activeTab = ref('notifications');

const toggleDropdown = (name) => {
  Object.keys(dropdowns.value).forEach(key => {
    if (key === name) {
      dropdowns.value[key] = !dropdowns.value[key];
    } else {
      dropdowns.value[key] = false;
    }
  });
};

const toggleSidebar = () => {
  const main = document.querySelector('.main');
  const overlay = document.querySelector('.sidebar-overlay');
  const menu = document.querySelector('.sidebar-menu');
  
  main?.classList.toggle('active');
  overlay?.classList.toggle('hidden');
  menu?.classList.toggle('-translate-x-full');
};

const toggleFullscreen = () => {
  if (document.fullscreenElement) {
    document.exitFullscreen();
  } else {
    document.documentElement.requestFullscreen();
  }
};
</script>

<template>
  <div class="py-2 px-6 bg-[#f8f4f3] flex items-center shadow-md shadow-black/5 sticky top-0 left-0 z-30">
    <button type="button" @click="toggleSidebar" class="text-lg text-gray-900 font-semibold sidebar-toggle">
      <i class="ri-menu-line"></i>
    </button>

    <ul class="ml-auto flex items-center">
      <li class="dropdown ml-3 relative">
        <button type="button" @click="toggleDropdown('profile')" class="flex items-center">
          <div class="flex-shrink-0 w-10 h-10 relative">
            <div class="p-1 bg-white rounded-full focus:outline-none focus:ring">
              <img class="w-8 h-8 rounded-full" src="https://avatars.githubusercontent.com/u/129702461?v=4" alt=""/>
            </div>
          </div>
          <div class="p-2 md:block text-left">
            <h2 class="text-sm font-semibold text-gray-800">Muhammad Faishal</h2>
            <p class="text-xs text-gray-500">Administrator</p>
          </div>
        </button>
        <ul v-show="dropdowns.profile" class="absolute right-0 shadow-md shadow-black/5 z-30 py-1.5 rounded-md bg-white border border-gray-100 w-full max-w-[140px]">
          <li>
            <a href="#" class="flex items-center text-[13px] py-1.5 px-4 text-gray-600 hover:text-[#f84525] hover:bg-gray-50">Profile</a>
          </li>
          <li>
            <a href="#" class="flex items-center text-[13px] py-1.5 px-4 text-gray-600 hover:text-[#f84525] hover:bg-gray-50">Settings</a>
          </li>
          <li>
            <a href="#" class="flex items-center text-[13px] py-1.5 px-4 text-gray-600 hover:text-[#f84525] hover:bg-gray-50 cursor-pointer">Log Out</a>
          </li>
        </ul>
      </li>
    </ul>
  </div>
</template>
