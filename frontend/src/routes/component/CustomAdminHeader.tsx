import React from 'react';
import { Layout, Button } from 'antd';
import { useAuth } from './AuthContext';
import { useNavigate } from 'react-router-dom';

const { Header } = Layout;

const AdminCustomHeader: React.FC = () => {
    const { handleLogout } = useAuth();
    const navigate = useNavigate();

    const handleLogoutClick = async () => {
        await handleLogout();
        navigate('/login'); 
    };

    return (
        <Header style={{ 
            color: 'white', 
            textAlign: 'center', 
            backgroundColor: '#004ab3', 
            display: 'flex', 
            justifyContent: 'center', 
            alignItems: 'center',
            position: 'relative'
        }}>
            <span style={{ fontSize: '18px' }}>TUS YURIKAI SYSTEM</span>
            <Button 
                type="primary" 
                onClick={handleLogoutClick} 
                danger 
                style={{ position: 'absolute', right: '16px' }}
            >
                ログアウト
            </Button>
        </Header>
    );
};

export default AdminCustomHeader;
