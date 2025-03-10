import React, { useEffect, useState } from 'react';
import { Table, Typography, Layout, Input, message } from 'antd';
import axios from 'axios';
import CustomHeader from '../../components/CustomHeader';
import CustomFooter from '../../components/CustomFooter';

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

const pageSize = 5;

const statusMapping: Record<string, Record<string, { label: string; color: string; backgroundColor: string }>> = {
    statusAcceptance: {
        accepted: { label: '承認済み', color: 'green', backgroundColor: '#e6f7e6' },
        pending: { label: '承認待ち', color: 'red', backgroundColor: '#fbeaea' },
    },
    statusAuthentication: {
        authenticated: { label: '認証済み', color: 'green', backgroundColor: '#e6f7e6' },
        not_authenticated: { label: '未認証', color: 'red', backgroundColor: '#fbeaea' },
    },
    statusFormConfirmation: {
        confirmed: { label: '確認済み', color: 'green', backgroundColor: '#e6f7e6' },
        not_confirmed: { label: '未確認', color: 'red', backgroundColor: '#fbeaea' },
    },
    statusRegistrationComplete: {
        completed: { label: '完了', color: 'green', backgroundColor: '#e6f7e6' },
        incomplete: { label: '未完了', color: 'red', backgroundColor: '#fbeaea' },
    },
};

const CircleRegister: React.FC = () => {
    const [organizations, setOrganizations] = useState<Organization[]>([]);
    const [loading, setLoading] = useState(true);
    const [filteredData, setFilteredData] = useState<Organization[]>([]);
    const [searchTerm, setSearchTerm] = useState<string>('');
    const [currentPage, setCurrentPage] = useState<number>(1);

    useEffect(() => {

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
    }, []);

    const getStatusLabel = (status: string, statusType: string) => {
        const statusInfo = statusMapping[statusType]?.[status];
        if (statusInfo) {
            return (
                <span style={{ color: statusInfo.color, backgroundColor: statusInfo.backgroundColor, padding: '2px 5px', borderRadius: '3px' }}>
                    {statusInfo.label}
                </span>
            );
        }
        // もしステータスが定義されていない場合
        return <span style={{ color: 'grey', backgroundColor: '#f0f0f0', padding: '2px 5px', borderRadius: '3px' }}>不明</span>;
    };

    const handleSearch = (value: string) => {
        setSearchTerm(value);
        const filtered = organizations.filter(org =>
            org.organizationName.toLowerCase().includes(value.toLowerCase())
        );
        setFilteredData(filtered);
        setCurrentPage(1);
    };

    const handleTableChange = (pagination: any) => {
        setCurrentPage(pagination.current);
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
            title: 'フォーム状況',
            dataIndex: 'statusFormConfirmation',
            key: 'statusFormConfirmation',
            render: (status: string) => getStatusLabel(status, 'statusFormConfirmation'),
        },
        {
            title: '登録状況',
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
                    <Title level={2}>団体登録状況</Title>
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
                        pagination={{
                            current: currentPage,
                            pageSize: pageSize,
                            total: filteredData.length,
                            onChange: (page) => setCurrentPage(page),
                        }}
                        onChange={handleTableChange}
                    />
                </div>
            </Content>
            <CustomFooter />
        </Layout>
    );
};

export default CircleRegister;
