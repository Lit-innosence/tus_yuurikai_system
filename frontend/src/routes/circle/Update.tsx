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
                            { pattern: /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i, message: '有効なIDを入力してください。'}
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

                    <Form.Item label="代表者名">
                        <Input.Group compact>
                            <Form.Item
                                name="familyName"
                                noStyle
                                rules={[
                                    { required: true, message: '代表者の名字を入力してください' }
                                ]}
                            >
                                <Input style={{ width: '50%' }} placeholder="名字" />
                            </Form.Item>
                            <Form.Item
                                name="givenName"
                                noStyle
                                rules={[
                                    { required: true, message: '代表者の名前を入力してください' }
                                ]}
                            >
                                <Input style={{ width: '50%' }} placeholder="名前" />
                            </Form.Item>
                        </Input.Group>
                    </Form.Item>

                    <Form.Item
                        label="代表者の学生ID"
                        name="studentId"
                        rules={[
                            { required: true, message: '代表者の学生IDを入力してください' },
                            { pattern: /^[0-9AB]+$/, message: '学籍番号は0~9, A, Bの文字のみで入力してください。' }
                        ]}
                    >
                        <Input placeholder="代表者の学生IDを入力" />
                    </Form.Item>

                    <Form.Item
                        label="代表者のメールアドレス"
                        name="email"
                        rules={[
                            { required: true, message: '代表者のメールアドレスを入力してください' },
                            { type: 'email', message: '有効なメールアドレスを入力してください' }
                        ]}
                    >
                        <Input placeholder="代表者のメールアドレスを入力" />
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
