import React, { useEffect, useState } from 'react';
import { List, Button, Card, Typography, Select, Layout } from 'antd';
import axios from 'axios';
import { useNavigate, useLocation } from 'react-router-dom';
import CustomHeader from '../component/CustomHeader';
import CustomFooter from '../component/CustomFooter';
import Loading from '../Loading';

const { Title } = Typography;
const { Option } = Select;
const { Content } = Layout;

interface Locker {
    locker_id: string;
    floor: string;
    status: 'VACANT' | 'OCCUPIED' | 'OUT-OF-WORK';
}

// モックデータ
const mockResponse = {
    data: [
        {
            locker_id: '1001',
            floor: '1',
            status: 'VACANT' as 'VACANT',
        },
        {
            locker_id: '1002',
            floor: '1',
            status: 'OCCUPIED' as 'OCCUPIED',
        },
        {
            locker_id: '2001',
            floor: '2',
            status: 'VACANT' as 'VACANT',
        },
        {
            locker_id: '2002',
            floor: '2',
            status: 'OCCUPIED' as 'OCCUPIED',
        },
        {
            locker_id: '2003',
            floor: '2',
            status: 'OUT-OF-WORK' as 'OUT-OF-WORK',
        },
    ],
};

const Register: React.FC = () => {
    const [lockers, setLockers] = useState<Locker[]>([]);
    const [selectedLocker, setSelectedLocker] = useState<string | null>(null);
    const [loading, setLoading] = useState<boolean>(true);
    const [selectedFloor, setSelectedFloor] = useState<string>('1');
    const navigate = useNavigate();
    const location = useLocation();
    const pairInfo = location.state?.pairInfo;

    useEffect(() => {
        const fetchData = async (floor: string) => {
            try {
                const response = mockResponse;
                const filteredLockers = response.data.filter(locker => locker.floor === floor);
                setLockers(filteredLockers);
                setLoading(false);
            } catch (error) {
                console.error('Error fetching locker data:', error);
                setLoading(false);
            }
        };

        fetchData(selectedFloor);
    }, [selectedFloor]);

    const handleFloorChange = (floor: string) => {
        setSelectedFloor(floor);
        setLoading(true);
    };

    const handleLockerSelect = (lockerId: string) => {
        setSelectedLocker(lockerId);
    };

    const handleRegister = () => {
        if (selectedLocker) {
            navigate('/locker/register/confirm', { state: { lockerId: selectedLocker, pairInfo } });
        }
    };

    return (
        <Layout style={{ minHeight: '100vh' }}>
            <CustomHeader />
            <Content style={{ padding: '50px 50px' }}>
                <Card style={{ maxWidth: 800, margin: '20px auto' }}>
                    <Title level={3}>ロッカーを選択してください</Title>

                    <Select
                        value={selectedFloor}
                        onChange={handleFloorChange}
                        style={{ width: 200, marginBottom: 20 }}
                    >
                        <Option value="1">1階</Option>
                        <Option value="2">2階</Option>
                        <Option value="3">3階</Option>
                        <Option value="4">4階</Option>
                        <Option value="5">5階</Option>
                        <Option value="6">6階</Option>
                    </Select>

                    {loading ? (
                        <Loading />
                    ) : (
                        <List
                            dataSource={lockers}
                            renderItem={(locker) => (
                                <List.Item
                                    style={{
                                        backgroundColor: locker.status === 'VACANT' ? '#d0f0c0' : 'transparent',
                                        padding: '10px',
                                        marginBottom: '10px',
                                        borderRadius: '5px',
                                        border: '1px solid #e0e0e0',
                                        opacity: locker.status !== 'VACANT' ? 0.5 : 1, // 使用できないロッカーの不透明度を下げる
                                        color: locker.status !== 'VACANT' ? '#A9A9A9' : 'inherit', // 使用できないロッカーを薄いグレーに
                                    }}
                                    actions={[
                                        locker.status === 'VACANT' ? (
                                            <Button
                                                type={selectedLocker === locker.locker_id ? 'primary' : 'default'}
                                                onClick={() => handleLockerSelect(locker.locker_id)}
                                            >
                                                選択
                                            </Button>
                                        ) : (
                                            <Button disabled>{locker.status === 'OCCUPIED' ? '使用中' : '故障中'}</Button>
                                        ),
                                    ]}
                                >
                                    <List.Item.Meta
                                        title={`ロッカー番号: ${locker.locker_id} 　|　 状態: ${locker.status === 'VACANT' ? '空き' : locker.status === 'OCCUPIED' ? '使用中' : '故障中'}`}
                                    />
                                </List.Item>
                            )}
                        />
                    )}

                    {selectedLocker && (
                        <Button type="primary" block onClick={handleRegister}>
                            ロッカー登録
                        </Button>
                    )}
                </Card>
            </Content>
            <CustomFooter />
        </Layout>
    );
};

export default Register;
