import React from 'react';
import { Layout } from 'antd';

const { Footer } = Layout;

const CustomFooter: React.FC = () => {
    return (
        <Footer style={{ textAlign: 'center', backgroundColor: 'white' }}>
            YURIKAI SYSTEM ©2024
        </Footer>
    );
};

export default CustomFooter;
