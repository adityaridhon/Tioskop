<script setup>
import { ref, computed, onMounted } from 'vue';
import { useAuth } from '@/composables/useAuth';

const dropdowns = ref({
  search: false,
  notifications: false,
  profile: false
});
const showLogoutModal = ref(false);

const activeTab = ref('notifications');

const { user, fetchProfile, logout } = useAuth();

const displayName = computed(() => user.value?.name || 'Admin');
const displayRole = computed(() => {
  const role = user.value?.role || 'Administrator';
  return role.charAt(0).toUpperCase() + role.slice(1).toLowerCase();
});

onMounted(() => {
  if (!user.value) {
    fetchProfile().catch(() => {
      // swallow error here; UI will keep fallback values
    });
  }
});

const toggleDropdown = (name) => {
  Object.keys(dropdowns.value).forEach(key => {
    if (key === name) {
      dropdowns.value[key] = !dropdowns.value[key];
    } else {
      dropdowns.value[key] = false;
    }
  });
};

const openLogoutModal = () => {
  showLogoutModal.value = true;
};

const closeLogoutModal = () => {
  showLogoutModal.value = false;
};

const confirmLogout = () => {
  logout();
  showLogoutModal.value = false;
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
          <div class="p-2 md:block text-left">
            <h2 class="text-sm font-semibold text-gray-800">{{ displayName }}</h2>
            <p class="text-xs text-gray-500 capitalize">{{ displayRole }}</p>
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
            <button type="button" @click="openLogoutModal" class="w-full text-left flex items-center text-[13px] py-1.5 px-4 text-gray-600 hover:text-[#f84525] hover:bg-gray-50">Log Out</button>
          </li>
        </ul>
      </li>
    </ul>
  </div>

  <transition name="fade">
    <div v-if="showLogoutModal" class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 px-4">
      <div class="bg-white rounded-lg shadow-xl w-full max-w-sm p-6 text-center">
        <h3 class="text-lg font-semibold text-gray-900 mb-2">Logout dari akun?</h3>
        <p class="text-sm text-gray-600 mb-6">Pastikan semua pekerjaan telah disimpan sebelum keluar.</p>
        <div class="flex gap-3">
          <button type="button" @click="closeLogoutModal" class="flex-1 py-2 rounded-lg border border-gray-300 text-gray-700 hover:bg-gray-50">Batal</button>
          <button type="button" @click="confirmLogout" class="flex-1 py-2 rounded-lg bg-red-600 text-white hover:bg-red-700">Logout</button>
        </div>
      </div>
    </div>
  </transition>
</template>
