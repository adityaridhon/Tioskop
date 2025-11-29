import { ref } from 'vue';
import { useRouter } from 'vue-router';
import { authAPI } from '@/services/api';

const user = ref(JSON.parse(localStorage.getItem('user') || 'null'));
const token = ref(localStorage.getItem('token') || '');
const isLoading = ref(false);
const error = ref(null);

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

  const logout = () => {
    token.value = '';
    user.value = null;
    localStorage.removeItem('token');
    localStorage.removeItem('user');
    sessionStorage.removeItem('token');
    sessionStorage.removeItem('user');
  };

  return {
    user,
    token,
    isLoading,
    error,
    login,
    register,
    logout,
  };
}
