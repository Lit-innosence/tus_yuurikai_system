import React from 'react';
import { Layout, Button } from 'antd';
import { useNavigate } from 'react-router-dom';

const { Header } = Layout;

const CustomHeader: React.FC = () => {
    const navigate = useNavigate();

    const handleGoBack = () => {
        navigate(-1);
    };

    return (
        <Header
            style={{
                position: 'relative',
                backgroundColor: '#004ab3',
                height: '64px',
                padding: '0 20px'
            }}
        >
            {/* 左側の戻るボタン */}
            <Button
                type="primary"
                onClick={handleGoBack}
                style={{
                    position: 'absolute',
                    left: '20px',
                    top: '50%',
                    transform: 'translateY(-50%)'
                }}
            >
                戻る
            </Button>
            {/* 中央に固定されたタイトル */}
            <div
                style={{
                    position: 'absolute',
                    left: '50%',
                    top: '50%',
                    transform: 'translate(-50%, -50%)',
                    color: 'white',
                    fontSize: '16px'
                }}
            >
                TUS YURIKAI SYSTEM
            </div>
        </Header>
    );
};

export default CustomHeader;
