import React from 'react';
import { Layout } from 'antd';

const { Header } = Layout;

const CustomHeader: React.FC = () => {
    return (
        <Header style={{ color: 'white', textAlign: 'center', backgroundColor: '#004ab3' }}>
            TUS YURIKAI SYSTEM
        </Header>
    );
};

export default CustomHeader;
