<script setup>
import { ref, computed, onMounted } from 'vue';
// TODO: Create useUsers composable when backend endpoint is ready
// import { useUsers } from '@/composables/useApi';

// Search query
const searchQuery = ref('');

// TODO: Replace with API data when backend endpoint is ready
// const { users: usersData, loading, error, fetchAll } = useUsers();

// Sample user data (will be replaced with real data from backend)
const users = ref([
  { id: 1, name: 'admin', email: 'admin@tioskop.com', role: 'admin', created_at: '2025-11-26 10:44:27' },
  { id: 2, name: 'Adit', email: 'adit@gmail.com', role: 'customer', created_at: '2025-11-26 10:46:38' },
]);

// Computed filtered users
const filteredUsers = computed(() => {
  if (!searchQuery.value) return users.value;
  const query = searchQuery.value.toLowerCase();
  return users.value.filter(user => 
    user.name.toLowerCase().includes(query) ||
    user.email.toLowerCase().includes(query)
  );
});

// Format date
const formatDate = (dateString) => {
  const date = new Date(dateString);
  return date.toLocaleDateString('id-ID', { 
    day: 'numeric', 
    month: 'long', 
    year: 'numeric' 
  });
};

// TODO: Add CRUD functions when backend is ready
// onMounted(async () => {
//   await fetchAll();
// });
</script>

<template>
  <div class="p-6">
    <!-- Header -->
    <div class="mb-6">
      <h1 class="text-2xl font-bold text-gray-900 mb-2">Data Pelanggan</h1>
      <p class="text-sm text-gray-500">Daftar semua data pelanggan</p>
    </div>

    <!-- Search Bar -->
    <div class="mb-6">
      <div class="relative max-w-md">
        <i class="bx bx-search absolute left-3 top-1/2 -translate-y-1/2 text-gray-400 text-xl"></i>
        <input 
          v-model="searchQuery"
          type="text" 
          placeholder="Nama Pengguna" 
          class="w-full pl-10 pr-4 py-2.5 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
        >
      </div>
    </div>

    <!-- Table -->
    <div class="bg-white border border-gray-200 rounded-lg overflow-hidden">
      <div class="overflow-x-auto">
        <table class="w-full">
          <thead class="bg-gray-50 border-b border-gray-200">
            <tr>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Nama Pelanggan
              </th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Email
              </th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Role
              </th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Tanggal Bergabung
              </th>
            </tr>
          </thead>
          <tbody class="bg-white divide-y divide-gray-200">
            <tr v-for="user in filteredUsers" :key="user.id" class="hover:bg-gray-50 transition-colors">
              <td class="px-6 py-4 whitespace-nowrap">
                <div class="flex items-center">
                  <i class="bx bx-user-circle text-3xl text-gray-400 mr-3"></i>
                  <span class="text-sm font-medium text-gray-900">{{ user.name }}</span>
                </div>
              </td>
              <td class="px-6 py-4 whitespace-nowrap">
                <div class="flex items-center text-sm text-gray-600">
                  <i class="bx bx-envelope text-lg mr-2"></i>
                  {{ user.email }}
                </div>
              </td>
              <td class="px-6 py-4 whitespace-nowrap">
                <span :class="[
                  'inline-block px-2 py-1 text-xs font-medium rounded-full',
                  user.role === 'admin' ? 'bg-purple-100 text-purple-700' : 'bg-blue-100 text-blue-700'
                ]">
                  {{ user.role }}
                </span>
              </td>
              <td class="px-6 py-4 whitespace-nowrap">
                <div class="flex items-center text-sm text-gray-600">
                  <i class="bx bx-calendar text-lg mr-2"></i>
                  {{ formatDate(user.created_at) }}
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <!-- Pagination -->
      <div class="bg-gray-50 px-6 py-4 border-t border-gray-200 flex items-center justify-between">
        <div class="text-sm text-gray-600">
          Menampilkan 10 dari {{ users.length }} data
        </div>
        <div class="flex items-center gap-2">
          <button class="px-3 py-1.5 text-sm border border-gray-300 rounded hover:bg-gray-100 transition-colors disabled:opacity-50 disabled:cursor-not-allowed">
            Sebelumnya
          </button>
          <button class="px-3 py-1.5 text-sm bg-blue-900 text-white rounded hover:bg-blue-800 transition-colors">
            1
          </button>
          <button class="px-3 py-1.5 text-sm border border-gray-300 rounded hover:bg-gray-100 transition-colors">
            Berikutnya
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
