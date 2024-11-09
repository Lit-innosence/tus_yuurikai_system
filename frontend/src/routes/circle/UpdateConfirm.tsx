import React, { useState } from 'react';
import { useNavigate, useLocation } from 'react-router-dom';
import { Button, Layout, Card, Checkbox, message } from 'antd';
import axios from 'axios';
import CustomHeader from '../component/CustomHeader';
import CustomFooter from '../component/CustomFooter';

const { Content } = Layout;

const CircleUpdateConfirm: React.FC = () => {
    const navigate = useNavigate();
    const location = useLocation();
    const { formData } = location.state as { formData: any };

    const [isChecked, setIsChecked] = useState(false);

    const handleCheckboxChange = (e: any) => {
        setIsChecked(e.target.checked);
    };

    const handleConfirm = async () => {
        try {
            const response = await axios.post('/api/circle/update', formData);
            if (response.status === 200) {
                message.success('団体情報が正常に更新されました');
                navigate('/circle/update/complete');
            }
        } catch (error) {
            message.error('団体情報の更新に失敗しました');
        }
    };

    return (
        <Layout style={{ minHeight: '100vh' }}>
            <CustomHeader />
            <Content style={{ padding: '50px', display: 'flex', justifyContent: 'center', alignItems: 'center' }}>
                <Card
                    title="入力内容の確認"
                    bordered={true}
                    style={{ width: '100%', maxWidth: '600px', textAlign: 'left' }}
                    headStyle={{ fontSize: '1.5em', textAlign: 'center' }}
                >
                    <h3>団体情報</h3>
                    <p><strong>団体ID:</strong> {formData.organizationId}</p>
                    <p><strong>団体名:</strong> {formData.organizationName}</p>
                    
                    <hr style={{ width: '100%', border: 'none', borderTop: '1px solid #e8e8e8', margin: '20px 0' }} />

                    <h3>代表者情報</h3>
                    <p><strong>代表者名:</strong> {formData.familyName} {formData.givenName}</p>
                    <p><strong>代表者の学生ID:</strong> {formData.studentId}</p>
                    <p><strong>代表者のメールアドレス:</strong> {formData.email}</p>
                    
                    <div style={{ display: 'flex', justifyContent: 'center', marginTop: '20px' }}>
                        <Checkbox onChange={handleCheckboxChange}>
                            入力内容を確認しました。
                        </Checkbox>
                    </div>

                    <div style={{ display: 'flex', justifyContent: 'center', marginTop: '30px' }}>
                        <Button type="primary" onClick={handleConfirm} disabled={!isChecked}>
                            確認して更新
                        </Button>
                    </div>
                </Card>
            </Content>
            <CustomFooter />
        </Layout>
    );
};

export default CircleUpdateConfirm;
