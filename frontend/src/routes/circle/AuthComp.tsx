import React from 'react';
import { useNavigate } from 'react-router-dom';
import { Button, Result } from 'antd';

const AuthComp: React.FC = () => {
  const navigate = useNavigate();  

  return (
    <div style={{ display: 'flex', justifyContent: 'center', alignItems: 'center', minHeight: '100vh' }}>
      <Result
        status="success"
        title="認証に成功しました！"
        subTitle="団体登録をするには、代表者と副代表者、両方のメール認証が必要になります。"
        extra={[
          <Button type="primary" key="dashboard" onClick={() => navigate('/circle/register/process')}>
            今後のステップを確認する
          </Button>,
        ]}
      />
    </div>
  );
};

export default AuthComp;
