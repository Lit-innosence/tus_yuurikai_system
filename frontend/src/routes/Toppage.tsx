import React from 'react';

type RootProps = {
    children?: React.ReactNode;
};

const Root: React.FC<RootProps> = (props) => {
    return (
    <div className='container vh-100 js-loading'>
        <div className='row vh-100 justify-content-center align-items-center'>
            <p>This is TOP PAGE !</p>
        </div>
    </div>
    );
};

export default Root;
