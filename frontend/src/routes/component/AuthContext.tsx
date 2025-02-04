import { useMutation } from '@tanstack/react-query';
import axios from 'axios';
import { createContext, useContext, useState, useEffect } from 'react';
import constants from '../constants';
import { LoginFormData } from '../types';

type AuthState = {
  loggedIn: boolean;
  handleLogin: (formData: LoginFormData) => Promise<void>;
  isLoading: boolean;
};

const AuthContext = createContext<AuthState>({
  loggedIn: false,
  handleLogin: async () => {},
  isLoading: false,
});

export const useAuth = () => useContext(AuthContext);

type AuthProviderProps = {
  children?: React.ReactNode;
};

const AuthProvider: React.FC<AuthProviderProps> = (props) => {
  const [loggedIn, setLoggedIn] = useState<boolean>(false);

  // axiosの設定: Cookieを有効にして、サーバー間での認証を可能にする
  axios.defaults.withCredentials = true;

  const loginMutation = useMutation<void, Error, LoginFormData>({
    mutationFn: async (formData: LoginFormData) => {
      await axios.post(`${constants.backendApiEndpoint}/api/login`, formData);
    },
    onSuccess: () => {
      setLoggedIn(true);
    },
  });

  const handleLogin = async (formData: LoginFormData): Promise<void> => {
    try {
        await loginMutation.mutateAsync(formData);
    } catch (error: any) {
        if (axios.isAxiosError(error) && error.response) {
            if (error.response.status === 401) {
                throw new Error('ユーザー名またはパスワードが間違っています');
            }
        }
        throw new Error('ログインに失敗しました。もう一度お試しください');
    }
};

  const authState: AuthState = {
    loggedIn,
    handleLogin,
    isLoading: loginMutation.status === 'pending', 
  };

  return <AuthContext.Provider value={authState}>{props.children}</AuthContext.Provider>;
};

export default AuthProvider;
