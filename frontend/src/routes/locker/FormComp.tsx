import React from 'react';
import { useNavigate } from 'react-router-dom';
import { Button, Result } from 'antd';

const FormComp: React.FC = () => {
    const navigate = useNavigate();  

    return (
        <div style={{ display: 'flex', justifyContent: 'center', alignItems: 'center', minHeight: '100vh' }}>
        <Result
            status="success"
            title="フォームの提出に成功しました！"
            subTitle="登録した学籍番号宛に認証用のメールを送信しました。"
            extra={[
            <Button type="primary" key="dashboard" onClick={() => navigate('/locker-process')}>
                今後のステップを確認する
            </Button>,
            ]}
        />
        </div>
    );
};

export default FormComp;
