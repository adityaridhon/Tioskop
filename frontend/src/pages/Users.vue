<template>
  <div class="min-h-screen bg-gray-100">
    <SidebarAdmin />
    
    <!-- Main Content -->
    <div class="lg:ml-64">
      <UsersComponent />
    </div>
  </div>
</template>

<script setup>
import SidebarAdmin from '../components/SidebarAdmin.vue';
import UsersComponent from '../components/users.vue';
</script>
