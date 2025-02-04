import React from 'react';
import { useNavigate } from 'react-router-dom';
import { Button, Result } from 'antd';

type Page404Props = {
    children?: React.ReactNode;
};

const Page404: React.FC<Page404Props> = (props) => {
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
                status="404"
                title="404"
                subTitle="すみません、そのようなページは存在しません。"
                extra={<Button type="primary" onClick={() => { navigate('/') }}>ホームに戻る</Button>}
            />
        </div>
    );
};

export default Page404;
