import React from 'react';
import { useNavigate } from 'react-router-dom';
import { Result } from 'antd';

const RegisterComp: React.FC = () => {
  const navigate = useNavigate();  

  return (
    <div style={{ display: 'flex', justifyContent: 'center', alignItems: 'center', minHeight: '100vh' }}>
      <Result
        status="success"
        title="ロッカー登録に成功しました！"
        subTitle="ロッカー申請が許可されました。"
      />
    </div>
  );
};

export default RegisterComp;
