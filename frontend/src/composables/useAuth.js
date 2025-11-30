import { ref } from 'vue';
import { useRouter } from 'vue-router';
import { authAPI } from '@/services/api';

const getStoredToken = () => localStorage.getItem('token') || sessionStorage.getItem('token') || '';
const getStoredUser = () => {
  const serialized = localStorage.getItem('user') || sessionStorage.getItem('user') || 'null';
  try {
    return JSON.parse(serialized);
  } catch (_) {
    return null;
  }
};

const user = ref(getStoredUser());
const token = ref(getStoredToken());
const isLoading = ref(false);
const error = ref(null);

const persistUser = (data) => {
  const serialized = JSON.stringify(data);
  if (localStorage.getItem('token')) {
    localStorage.setItem('user', serialized);
  }
  if (sessionStorage.getItem('token')) {
    sessionStorage.setItem('user', serialized);
  }
};

export function useAuth() {
  const router = useRouter();

  const login = async (email, password, remember = false) => {
    isLoading.value = true;
    error.value = null;
    try {
      const resp = await authAPI.login({ email, password });
      if (!resp.success) {
        throw new Error(resp.message || 'Login gagal');
      }

      const loginData = resp.data; // { user, token }
      if (!loginData) throw new Error('Respons login tidak memiliki data');

      // store
      token.value = loginData.token;
      user.value = loginData.user;

      if (remember) {
        localStorage.setItem('token', token.value);
        localStorage.setItem('user', JSON.stringify(user.value));
      } else {
        // still keep session for page lifetime
        sessionStorage.setItem('token', token.value);
        sessionStorage.setItem('user', JSON.stringify(user.value));
      }

      // Redirect based on role (case-insensitive)
      const role = (user.value && user.value.role) ? user.value.role.toString().toUpperCase() : '';
      if (role === 'ADMIN') {
        router.push('/admin');
      } else {
        router.push('/');
      }

      return resp;
    } catch (err) {
      error.value = err.message || String(err);
      throw err;
    } finally {
      isLoading.value = false;
    }
  };

  const register = async (name, email, password, role = 'customer') => {
    isLoading.value = true;
    error.value = null;
    try {
      const resp = await authAPI.register({ name, email, password, role });
      if (!resp.success) {
        throw new Error(resp.message || 'Registrasi gagal');
      }
      return resp;
    } catch (err) {
      error.value = err.message || String(err);
      throw err;
    } finally {
      isLoading.value = false;
    }
  };

  const fetchProfile = async () => {
    if (!token.value) return null;
    isLoading.value = true;
    error.value = null;
    try {
      const resp = await authAPI.profile();
      if (!resp.success || !resp.data) {
        throw new Error(resp.message || 'Gagal mengambil profile');
      }
      user.value = resp.data;
      persistUser(resp.data);
      return resp.data;
    } catch (err) {
      error.value = err.message || String(err);
      throw err;
    } finally {
      isLoading.value = false;
    }
  };

  const logout = () => {
    token.value = '';
    user.value = null;
    localStorage.removeItem('token');
    localStorage.removeItem('user');
    sessionStorage.removeItem('token');
    sessionStorage.removeItem('user');
    router.push('/login');
  };

  return {
    user,
    token,
    isLoading,
    error,
    login,
    register,
    fetchProfile,
    logout,
  };
}
