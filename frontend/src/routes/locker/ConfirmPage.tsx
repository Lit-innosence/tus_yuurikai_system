import React, { useState } from 'react';
import { useNavigate, useLocation } from 'react-router-dom';
import axios from 'axios';
import { Button, Layout, Card, Checkbox, message } from 'antd';
import CustomHeader from '../component/CustomHeader';
import CustomFooter from '../component/CustomFooter';
import constants from '../constants';

const { Content } = Layout;

const ConfirmPage: React.FC = () => {
    const navigate = useNavigate();
    const location = useLocation();
    const { formData } = location.state as { formData: any };

    // チェックボックスの状態を管理
    const [isChecked, setIsChecked] = useState(false);

    const handleCheckboxChange = (e: any) => {
        setIsChecked(e.target.checked);
    };

    const handleConfirm = async () => {
        const formattedData = {
            data: {
                mainUser: {
                    studentId: formData.studentId,
                    familyName: formData.lastName,
                    givenName: formData.firstName,
                },
                coUser: {
                    studentId: formData.coUserStudentId,
                    familyName: formData.coUserLastName,
                    givenName: formData.coUserFirstName,
                },
            },
        };

        try {
            const response = await axios.post(`${constants.backendApiEndpoint}/api/locker/token-gen`, formattedData);
            console.log('成功:', response.data);
            message.success('フォームが正常に送信されました');
            navigate('/locker/form/complete');
        } catch (error) {
            console.error('エラー:', error);
            message.error('送信に失敗しました');
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
                    <h3>申請者</h3>
                    <p><strong>学籍番号:</strong> {formData.studentId}</p>
                    <p><strong>氏名:</strong> {formData.lastName} {formData.firstName}</p>
                    
                    <hr style={{ width: '100%', border: 'none', borderTop: '1px solid #e8e8e8', margin: '20px 0' }} />

                    <h3>共同利用者</h3>
                    <p><strong>学籍番号:</strong> {formData.coUserStudentId}</p>
                    <p><strong>氏名:</strong> {formData.coUserLastName} {formData.coUserFirstName}</p>
                    
                    <div style={{ display: 'flex', justifyContent: 'center', marginTop: '20px' }}>
                        <Checkbox onChange={handleCheckboxChange}>
                            入力内容を確認しました。
                        </Checkbox>
                    </div>

                    <div style={{ display: 'flex', justifyContent: 'center', marginTop: '30px' }}>
                        <Button type="primary" onClick={handleConfirm} disabled={!isChecked}>
                            確認して登録
                        </Button>
                    </div>
                </Card>
            </Content>
            <CustomFooter />
        </Layout>
    );
};

export default ConfirmPage;
