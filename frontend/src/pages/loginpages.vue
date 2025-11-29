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
                <p class="text-sm text-gray-400 mb-1">Please enter your details</p>
                <h1 class="text-3xl font-bold leading-tight tracking-tight text-gray-900">
                    Welcome back
                </h1>
              </div>
              <form class="space-y-4 md:space-y-6" action="#">
                  <div>
                      <label for="email" class="block mb-2 text-sm font-medium text-gray-700">Email address</label>
                      <input type="email" name="email" id="email" class="bg-gray-100 border border-gray-200 text-gray-900 rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5" placeholder="" required="">
                  </div>
                  <div>
                      <label for="password" class="block mb-2 text-sm font-medium text-gray-700">Password</label>
                      <input type="password" name="password" id="password" placeholder="" class="bg-gray-100 border border-gray-200 text-gray-900 rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5" required="">
                  </div>
                  <div class="flex items-center justify-between">
                      <div class="flex items-start">
                          <div class="flex items-center h-5">
                            <input id="remember" aria-describedby="remember" type="checkbox" class="w-4 h-4 border border-gray-300 rounded bg-white focus:ring-3 focus:ring-blue-300">
                          </div>
                          <div class="ml-3 text-sm">
                            <label for="remember" class="text-gray-600">Remember for 30 days</label>
                          </div>
                      </div>
                      <a href="#" class="text-sm font-medium text-blue-600 hover:underline">Forgot password</a>
                  </div>
                  <button type="submit" class="w-full text-white bg-blue-600 hover:bg-blue-700 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center">Sign up</button>
                  <p class="text-sm font-light text-gray-600 text-center">
                      Don't have an account? <RouterLink to="/register" class="font-medium text-blue-600 hover:underline">Sign up</RouterLink>
                  </p>
              </form>
          </div>
      </div>
  </div>
</section>
</template>

<script setup>
import { RouterLink } from 'vue-router';
</script>
