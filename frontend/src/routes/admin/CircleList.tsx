import React, { useEffect, useState } from 'react';
import { Table, Typography, Layout, Input, message, Modal, Button, Select } from 'antd';
import { ExportOutlined } from '@ant-design/icons';
import axios from 'axios'; 
import CustomHeader from '../component/CustomHeader';
import CustomFooter from '../component/CustomFooter';

const { Title } = Typography;
const { Content } = Layout;
const { Search } = Input;
const { Option } = Select;

interface Organization {
    organizationId: string;
    organizationName: string;
    organizationEmail: string;
    mainId: string;
    mainFamilyName: string;
    mainGivenName: string;
    mainEmail: string;
    mainPhone: string;
    coId: string;
    coFamilyName: string;
    coGivenName: string;
    coEmail: string;
    coPhone: string;
    bUrl: string;
    cUrl: string;
    dUrl: string;
    statusAcceptance: string;
    statusAuthentication: string;
    statusFormConfirmation: string;
    statusRegistrationComplete: string;
}

const pageSize = 5;
const baseUrl = "https://drive.google.com/file/d/";
const urlSuffix = "/view";

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

const CircleList: React.FC = () => {
    const [organizations, setOrganizations] = useState<Organization[]>([]);
    const [loading, setLoading] = useState(true);
    const [filteredData, setFilteredData] = useState<Organization[]>([]);
    const [searchTerm, setSearchTerm] = useState<string>('');
    const [currentPage, setCurrentPage] = useState<number>(1);

    // モーダル制御用のstate
    const [isModalOpen, setIsModalOpen] = useState(false);
    const [selectedOrg, setSelectedOrg] = useState<Organization | null>(null);

    // 選択メニュー用の一時ステータス
    const [tempStatusAcceptance, setTempStatusAcceptance] = useState<string>('pending');
    const [tempStatusAuthentication, setTempStatusAuthentication] = useState<string>('not_authenticated');
    const [tempStatusFormConfirmation, setTempStatusFormConfirmation] = useState<string>('not_confirmed');
    const [tempStatusRegistrationComplete, setTempStatusRegistrationComplete] = useState<string>('incomplete');

    useEffect(() => {
        // 本来はこちらのAPIから取得
        // const fetchOrganizations = async () => {
        //     try {
        //         const response = await axios.get<{ data: Organization[] }>('/api/admin/circle/list', { withCredentials: true });
        //         setOrganizations(response.data.data);
        //         setFilteredData(response.data.data);
        //     } catch (error) {
        //         message.error('データの取得に失敗しました。');
        //     } finally {
        //         setLoading(false);
        //     }
        // };
        // fetchOrganizations();

        // モックデータ
        const mockData: Organization[] = [
            {
                organizationId: 'C00001',
                organizationName: '団体A',
                organizationEmail: 'a@example.com',
                mainId: '1001',
                mainFamilyName: '田中',
                mainGivenName: '太郎',
                mainEmail: 'tanaka@example.com',
                mainPhone: '090-1234-5678',
                coId: '2001',
                coFamilyName: '佐藤',
                coGivenName: '花子',
                coEmail: 'sato@example.com',
                coPhone: '080-9876-5432',
                bUrl: '14Zh77eFIDdmFKuBWioE-GaeNsivF0x5E',
                cUrl: '1RcbGGaPWwXv1lbfp6Ju8kJ1CQ60IF2eD',
                dUrl: '19gAhh-rNTgws7rmm8aEj_04Y0fm-IQD2',
                statusAcceptance: 'accepted',
                statusAuthentication: 'authenticated',
                statusFormConfirmation: 'confirmed',
                statusRegistrationComplete: 'completed',
            },
            {
                organizationId: 'C00002',
                organizationName: '団体B',
                organizationEmail: 'b@example.com',
                mainId: '1002',
                mainFamilyName: '山田',
                mainGivenName: '次郎',
                mainEmail: 'yamada@example.com',
                mainPhone: '090-5678-1234',
                coId: '2002',
                coFamilyName: '高橋',
                coGivenName: '三郎',
                coEmail: 'takahashi@example.com',
                coPhone: '080-4321-8765',
                bUrl: 'b-file-b',
                cUrl: 'c-file-b',
                dUrl: 'd-file-b',
                statusAcceptance: 'pending',
                statusAuthentication: 'not_authenticated',
                statusFormConfirmation: 'not_confirmed',
                statusRegistrationComplete: 'incomplete',
            },
        ];

        setTimeout(() => {
            setOrganizations(mockData);
            setFilteredData(mockData);
            setLoading(false);
        }, 1000);
    }, []);

    const getStatusLabel = (status: string, statusType: string) => {
        const statusInfo = statusMapping[statusType]?.[status];
        if (statusInfo) {
            return (
                <span
                    style={{
                        color: statusInfo.color,
                        backgroundColor: statusInfo.backgroundColor,
                        padding: '2px 5px',
                        borderRadius: '3px',
                    }}
                >
                    {statusInfo.label}
                </span>
            );
        }
        return (
            <span
                style={{
                    color: 'grey',
                    backgroundColor: '#f0f0f0',
                    padding: '2px 5px',
                    borderRadius: '3px',
                }}
            >
                不明
            </span>
        );
    };

    const handleSearch = (value: string) => {
        setSearchTerm(value);
        const filtered = organizations.filter((org) =>
            org.organizationName.toLowerCase().includes(value.toLowerCase())
        );
        setFilteredData(filtered);
        setCurrentPage(1);
    };

    const handleTableChange = (pagination: any) => {
        setCurrentPage(pagination.current);
    };

    // モーダルを開く関数
    const handleOpenModal = (record: Organization) => {
        setSelectedOrg(record);
        // 既存のステータスをそのままセット
        setTempStatusAcceptance(record.statusAcceptance);
        setTempStatusAuthentication(record.statusAuthentication);
        setTempStatusFormConfirmation(record.statusFormConfirmation);
        setTempStatusRegistrationComplete(record.statusRegistrationComplete);
        setIsModalOpen(true);
    };

    // モーダルを閉じる関数
    const handleCancel = () => {
        setIsModalOpen(false);
    };

    // 保存ボタンクリック時：APIにデータ送信
    const handleSave = async () => {
        if (!selectedOrg) return;

        try {
            // 実際にはAPIのURLを修正して利用してください
            await axios.post(
                '/api/admin/circle/status/update',
                {
                    ...selectedOrg,
                    statusAcceptance: tempStatusAcceptance,
                    statusAuthentication: tempStatusAuthentication,
                    statusFormConfirmation: tempStatusFormConfirmation,
                    statusRegistrationComplete: tempStatusRegistrationComplete,
                },
                { withCredentials: true }
            );
            message.success('ステータスを更新しました');
        } catch (error) {
            message.error('更新に失敗しました');
        } finally {
            setIsModalOpen(false);
        }
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
            title: '団体メールアドレス',
            dataIndex: 'organizationEmail',
            key: 'organizationEmail',
        },
        {
            title: '代表者情報',
            key: 'mainInfo',
            render: (record: Organization) =>
                `${record.mainFamilyName} ${record.mainGivenName} (${record.mainEmail}, ${record.mainPhone})`,
        },
        {
            title: '副代表者情報',
            key: 'coInfo',
            render: (record: Organization) =>
                `${record.coFamilyName} ${record.coGivenName} (${record.coEmail}, ${record.coPhone})`,
        },
        {
            title: 'B用紙',
            key: 'bUrl',
            render: (record: Organization) => (
                <a href={`${baseUrl}${record.bUrl}${urlSuffix}`} target="_blank" rel="noopener noreferrer">
                    URL <ExportOutlined />
                </a>
            ),
        },
        {
            title: 'C用紙',
            key: 'cUrl',
            render: (record: Organization) => (
                <a href={`${baseUrl}${record.cUrl}${urlSuffix}`} target="_blank" rel="noopener noreferrer">
                    URL <ExportOutlined />
                </a>
            ),
        },
        {
            title: 'D用紙',
            key: 'dUrl',
            render: (record: Organization) => (
                <a href={`${baseUrl}${record.dUrl}${urlSuffix}`} target="_blank" rel="noopener noreferrer">
                    URL <ExportOutlined />
                </a>
            ),
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
        {
            title: 'ステータス更新',
            key: 'updateStatus',
            render: (record: Organization) => (
                <Button type="primary" onClick={() => handleOpenModal(record)}>
                    更新
                </Button>
            ),
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
                    overflowX: 'auto',
                }}
            >
                <div style={{ width: '100%', overflowX: 'auto' }}>
                    <Title level={2}>団体一覧</Title>
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
                        scroll={{ x: 'max-content' }}
                    />
                </div>

                {/* ステータス更新用モーダル */}
                <Modal
                    title="ステータス更新"
                    visible={isModalOpen}
                    onCancel={handleCancel}
                    footer={null}
                >
                    {selectedOrg && (
                        <>
                            <p>団体名: {selectedOrg.organizationName}</p>
                            {/* 承認状況 */}
                            <div style={{ marginBottom: 10 }}>
                                <span style={{ display: 'inline-block', width: 120 }}>承認状況</span>
                                <Select
                                    value={tempStatusAcceptance}
                                    onChange={setTempStatusAcceptance}
                                    style={{ width: 200 }}
                                >
                                    <Option value="accepted">承認済み</Option>
                                    <Option value="pending">承認待ち</Option>
                                </Select>
                            </div>

                            {/* 認証状況 */}
                            <div style={{ marginBottom: 10 }}>
                                <span style={{ display: 'inline-block', width: 120 }}>認証状況</span>
                                <Select
                                    value={tempStatusAuthentication}
                                    onChange={setTempStatusAuthentication}
                                    style={{ width: 200 }}
                                >
                                    <Option value="authenticated">認証済み</Option>
                                    <Option value="not_authenticated">未認証</Option>
                                </Select>
                            </div>

                            {/* フォーム状況 */}
                            <div style={{ marginBottom: 10 }}>
                                <span style={{ display: 'inline-block', width: 120 }}>フォーム状況</span>
                                <Select
                                    value={tempStatusFormConfirmation}
                                    onChange={setTempStatusFormConfirmation}
                                    style={{ width: 200 }}
                                >
                                    <Option value="confirmed">確認済み</Option>
                                    <Option value="not_confirmed">未確認</Option>
                                </Select>
                            </div>

                            {/* 登録状況 */}
                            <div style={{ marginBottom: 20 }}>
                                <span style={{ display: 'inline-block', width: 120 }}>登録状況</span>
                                <Select
                                    value={tempStatusRegistrationComplete}
                                    onChange={setTempStatusRegistrationComplete}
                                    style={{ width: 200 }}
                                >
                                    <Option value="completed">完了</Option>
                                    <Option value="incomplete">未完了</Option>
                                </Select>
                            </div>

                            <div style={{ textAlign: 'right' }}>
                                <Button onClick={handleCancel} style={{ marginRight: 8 }}>
                                    キャンセル
                                </Button>
                                <Button type="primary" onClick={handleSave}>
                                    保存
                                </Button>
                            </div>
                        </>
                    )}
                </Modal>
            </Content>
            <CustomFooter />
        </Layout>
    );
};

export default CircleList;
