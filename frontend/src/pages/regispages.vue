<template>
    <section class="bg-gray-50">
  <div class="flex flex-col items-center justify-center px-6 py-8 mx-auto md:h-screen lg:py-0">
      <div class="w-full bg-white rounded-lg shadow-lg md:mt-0 sm:max-w-md xl:p-0">
          <div class="p-6 space-y-4 md:space-y-6 sm:p-8">
              <div class="flex justify-center mb-4">
                <div class="relative">
                  <!-- Background Shape -->
                  <div class="absolute inset-0 bg-gradient-to-br from-blue-900 to-blue-950 rounded-2xl transform -skew-x-6 shadow-lg"></div>
                  <!-- Text -->
                  <h1 class="relative text-2xl font-bold px-6 py-2 text-white">
                    TIOSKOP
                  </h1>
                </div>
              </div>
              <div>
                <h1 class="text-3xl font-bold leading-tight tracking-tight text-gray-900 text-center">
                    Create account
                </h1>
              </div>
              <form class="space-y-4 md:space-y-6" @submit.prevent="handleRegister">
                  <div>
                      <label for="name" class="block mb-2 text-sm font-medium text-gray-700">Full name</label>
                      <input type="text" v-model="name" name="name" id="name" class="bg-gray-100 border border-gray-200 text-gray-900 rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5" placeholder="" required="">
                  </div>
                  <div>
                      <label for="email" class="block mb-2 text-sm font-medium text-gray-700">Email address</label>
                      <input type="email" v-model="email" name="email" id="email" class="bg-gray-100 border border-gray-200 text-gray-900 rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5" placeholder="" required="">
                  </div>
                  <div>
                      <label for="password" class="block mb-2 text-sm font-medium text-gray-700">Password</label>
                      <input type="password" v-model="password" name="password" id="password" placeholder="" class="bg-gray-100 border border-gray-200 text-gray-900 rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5" required="">
                  </div>
                  <div>
                      <label for="confirm-password" class="block mb-2 text-sm font-medium text-gray-700">Confirm password</label>
                      <input type="password" v-model="confirmPassword" name="confirm-password" id="confirm-password" placeholder="" class="bg-gray-100 border border-gray-200 text-gray-900 rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5" required="">
                  </div>
                  <div class="flex items-start">
                      <div class="flex items-center h-5">
                        <input id="terms" aria-describedby="terms" type="checkbox" v-model="terms" class="w-4 h-4 border border-gray-300 rounded bg-white focus:ring-3 focus:ring-blue-300" required="">
                      </div>
                      <div class="ml-3 text-sm">
                        <label for="terms" class="font-light text-gray-600">I accept the <a class="font-medium text-blue-600 hover:underline" href="#">Terms and Conditions</a></label>
                      </div>
                  </div>
                  <div v-if="errorMessage" class="text-red-500 text-sm text-center">{{ errorMessage }}</div>
                  <button type="submit" :disabled="isLoading" class="w-full text-white bg-blue-600 hover:bg-blue-700 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center disabled:opacity-50">
                    {{ isLoading ? 'Creating account...' : 'Sign up' }}
                  </button>
                  <p class="text-sm font-light text-gray-600 text-center">
                      Already have an account? <RouterLink to="/login" class="font-medium text-blue-600 hover:underline">Sign in</RouterLink>
                  </p>
              </form>
          </div>
      </div>
  </div>
</section>
</template>

<script setup>
import { ref } from 'vue';
import { RouterLink } from 'vue-router';
import { useAuth } from '@/composables/useAuth';

const { register, isLoading, error } = useAuth();

const name = ref('');
const email = ref('');
const password = ref('');
const confirmPassword = ref('');
const terms = ref(false);
const errorMessage = ref('');

const handleRegister = async () => {
  errorMessage.value = '';

  if (!name.value.trim()) {
    errorMessage.value = 'Name is required';
    return;
  }

  if (password.value !== confirmPassword.value) {
    errorMessage.value = 'Passwords do not match';
    return;
  }

  if (!terms.value) {
    errorMessage.value = 'You must accept the Terms and Conditions';
    return;
  }

  try {
    await register(name.value || email.value.split('@')[0], email.value, password.value, 'customer');
    alert('Registration successful! Please login.');
    // router.push('/login') handled by pages or user action; keep simple
    window.location.href = '/login';
  } catch (err) {
    errorMessage.value = err?.message || error.value || 'Registrasi gagal';
  }
};
</script>
