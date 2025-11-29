<script setup>
import { ref } from 'vue';

// Search query
const searchQuery = ref('');

// Sample user data
const users = ref([
  { id: 1, name: 'Budi Santoso', email: 'budi@example.com', phone: '+62 812 3456 7890', joinDate: '15 Juni 2023' },
  { id: 2, name: 'Siti Nurhaliza', email: 'siti@example.com', phone: '+62 813 2345 6789', joinDate: '22 Mei 2023' },
  { id: 3, name: 'Andi Wijaya', email: 'andi@example.com', phone: '+62 821 9876 5432', joinDate: '3 Juli 2023' },
  { id: 4, name: 'Dewi Lestari', email: 'dewi@example.com', phone: '+62 856 1234 5678', joinDate: '18 Agustus 2023' },
  { id: 5, name: 'Rijal Verdiansyah', email: 'rijal@example.com', phone: '+62 877 9988 7766', joinDate: '5 April 2023' },
  { id: 6, name: 'Nia Ramadhani', email: 'nia@example.com', phone: '+62 858 5544 3322', joinDate: '12 Maret 2023' },
  { id: 7, name: 'Eko Prasetyo', email: 'eko@example.com', phone: '+62 822 1122 3344', joinDate: '28 Februari 2023' },
  { id: 8, name: 'Maya Sari', email: 'maya@example.com', phone: '+62 811 6677 8899', joinDate: '7 Januari 2023' },
  { id: 9, name: 'Hendra Gunawan', email: 'hendra@example.com', phone: '+62 819 4455 6677', joinDate: '19 September 2023' },
  { id: 10, name: 'Fitri Handayani', email: 'fitri@example.com', phone: '+62 857 3366 9988', joinDate: '14 Oktober 2023' },
]);

// Computed filtered users
const filteredUsers = () => {
  if (!searchQuery.value) return users.value;
  const query = searchQuery.value.toLowerCase();
  return users.value.filter(user => 
    user.name.toLowerCase().includes(query) ||
    user.email.toLowerCase().includes(query) ||
    user.phone.includes(query)
  );
};
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
                Nomor Telepon
              </th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Tanggal Bergabung
              </th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Aksi
              </th>
            </tr>
          </thead>
          <tbody class="bg-white divide-y divide-gray-200">
            <tr v-for="user in filteredUsers()" :key="user.id" class="hover:bg-gray-50 transition-colors">
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
                <div class="flex items-center text-sm text-gray-600">
                  <i class="bx bx-phone text-lg mr-2"></i>
                  {{ user.phone }}
                </div>
              </td>
              <td class="px-6 py-4 whitespace-nowrap">
                <div class="flex items-center text-sm text-gray-600">
                  <i class="bx bx-calendar text-lg mr-2"></i>
                  {{ user.joinDate }}
                </div>
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm">
                <button class="text-blue-600 hover:text-blue-800 font-medium">
                  Edit
                </button>
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
          <button class="px-3 py-1.5 text-sm bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors">
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
