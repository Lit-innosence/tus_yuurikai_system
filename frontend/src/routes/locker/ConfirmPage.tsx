import React from 'react';
import { useNavigate, useLocation } from 'react-router-dom';
import axios from 'axios';
import { Button, Layout, Card, message } from 'antd';

const { Header, Content, Footer } = Layout;

const ConfirmPage: React.FC = () => {
    const navigate = useNavigate();
    const location = useLocation();
    const { formData } = location.state as { formData: any };

    const handleConfirm = async () => {
        const formattedData = {
            data: {
                main_user: {
                    student_id: formData.studentId,
                    family_name: formData.lastName,
                    given_name: formData.firstName,
                },
                co_user: {
                    student_id: formData.coUserStudentId,
                    family_name: formData.coUserLastName,
                    given_name: formData.coUserFirstName,
                },
            },
        };

        try {
            // const response = await axios.post('https://your-backend-server.com/api/endpoint', formattedData);
            // console.log('成功:', response.data);
            message.success('フォームが正常に送信されました');
            navigate('/form-complete');
        } catch (error) {
            console.error('エラー:', error);
            message.error('送信に失敗しました');
        }
    };

    return (
        <Layout style={{ minHeight: '100vh' }}>
            <Header style={{ color: 'white', textAlign: 'center', backgroundColor: '#004ab3' }}>
                TUS_YURUKAI_SYSTEM
            </Header>
            <Content style={{ padding: '50px', display: 'flex', justifyContent: 'center', alignItems: 'center' }}>
                <Card
                    title="入力内容の確認"
                    bordered={true}
                    style={{ width: '100%', maxWidth: '600px', textAlign: 'left' }}
                    headStyle={{ fontSize: '1.5em', textAlign: 'center' }}
                >
                    <h3>利用者</h3>
                    <p><strong>学籍番号:</strong> {formData.studentId}</p>
                    <p><strong>氏名:</strong> {formData.lastName} {formData.firstName}</p>
                    
                    <hr style={{ width: '100%', border: 'none', borderTop: '1px solid #e8e8e8', margin: '20px 0' }} />

                    <h3>共同利用者</h3>
                    <p><strong>学籍番号:</strong> {formData.coUserStudentId}</p>
                    <p><strong>氏名:</strong> {formData.coUserLastName} {formData.coUserFirstName}</p>
                    
                    <div style={{ display: 'flex', justifyContent: 'center', marginTop: '30px' }}>
                        <Button type="primary" onClick={handleConfirm}>
                            確認して登録
                        </Button>
                    </div>
                </Card>
            </Content>
            <Footer style={{ textAlign: 'center', backgroundColor: 'white' }}>
                YURUKAI SYSTEM ©2024
            </Footer>
        </Layout>
    );
};

export default ConfirmPage;
