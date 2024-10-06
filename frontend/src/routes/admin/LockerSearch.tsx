import React, { useState } from 'react';
import { Layout, Input, Button, Form, Typography, Card, Table } from 'antd';
import CustomHeader from '../component/CustomHeader';
import CustomFooter from '../component/CustomFooter';

const { Content } = Layout;
const { Title } = Typography;

// モックデータ
const mockResponse = {
    data: [
        {
        locker_id: '2001',
        floor: 2,
        main_user: {
            student_id: '4622999',
            family_name: '山田',
            given_name: '太郎',
        },
        co_user: {
            student_id: '4623000',
            family_name: '山田',
            given_name: '次郎',
        },
        year: 2024,
        },
        {
        locker_id: '3001',
        floor: 3,
        main_user: {
            student_id: '4522999',
            family_name: '佐藤',
            given_name: '太郎',
        },
        co_user: {
            student_id: '4523000',
            family_name: '佐藤',
            given_name: '次郎',
        },
        year: 2024,
        },
    ],
};

const LockerUserSearch: React.FC = () => {
const [searchResults, setSearchResults] = useState<typeof mockResponse.data>([]);
const [form] = Form.useForm();

const handleSearch = async (values: { family_name?: string; given_name?: string; floor?: number; year?: number }) => {
    // クエリパラメータを生成
    const year = values.year || new Date().getFullYear();
    let query = `/locker/user-search/${year}/`;
    const params = new URLSearchParams();

    if (values.family_name) params.append('family_name', values.family_name);
    if (values.given_name) params.append('given_name', values.given_name);
    if (values.floor !== undefined) params.append('floor', values.floor.toString());

    if (params.toString()) {
    query += `?${params.toString()}`;
    }

    console.log('Generated Query:', query);

    /*
    try {
    const response = await axios.get(query);
    setSearchResults(response.data.data); // レスポンスデータを設定
    } catch (error) {
    console.error('Error fetching data:', error);
    }
    */

    // モックデータを使ったフィルタリング
    const filteredResults = mockResponse.data.filter((item) => {
    return (
        (!values.year || item.year === values.year) &&
        (!values.family_name || item.main_user.family_name.includes(values.family_name) || item.co_user.family_name.includes(values.family_name)) &&
        (!values.given_name || item.main_user.given_name.includes(values.given_name) || item.co_user.given_name.includes(values.given_name)) &&
        (values.floor === undefined || item.floor === values.floor)
    );
    });

    setSearchResults(filteredResults);
};

// 検索結果のカラム定義
const columns = [
    {
    title: 'ロッカーID',
    dataIndex: 'locker_id',
    key: 'locker_id',
    },
    {
    title: 'フロア',
    dataIndex: 'floor',
    key: 'floor',
    },
    {
    title: '年',
    dataIndex: 'year',
    key: 'year',
    },
    {
    title: '主利用者',
    key: 'main_user',
    render: (record: any) => `${record.main_user.family_name} ${record.main_user.given_name} (学籍番号: ${record.main_user.student_id})`,
    },
    {
    title: '共同利用者',
    key: 'co_user',
    render: (record: any) => `${record.co_user.family_name} ${record.co_user.given_name} (学籍番号: ${record.co_user.student_id})`,
    },
];

return (
    <Layout style={{ minHeight: '100vh' }}>
        <CustomHeader />
        <Content style={{ padding: '50px 50px', background: '#fff' }}>
            <Card style={{ maxWidth: 800, margin: '20px auto', padding: '20px' }}>
            <Title level={3}>ロッカー利用者検索</Title>
            <Form form={form} layout="vertical" onFinish={handleSearch}>
                <Form.Item label="年" name="year">
                <Input type="number" placeholder="年を入力 (例: 2024)" />
                </Form.Item>
                <Form.Item label="姓" name="family_name">
                <Input placeholder="姓を入力" />
                </Form.Item>
                <Form.Item label="名" name="given_name">
                <Input placeholder="名を入力" />
                </Form.Item>
                <Form.Item label="フロア" name="floor">
                <Input type="number" placeholder="フロア番号を入力" />
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
                rowKey="locker_id"
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
