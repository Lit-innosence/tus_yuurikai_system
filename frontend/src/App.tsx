import { QueryClient, QueryClientProvider } from 'react-query';
import { Suspense, lazy } from 'react';
import { Route, Routes } from 'react-router-dom';
import Loading from './routes/Loading';
import Page404 from './routes/Page404';

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      suspense: true,
    },
  },
});

const Toppage = lazy(() => import('./routes/Toppage'));
const LockerTerms = lazy(() => import('./routes/locker/Terms'));
const LockerForm = lazy(() => import('./routes/locker/Form'));
const LockerConfirm = lazy(() => import('./routes/locker/ConfirmPage'));
const FormComp = lazy(() => import('./routes/locker/FormComp'));
const AuthComp = lazy(() => import('./routes/locker/AuthComp'));
const LockerProcess = lazy(() => import('./routes/locker/Process'));


function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <Suspense fallback={<Loading />}>
      <Routes>
        <Route path='/' element={<Toppage/>}/>
        <Route path='/locker/terms' element={<LockerTerms/>}/>
        <Route path='/locker/form' element={<LockerForm/>}/>
        <Route path='/locker/confirm' element={<LockerConfirm/>}/>
        <Route path='/locker/form/complete' element={<FormComp/>}/>
        <Route path='/locker/auth/complete' element={<AuthComp/>}/>
        <Route path='/locker/process' element={<LockerProcess/>}/>
        <Route path='*' element={<Page404/>}/>
      </Routes>
      </Suspense>
    </QueryClientProvider>
  );
}

export default App;
