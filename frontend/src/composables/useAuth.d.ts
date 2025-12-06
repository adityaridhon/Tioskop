import { Ref } from 'vue';

export interface User {
  id: number;
  name: string;
  email: string;
  role: string;
}

export interface AuthState {
  user: Ref<User | null>;
  token: Ref<string>;
  isLoading: Ref<boolean>;
  error: Ref<string | null>;
  login: (email: string, password: string, remember?: boolean) => Promise<any>;
  register: (name: string, email: string, password: string, role?: string) => Promise<any>;
  fetchProfile: () => Promise<User | null>;
  logout: () => void;
}

export function useAuth(): AuthState;
