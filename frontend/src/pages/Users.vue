<template>
  <div class="min-h-screen bg-gray-100">
    <SidebarAdmin />
    
    <div class="lg:ml-64">
      <UsersComponent />
    </div>
  </div>
</template>

<script setup>
import SidebarAdmin from '../admin/SidebarAdmin.vue';
import UsersComponent from '../admin/users.vue';
</script>
