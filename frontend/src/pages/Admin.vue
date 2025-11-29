<script setup>
import SidebarAdmin from '../components/SidebarAdmin.vue';
import NavbarAdmin from '../components/NavbarAdmin.vue';
import DashboardContent from '../components/DashboardContent.vue';
</script>

<template>
  <div class="min-h-screen bg-gray-200">
    <SidebarAdmin />
    
    <main class="w-full md:w-[calc(100%-256px)] md:ml-64 bg-gray-200 min-h-screen transition-all main">
      <NavbarAdmin />
      <DashboardContent />
    </main>
  </div>
</template>

<style scoped>
@media (min-width: 768px) {
  .main.active {
    margin-left: 0px;
    width: 100%;
  }
}
</style>