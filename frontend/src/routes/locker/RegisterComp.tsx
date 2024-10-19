import React from 'react';
import { useLocation } from 'react-router-dom';
import { Result } from 'antd';

const RegisterComp: React.FC = () => {
    const location = useLocation();
    const { lockerId } = location.state || {}; // Retrieve lockerId from location state

    return (
        <div style={{ display: 'flex', justifyContent: 'center', alignItems: 'center', minHeight: '100vh' }}>
            <Result
                status="success"
                title="ロッカー登録に成功しました！"
                subTitle={`ロッカー番号は${lockerId}です`} // Display the lockerId dynamically
            />
        </div>
    );
};

export default RegisterComp;
