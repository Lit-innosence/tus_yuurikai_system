import React, { useState } from 'react';
import { Layout, Input, Button, Form, Typography, Card, Table, message } from 'antd';
import CustomAdminHeader from '../component/CustomAdminHeader';
import CustomFooter from '../component/CustomFooter';
import axios from 'axios';
import constants from '../constants';

const { Content } = Layout;
const { Title } = Typography;

const LockerUserSearch: React.FC = () => {
    const [searchResults, setSearchResults] = useState([]);
    const [form] = Form.useForm();

    const handleSearch = async (values: { family_name?: string; given_name?: string; floor?: number; year?: number }) => {
        // クエリパラメータを生成
        const year = values.year || new Date().getFullYear();
        let query = `${constants.backendApiEndpoint}/api/admin/locker/user-search/${year}`;
        const params = new URLSearchParams();

        if (values.family_name) params.append('familyname', values.family_name);
        if (values.given_name) params.append('givenname', values.given_name);
        if (values.floor !== undefined) params.append('floor', values.floor.toString());

        if (params.toString()) {
            query += `?${params.toString()}`;
        }

        console.log('Generated Query:', query);

        try {
            // APIからデータを取得
            const response = await axios.get(query, { withCredentials: true });
            setSearchResults(response.data.data);
        } catch (error: any) {
            if (axios.isAxiosError(error)) {
                if (error.response?.status === 401 || error.response?.status === 400) {
                    message.error('認証エラーです。再度ログインしてください。');
                    window.location.href = '/login';
                } else if (error.response?.status === 404) {
                    message.warning('該当するデータが見つかりませんでした。');
                } else {
                    console.error('データの取得に失敗しました:', error);
                    message.error('データの取得に失敗しました。');
                }
            }
        }
    };

    // 検索結果のカラム定義
    const columns = [
        {
            title: 'ロッカーID',
            dataIndex: 'lockerId',
            key: 'lockerId',
        },
        {
            title: '階数',
            dataIndex: 'floor',
            key: 'floor',
        },
        {
            title: '年',
            dataIndex: 'year',
            key: 'year',
        },
        {
            title: '主利用者 [学籍番号, 名前]',
            key: 'mainUser',
            render: (_: any, record: any) => {
                const mainUser = record.mainUser;
                return `[${mainUser.studentId}, ${mainUser.familyName} ${mainUser.givenName}]`;
            },
        },
        {
            title: '共同利用者 [学籍番号, 名前]',
            key: 'coUser',
            render: (_: any, record: any) => {
                const coUser = record.coUser;
                return `[${coUser.studentId}, ${coUser.familyName} ${coUser.givenName}]`;
            },
        },
    ];

    return (
        <Layout style={{ minHeight: '100vh' }}>
            <CustomAdminHeader />
            <Content style={{ padding: '50px 50px', background: '#fff' }}>
                <Card style={{ maxWidth: 800, margin: '20px auto', padding: '20px' }}>
                    <Title level={3}>ロッカー利用者検索</Title>
                    <Form form={form} layout="vertical" onFinish={handleSearch}>
                        <Form.Item rules={[{ required: true, message: '年を入力してください' }]} label="年" name="year">
                            <Input type="number" placeholder="年を入力 (例: 2024)" />
                        </Form.Item>
                        <Form.Item label="姓" name="family_name">
                            <Input placeholder="姓を入力" />
                        </Form.Item>
                        <Form.Item label="名" name="given_name">
                            <Input placeholder="名を入力" />
                        </Form.Item>
                        <Form.Item label="階数" name="floor">
                            <Input type="number" placeholder="階数を入力" />
                        </Form.Item>
                        <Button type="primary" htmlType="submit" block>
                            検索
                        </Button>
                    </Form>
                </Card>

                {searchResults.length > 0 && (
                    <Card style={{ maxWidth: 1000, margin: '20px auto', padding: '20px' }}>
                        <Title level={4}>検索結果</Title>
                        <Table
                            columns={columns}
                            dataSource={searchResults}
                            rowKey="lockerId"
                            pagination={false}
                            bordered
                        />
                    </Card>
                )}
            </Content>
            <CustomFooter />
        </Layout>
    );
};

export default LockerUserSearch;
