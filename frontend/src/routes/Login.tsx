import React from 'react';
import { useNavigate } from 'react-router-dom';
import { Form, Input, Button, Layout, message } from 'antd';
import { useAuth } from '../components/AuthContext';
import CustomHeader from '../components/CustomHeader';
import CustomFooter from '../components/CustomFooter';

const { Content } = Layout;

const LoginForm: React.FC = () => {
    const navigate = useNavigate();
    const { handleLogin } = useAuth(); 

    const onFinish = async (values: any) => {
        try {
            await handleLogin(values);
            message.success('ログインに成功しました');
            navigate('/admin'); 
        } catch (error: any) {
            message.error(error.message);
        }
    };
    

    return (
        <Layout style={{ minHeight: '100vh' }}>
            <CustomHeader />
            <Content style={{ padding: '50px', display: 'flex', justifyContent: 'center', alignItems: 'center' }}>
                <Form
                    name="loginForm"
                    layout="vertical"
                    onFinish={onFinish}
                    style={{ maxWidth: '400px', width: '100%' }}
                >
                    <h3>ログイン</h3>
                    <Form.Item
                        label="ユーザー名"
                        name="username"
                        rules={[{ required: true, message: 'ユーザー名を入力してください' }]}
                    >
                        <Input placeholder="ユーザー名を入力" />
                    </Form.Item>

                    <Form.Item
                        label="パスワード"
                        name="password"
                        rules={[{ required: true, message: 'パスワードを入力してください' }]}
                    >
                        <Input.Password placeholder="パスワードを入力" />
                    </Form.Item>

                    <Form.Item>
                        <Button type="primary" htmlType="submit" block>
                            ログイン
                        </Button>
                    </Form.Item>
                </Form>
            </Content>
            <CustomFooter />
        </Layout>
    );
};

export default LoginForm;
