import { useMutation } from '@tanstack/react-query';
import axios from 'axios';
import { createContext, useContext, useState, useEffect } from 'react';
import constants from '../constants';
import { LoginFormData } from '../types';

type AuthState = {
  loggedIn: boolean;
  handleLogin: (formData: LoginFormData) => Promise<void>;
  handleLogout: () => void;
  isLoading: boolean;
};

const AuthContext = createContext<AuthState>({
  loggedIn: false,
  handleLogin: async () => {},
  handleLogout: () => {},
  isLoading: false,
});

export const useAuth = () => useContext(AuthContext);

type AuthProviderProps = {
  children?: React.ReactNode;
};

const AuthProvider: React.FC<AuthProviderProps> = (props) => {
  const [loggedIn, setLoggedIn] = useState<boolean>(!!localStorage.getItem('token'));

  const loginMutation = useMutation<void, Error, LoginFormData>({
    mutationFn: async (formData: LoginFormData) => {
      const response = await axios.post(`${constants.backendApiEndpoint}/api/login`, formData);
      localStorage.setItem('token', response.data.token); // JWTトークンを保存
    },
    onSuccess: () => {
      setLoggedIn(true);
    },
  });

  const handleLogin = async (formData: LoginFormData): Promise<void> => {
    try {
      await loginMutation.mutateAsync(formData);
    } catch (error) {
      console.error('Login failed', error);
    }
  };

  const handleLogout = () => {
    localStorage.removeItem('token');
    setLoggedIn(false);
    window.open('/login', '_self');
  };

  useEffect(() => {
    const token = localStorage.getItem('token');
    if (token) {
      axios.defaults.headers.common['Authorization'] = `Bearer ${token}`;
    }
  }, [loggedIn]);

  const authState: AuthState = {
    loggedIn,
    handleLogin,
    handleLogout,
    isLoading: loginMutation.status === 'pending', 
  };

  return <AuthContext.Provider value={authState}>{props.children}</AuthContext.Provider>;
};

export default AuthProvider;
