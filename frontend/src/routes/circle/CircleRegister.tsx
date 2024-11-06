import React, { useEffect, useState } from 'react';
import { Table, Typography, Layout, Input, message } from 'antd';
// import axios from 'axios';
import CustomHeader from '../component/CustomHeader';
import CustomFooter from '../component/CustomFooter';

const { Title } = Typography;
const { Content } = Layout;
const { Search } = Input;

interface Organization {
    organizationId: string;
    organizationName: string;
    statusAcceptance: string;
    statusAuthentication: string;
    statusFormConfirmation: string;
    statusRegistrationComplete: string;
}

const mockData: Organization[] = [
    {
        organizationId: "550e8400-e29b-41d4-a716-446655440000",
        organizationName: "団体A",
        statusAcceptance: "accepted",
        statusAuthentication: "authenticated",
        statusFormConfirmation: "confirmed",
        statusRegistrationComplete: "completed"
    },
    {
        organizationId: "550e8400-e29b-41d4-a716-446655440001",
        organizationName: "団体B",
        statusAcceptance: "pending",
        statusAuthentication: "not_authenticated",
        statusFormConfirmation: "not_confirmed",
        statusRegistrationComplete: "incomplete"
    },
];

const pageSize = 5;

const statusMapping: Record<string, Record<string, { label: string; color: string }>> = {
    statusAcceptance: {
        accepted: { label: '承認済み', color: 'green' },
        pending: { label: '承認待ち', color: 'red' },
    },
    statusAuthentication: {
        authenticated: { label: '認証済み', color: 'green' },
        not_authenticated: { label: '未認証', color: 'red' },
    },
    statusFormConfirmation: {
        confirmed: { label: '確認済み', color: 'green' },
        not_confirmed: { label: '未確認', color: 'red' },
    },
    statusRegistrationComplete: {
        completed: { label: '完了', color: 'green' },
        incomplete: { label: '未完了', color: 'red' },
    },
};

const CircleRegister: React.FC = () => {
    const [organizations, setOrganizations] = useState<Organization[]>([]);
    const [loading, setLoading] = useState(true);
    const [filteredData, setFilteredData] = useState<Organization[]>([]);
    const [searchTerm, setSearchTerm] = useState<string>('');

    useEffect(() => {
        // モックデータを使用
        setTimeout(() => {
            setOrganizations(mockData);
            setFilteredData(mockData);
            setLoading(false);
        }, 1000);

        // 本来は以下のようにAPIを叩いてデータを取得する
        /*
        const fetchOrganizations = async () => {
            try {
                const response = await axios.get<{ data: Organization[] }>('/api/circle/status');
                setOrganizations(response.data.data);
                setFilteredData(response.data.data);
            } catch (error) {
                message.error('データの取得に失敗しました。');
            } finally {
                setLoading(false);
            }
        };
        fetchOrganizations();
        */
    }, []);

    const getStatusLabel = (status: string, statusType: string) => {
        const statusInfo = statusMapping[statusType]?.[status];
        if (statusInfo) {
            return <span style={{ color: statusInfo.color }}>{statusInfo.label}</span>;
        }
        // もしステータスが存在しない場合
        return <span style={{ color: 'grey' }}>不明</span>;
    };

    const handleSearch = (value: string) => {
        setSearchTerm(value);
        const filtered = organizations.filter(org =>
            org.organizationName.toLowerCase().includes(value.toLowerCase())
        );
        setFilteredData(filtered);
    };

    const columns = [
        {
            title: '団体ID',
            dataIndex: 'organizationId',
            key: 'organizationId',
            width: 150,
        },
        {
            title: '団体名',
            dataIndex: 'organizationName',
            key: 'organizationName',
        },
        {
            title: '承認状況',
            dataIndex: 'statusAcceptance',
            key: 'statusAcceptance',
            render: (status: string) => getStatusLabel(status, 'statusAcceptance'),
        },
        {
            title: '認証状況',
            dataIndex: 'statusAuthentication',
            key: 'statusAuthentication',
            render: (status: string) => getStatusLabel(status, 'statusAuthentication'),
        },
        {
            title: 'フォーム確認状況',
            dataIndex: 'statusFormConfirmation',
            key: 'statusFormConfirmation',
            render: (status: string) => getStatusLabel(status, 'statusFormConfirmation'),
        },
        {
            title: '登録完了状況',
            dataIndex: 'statusRegistrationComplete',
            key: 'statusRegistrationComplete',
            render: (status: string) => getStatusLabel(status, 'statusRegistrationComplete'),
        },
    ];

    return (
        <Layout style={{ minHeight: '100vh' }}>
            <CustomHeader />
            <Content
                style={{
                    padding: '50px',
                    display: 'flex',
                    justifyContent: 'center',
                    alignItems: 'center',
                    flexDirection: 'column',
                }}
            >
                <div style={{ width: '100%', maxWidth: '800px' }}>
                    <Title level={2}>団体情報</Title>
                    <Search
                        placeholder="団体名で検索"
                        value={searchTerm}
                        onChange={(e) => handleSearch(e.target.value)}
                        onSearch={handleSearch}
                        style={{ marginBottom: '20px', width: '100%' }}
                    />
                    <Table
                        columns={columns}
                        dataSource={filteredData}
                        rowKey="organizationId"
                        loading={loading}
                        pagination={{ pageSize }}
                    />
                </div>
            </Content>
            <CustomFooter />
        </Layout>
    );
};

export default CircleRegister;
