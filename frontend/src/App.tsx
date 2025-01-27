import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { Suspense, lazy } from 'react';
import { Route, Routes } from 'react-router-dom';
import Loading from './routes/Loading';
import Page404 from './routes/Page404';
import AuthProvider from './routes/component/AuthContext';
import Login from './routes/Login';
import Admin from './routes/admin/Admin';
import PrivateRoute from './routes/component/PrivateRouter';
import LockerMailAuth from './routes/locker/MailAuth';
import CircleRegisterAuth from './routes/circle/RegisterAuth';
import CircleUpdateAuth from './routes/circle/UpdateAuth';

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
const LockerRegister = lazy(() => import('./routes/locker/Register'));
const LockerRegisterConfirm = lazy(() => import('./routes/locker/RegisterConfirm'));
const LockerRegisterComplete = lazy(() => import('./routes/locker/RegisterComp'));

const CircleSelect = lazy(() => import('./routes/circle/CircleSelect'));
const CircleRegister = lazy(() => import('./routes/circle/Status'));
const CircleRegisterAuthComp = lazy(() => import('./routes/circle/AuthComp'));
const CircleRegisterProcess = lazy(() => import('./routes/circle/RegisterProcess'));
const CircleUpdate = lazy(() => import('./routes/circle/Update'));
const CircleUpdateConfirm = lazy(() => import('./routes/circle/UpdateConfirm'));
const CircleUpdateComplete = lazy(() => import('./routes/circle/UpdateComp'));
const CircleUpdateProcess = lazy(() => import('./routes/circle/UpdateProcess'));
const Redirect = lazy(() => import('./routes/component/Redirect'));

const LockerSearch = lazy(() => import('./routes/admin/LockerSearch'));
const AdminLockerSelect = lazy(() => import('./routes/admin/LockerSelect'));
const LockerReset = lazy(() => import('./routes/admin/LockerReset'));
const AdminCircleSelect = lazy(() => import('./routes/admin/CircleSelect'));
const CircleAccessLimit = lazy(() => import('./routes/admin/CircleAccessLimit'));
const CircleList = lazy(() => import('./routes/admin/CircleList'));

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
            <Route path='/locker/user-register' element={<LockerMailAuth />} />
            <Route path='/locker/register' element={<LockerRegister />} />
            <Route path='/locker/register/confirm' element={<LockerRegisterConfirm />} />
            <Route path='/locker/register/complete' element={<LockerRegisterComplete />} />

            <Route path='/circle' element={<CircleSelect />} />
            <Route path='/circle/register/status' element={<CircleRegister />} />
            <Route path='/circle/register/auth' element={<CircleRegisterAuth />} />
            <Route path='/circle/register/complete' element={<CircleRegisterAuthComp />} />
            <Route path='/circle/register/process' element={<CircleRegisterProcess />} />
            <Route path='/circle/update' element={<CircleUpdate />} />
            <Route path='/circle/update/auth' element={<CircleUpdateAuth />} />
            <Route path='/circle/update/confirm' element={<CircleUpdateConfirm />} />
            <Route path='/circle/update/complete' element={<CircleUpdateComplete />} />
            <Route path='/circle/update/process' element={<CircleUpdateProcess />} />
            
            <Route path='/redirect/*' element={<Redirect />} />

            <Route path='/login' element={<Login />} />
            <Route path='/admin' element={<PrivateRoute><Admin /></PrivateRoute>} />
            <Route path='/admin/locker' element={<PrivateRoute><AdminLockerSelect/></PrivateRoute>} />
            <Route path='/admin/circle' element={<PrivateRoute><AdminCircleSelect/></PrivateRoute>} />
            <Route path='/admin/locker/reset' element={<PrivateRoute><LockerReset /></PrivateRoute>} />
            <Route path='/admin/locker/search' element={<PrivateRoute><LockerSearch /></PrivateRoute>} />
            <Route path='/admin/circle/access' element={<PrivateRoute><CircleAccessLimit /></PrivateRoute>} />
            <Route path='/admin/circle/list' element={<PrivateRoute><CircleList /></PrivateRoute>} />

            <Route path='/locker/nopage' element={<Page404 />} />
            <Route path='/circle/nopage' element={<Page404 />} />
            <Route path='*' element={<Page404 />} />
          </Routes>
        </Suspense>
      </AuthProvider>
    </QueryClientProvider>
  );
}

export default App;
