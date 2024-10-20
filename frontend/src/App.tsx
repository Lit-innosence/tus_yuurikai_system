import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { Suspense, lazy } from 'react';
import { Route, Routes } from 'react-router-dom';
import { ReactQueryDevtools } from '@tanstack/react-query-devtools';
import Loading from './routes/Loading';
import Page404 from './routes/Page404';
import AuthProvider from './routes/component/AuthContext';
import Login from './routes/Login';
import Admin from './routes/admin/Admin';
import PrivateRoute from './routes/component/PrivateRouter';
import MailAuth from './routes/locker/MailAuth';

// QueryClientのインスタンスを作成
const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      retry: 1,
      refetchOnWindowFocus: false,
    },
  },
});

const Toppage = lazy(() => import('./routes/Toppage'));
const LockerTerms = lazy(() => import('./routes/locker/Terms'));
const LockerForm = lazy(() => import('./routes/locker/Form'));
const LockerFormConfirm = lazy(() => import('./routes/locker/ConfirmPage'));
const FormComp = lazy(() => import('./routes/locker/FormComp'));
const AuthComp = lazy(() => import('./routes/locker/AuthComp'));
const LockerProcess = lazy(() => import('./routes/locker/Process'));
const LockerSearch = lazy(() => import('./routes/admin/LockerSearch'));
const LockerRegister = lazy(() => import('./routes/locker/Register'));
const LockerRegisterConfirm = lazy(() => import('./routes/locker/RegisterConfirm'));
const LockerRegisterComplete = lazy(() => import('./routes/locker/RegisterComp'));

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <AuthProvider>
        <Suspense fallback={<Loading />}>
          <Routes>
            <Route path='/' element={<Toppage />} />
            <Route path='/locker/terms' element={<LockerTerms />} />
            <Route path='/locker/form' element={<LockerForm />} />
            <Route path='/locker/form/confirm' element={<LockerFormConfirm />} />
            <Route path='/locker/form/complete' element={<FormComp />} />
            <Route path='/locker/auth/complete' element={<AuthComp />} />
            <Route path='/locker/process' element={<LockerProcess />} />
            <Route path='/locker/user-register' element={<MailAuth />} />
            <Route path='/locker/register' element={<LockerRegister />} />
            <Route path='/locker/register/confirm' element={<LockerRegisterConfirm />} />
            <Route path='/locker/register/complete' element={<LockerRegisterComplete />} />
            <Route path='/login' element={<Login />} />
            <Route path='/admin' element={<PrivateRoute><Admin /></PrivateRoute>} />
            <Route path='/admin/locker/search' element={<PrivateRoute><LockerSearch /></PrivateRoute>} />
            <Route path='/locker/nopage' element={<Page404 />} />
            <Route path='*' element={<Page404 />} />
          </Routes>
        </Suspense>
      </AuthProvider>
      {/* React Query Devtools の追加 */}
      <ReactQueryDevtools initialIsOpen={false} />
    </QueryClientProvider>
  );
}

export default App;
