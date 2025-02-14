import { useMutation } from '@tanstack/react-query';
import axios from 'axios';
import { createContext, useContext, useState, useEffect } from 'react';
import constants from '../constants';
import { LoginFormData } from '../types';
import Cookies from 'js-cookie'; 

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
  const [loggedIn, setLoggedIn] = useState<boolean>(false);

  axios.defaults.withCredentials = true;

  // 初回レンダリング時にCookie内のトークンの有無を確認
  useEffect(() => {
    const token = Cookies.get('token');
    if (token) {
      setLoggedIn(true);
    }
  }, []);

  // ログイン処理
  const loginMutation = useMutation< void , Error, LoginFormData>({
    mutationFn: async (formData: LoginFormData) => {
      await axios.post(`${constants.backendApiEndpoint}/api/login`, formData);
    },
    onSuccess: () => {
      // Cookieにトークンを保存
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

  // ログアウト処理（Cookieからトークン削除）
  const handleLogout = async (): Promise<void> => {
    try {
      await axios.post(`${constants.backendApiEndpoint}/api/logout`, {}, { withCredentials: true });
    } catch (error) {
      console.error('ログアウトAPIの呼び出しに失敗しました', error);
    }
    
    setLoggedIn(false);
  };  

  const authState: AuthState = {
    loggedIn,
    handleLogin,
    handleLogout,
    isLoading: loginMutation.status === 'pending',
  };

  return <AuthContext.Provider value={authState}>{props.children}</AuthContext.Provider>;
};

export default AuthProvider;
