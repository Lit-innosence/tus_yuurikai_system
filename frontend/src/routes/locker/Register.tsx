import React, { useEffect, useState } from 'react';
import { List, Button, Card, Typography, Select, Layout } from 'antd';
import axios from 'axios';
import { useNavigate, useLocation } from 'react-router-dom';
import CustomHeader from '../component/CustomHeader';
import CustomFooter from '../component/CustomFooter';
import Loading from '../Loading';
import constants from '../constants';

const { Title } = Typography;
const { Option } = Select;
const { Content } = Layout;

interface Locker {
    lockerId: string;
    floor: string;
    status: 'vacant' | 'occupied' | 'out-of-work';
}

const Register: React.FC = () => {
    const [lockers, setLockers] = useState<Locker[]>([]);
    const [selectedLocker, setSelectedLocker] = useState<string | null>(null);
    const [loading, setLoading] = useState<boolean>(true);
    const [selectedFloor, setSelectedFloor] = useState<string>('all');
    const navigate = useNavigate();
    const location = useLocation();
    const pairInfo = location.state?.pairInfo;

    useEffect(() => {
        const fetchData = async (floor: string) => {
            try {
                let response;
                if (floor === 'all') {
                    // floorパラメータなしでリクエスト
                    response = await axios.post(`${constants.backendApiEndpoint}/api/locker/availability`);
                } else {
                    // floorパラメータを指定してリクエスト
                    response = await axios.post(`${constants.backendApiEndpoint}/api/locker/availability?floor=${floor}`);
                }
                
                // レスポンスのデータをフロントエンドの形式に変換
                const filteredLockers = response.data.data.map((locker: any) => ({
                    lockerId: locker.locker_id,
                    floor: locker.floor.toString(),
                    status: locker.status.toLowerCase(),
                }));

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
                        <Option value="all">全部</Option>
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
                                        backgroundColor: locker.status === 'vacant' ? '#d0f0c0' : 'transparent',
                                        padding: '10px',
                                        marginBottom: '10px',
                                        borderRadius: '5px',
                                        border: '1px solid #e0e0e0',
                                        opacity: locker.status !== 'vacant' ? 0.5 : 1,
                                        color: locker.status !== 'vacant' ? '#A9A9A9' : 'inherit',
                                    }}
                                    actions={[
                                        locker.status === 'vacant' ? (
                                            <Button
                                                type={selectedLocker === locker.lockerId ? 'primary' : 'default'}
                                                onClick={() => handleLockerSelect(locker.lockerId)}
                                            >
                                                選択
                                            </Button>
                                        ) : (
                                            <Button disabled>{locker.status === 'occupied' ? '使用中' : '故障中'}</Button>
                                        ),
                                    ]}
                                >
                                    <List.Item.Meta
                                        title={`ロッカー番号: ${locker.lockerId} 　|　 状態: ${locker.status === 'vacant' ? '空き' : locker.status === 'occupied' ? '使用中' : '故障中'}`}
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
