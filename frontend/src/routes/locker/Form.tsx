import React from 'react';
import { useNavigate } from 'react-router-dom';
import { Form, Input, Button, Layout } from 'antd';

const { Header, Content, Footer } = Layout;

const LockerForm: React.FC = () => {
    const navigate = useNavigate();  

    const onFinish = (values: any) => {
        // 入力内容を確認ページに渡す
        navigate('/locker-confirm', { state: { formData: values } });
    };

    return (
        <Layout style={{ minHeight: '100vh' }}>
            <Header style={{ color: 'white', textAlign: 'center', backgroundColor: '#004ab3' }}>
                TUS YURIKAI SYSTEM
            </Header>
            <Content style={{ padding: '50px', display: 'flex', justifyContent: 'center', alignItems: 'center' }}>
                <Form
                    name="lockerForm"
                    layout="vertical"
                    onFinish={onFinish}
                    style={{ maxWidth: '600px', width: '100%' }}
                >
                    <h3>利用者の情報</h3>
                    <Form.Item
                        label="学籍番号"
                        name="studentId"
                        rules={[
                            { required: true, message: '学籍番号を入力してください' },
                            {
                                pattern: /^[0-9AB]+$/,
                                message: '学籍番号は0~9, A, Bの文字のみで入力してください。'
                            }
                        ]}
                    >
                        <Input placeholder="学籍番号を入力" />
                    </Form.Item>

                    <Form.Item label="氏名">
                        <Input.Group compact>
                            <Form.Item
                                name="lastName"
                                noStyle
                                rules={[
                                    { required: true, message: '姓を入力してください' },
                                    {
                                        pattern: /^[A-Za-z\u3040-\u30FF\u4E00-\u9FFF]+$/,  
                                        message: '姓は日本語、もしくは英語で入力してください'
                                    }
                                ]}
                            >
                                <Input style={{ width: '50%' }} placeholder="姓" />
                            </Form.Item>
                            <Form.Item
                                name="firstName"
                                noStyle
                                rules={[
                                    { required: true, message: '名を入力してください' },
                                    {
                                        pattern: /^[A-Za-z\u3040-\u30FF\u4E00-\u9FFF]+$/,  
                                        message: '名は日本語、もしくは英語で入力してください'
                                    }
                                ]}
                            >
                                <Input style={{ width: '50%' }} placeholder="名" />
                            </Form.Item>
                        </Input.Group>
                    </Form.Item>

                    <h3>共同利用者の情報</h3>
                    <Form.Item
                        label="共同利用者 学籍番号"
                        name="coUserStudentId"
                        rules={[
                            { required: true, message: '共同使用者の学籍番号を入力してください' },
                            {
                                pattern: /^[0-9AB]+$/,
                                message: '学籍番号は0~9, A, Bの文字のみで入力してください。'
                            }
                        ]}
                    >
                        <Input placeholder="共同使用者の学籍番号を入力" />
                    </Form.Item>

                    <Form.Item label="共同利用者 氏名">
                        <Input.Group compact>
                            <Form.Item
                                name="coUserLastName"
                                noStyle
                                rules={[
                                    { required: true, message: '共同使用者の姓を入力してください' },
                                    {
                                        pattern: /^[A-Za-z\u3040-\u30FF\u4E00-\u9FFF]+$/,  
                                        message: '共同使用者の姓は日本語、もしくは英語で入力してください'
                                    }
                                ]}
                            >
                                <Input style={{ width: '50%' }} placeholder="共同使用者の姓" />
                            </Form.Item>
                            <Form.Item
                                name="coUserFirstName"
                                noStyle
                                rules={[
                                    { required: true, message: '共同使用者の名を入力してください' },
                                    {
                                        pattern: /^[A-Za-z\u3040-\u30FF\u4E00-\u9FFF]+$/,  
                                        message: '共同使用者の名は日本語、もしくは英語で入力してください'
                                    }
                                ]}
                            >
                                <Input style={{ width: '50%' }} placeholder="共同使用者の名" />
                            </Form.Item>
                        </Input.Group>
                    </Form.Item>

                    <Form.Item>
                        <Button type="primary" htmlType="submit" block>
                            次へ
                        </Button>
                    </Form.Item>
                </Form>
            </Content>
            <Footer style={{ textAlign: 'center', backgroundColor: 'white' }}>
                YURUKAI SYSTEM ©2024
            </Footer>
        </Layout>
    );
};

export default LockerForm;
