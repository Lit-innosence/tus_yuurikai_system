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

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <Suspense fallback={<Loading />}>
      <Routes>
        <Route path='/' element={<Toppage/>}/>
        <Route path='*' element={<Page404/>}/>
      </Routes>
      </Suspense>
    </QueryClientProvider>
  );
}

export default App;
