import React from 'react';

type Page404Props = {
    children?: React.ReactNode;
};

const Page404: React.FC<Page404Props> = (props) => {
    return (
        <div className='px-6 py-10 text-center text-neutral-800 dark:bg-neutral-700 dark:text-neutral-200'>
        <h1 className='mb-6 text-5xl font-bold'>404 </h1>
        <h3 className='mb-8 text-3xl font-bold'>Page not found</h3>
        </div>
    );
};

export default Page404;
