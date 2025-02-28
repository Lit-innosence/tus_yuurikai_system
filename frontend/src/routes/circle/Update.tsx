import React from 'react';
import { useNavigate } from 'react-router-dom';
import { Form, Input, Button, Layout, message } from 'antd';
import CustomHeader from '../component/CustomHeader';
import CustomFooter from '../component/CustomFooter';

const { Content } = Layout;

const CircleUpdate: React.FC = () => {
    const navigate = useNavigate();

    const onFinish = (values: any) => {
        navigate('/circle/update/confirm', { state: { formData: values } });
    };

    return (
        <Layout style={{ minHeight: '100vh' }}>
            <CustomHeader />
            <Content style={{ padding: '50px', display: 'flex', justifyContent: 'center', alignItems: 'center' }}>
                <Form
                    name="circleUpdateForm"
                    layout="vertical"
                    onFinish={onFinish}
                    style={{ maxWidth: '600px', width: '100%' }}
                >
                    <h3>団体情報の更新</h3>

                    <Form.Item
                        label="団体ID"
                        name="organizationId"
                        rules={[
                            { required: true, message: '団体IDを入力してください' },
                            { pattern: /^C\d{5}$/, message: '有効なIDを入力してください。' }
                        ]}
                    >
                        <Input placeholder="団体IDを入力" />
                    </Form.Item>

                    <Form.Item
                        label="団体名"
                        name="organizationName"
                        rules={[
                            { required: true, message: '団体名を入力してください' }
                        ]}
                    >
                        <Input placeholder="団体名を入力" />
                    </Form.Item>

                    <Form.Item label="旧代表者名">
                        <Input.Group compact>
                            <Form.Item
                                name="familyName"
                                noStyle
                                rules={[
                                    { required: true, message: '旧代表者の名字を入力してください' },
                                    { pattern: /^[a-zA-Z\p{sc=Kana}\p{sc=Hira}\p{sc=Han}]+$/u, message: '苗字は日本語、もしくは英語で入力してください'}
                                ]}
                            >
                                <Input style={{ width: '50%' }} placeholder="名字" />
                            </Form.Item>
                            <Form.Item
                                name="givenName"
                                noStyle
                                rules={[
                                    { required: true, message: '旧代表者の名前を入力してください' },
                                    { pattern: /^[a-zA-Z\p{sc=Kana}\p{sc=Hira}\p{sc=Han}]+$/u, message: '苗字は日本語、もしくは英語で入力してください'}
                                ]}
                            >
                                <Input style={{ width: '50%' }} placeholder="名前" />
                            </Form.Item>
                        </Input.Group>
                    </Form.Item>

                    <Form.Item
                        label="旧代表者の学籍番号"
                        name="studentId"
                        rules={[
                            { required: true, message: '旧代表者の学籍番号を入力してください' },
                            { pattern: /^[1-4678][1-9ab]\d{5}$/, message: '有効な学籍番号を入力してください。' }
                        ]}
                    >
                        <Input placeholder="旧代表者の学籍番号を入力" />
                    </Form.Item>

                    <Form.Item
                        label="旧代表者のメールアドレス"
                        name="email"
                        rules={[
                            { required: true, message: '旧代表者のメールアドレスを入力してください' },
                            { type: 'email', message: '有効なメールアドレスを入力してください' }
                        ]}
                    >
                        <Input placeholder="旧代表者のメールアドレスを入力" />
                    </Form.Item>

                    <Form.Item>
                        <Button type="primary" htmlType="submit" block>
                            確認する
                        </Button>
                    </Form.Item>
                </Form>
            </Content>
            <CustomFooter />
        </Layout>
    );
};

export default CircleUpdate;
