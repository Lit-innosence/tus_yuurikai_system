import React from 'react';
import { useNavigate } from 'react-router-dom';
import { Button, Result } from 'antd';

type TimeoutProps = {
    children?: React.ReactNode;
};

const Timeout: React.FC<TimeoutProps> = (props) => {
    const navigate = useNavigate();

    const containerStyle: React.CSSProperties = {
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'center',
        height: '100vh',
        width: '100vw',
    };

    return (
        <div style={containerStyle}>
            <Result
                status="info"
                title="申請期間外です"
                subTitle="現在、申請は受け付けておりません。次回の申請期間までお待ちください。"
                extra={<Button type="primary" onClick={() => navigate('/')}>ホームに戻る</Button>}
            />
        </div>
    );
};

export default Timeout;
