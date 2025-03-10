import React from 'react';
import { Layout } from 'antd';

const { Footer } = Layout;

const CustomFooter: React.FC = () => {
    const currentYear = new Date().getFullYear();
    return (
        <Footer style={{ textAlign: 'center', backgroundColor: 'white' }}>
            © 2024-{currentYear} TUS YUURIKAI SYSTEM All Rights Reserved.
        </Footer>
    );
};

export default CustomFooter;
