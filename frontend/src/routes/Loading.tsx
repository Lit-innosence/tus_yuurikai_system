import React from 'react';
import { Spin } from 'antd';
import { LoadingOutlined } from '@ant-design/icons';

type LoadingProps = {
    children?: React.ReactNode;
};

const loadingStyle: React.CSSProperties = {
    display: 'flex',
    justifyContent: 'center',
    alignItems: 'center',
    height: '100vh', 
    width: '100vw', 
};

const Loading: React.FC<LoadingProps> = (props) => {

    return (
        <div style={loadingStyle}>
            <Spin indicator={<LoadingOutlined spin />} tip="Loading..." style={{transform: 'scale(3)'}} />
        </div>
    );
};

export default Loading;
