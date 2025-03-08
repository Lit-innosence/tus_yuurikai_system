import React, { useEffect, useState } from 'react';
import { List, Button, Card, Typography, Select, Layout } from 'antd';
import axios from 'axios';
import { useNavigate, useLocation } from 'react-router-dom';
import CustomHeader from '../../component/CustomHeader';
import CustomFooter from '../../component/CustomFooter';
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
    const authId = location.state?.authId;

    useEffect(() => {
        const fetchData = async (floor: string) => {
            try {
                let response;
                if (floor === 'all') {
                    // floorパラメータなしでリクエスト
                    response = await axios.get(`${constants.backendApiEndpoint}/api/locker/availability`);
                } else {
                    // floorパラメータを指定してリクエスト
                    response = await axios.get(`${constants.backendApiEndpoint}/api/locker/availability?floor=${floor}`);
                }

                // レスポンスのデータをフロントエンドの形式に変換
                const filteredLockers = response.data.data.map((locker: any) => ({
                    lockerId: locker.lockerId,
                    floor: locker.floor.toString(),
                    status: locker.status.toLowerCase(),
                }));

                setLockers(filteredLockers);
                setLoading(false);
            } catch (error) {
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
            navigate('/locker/register/confirm', { state: { lockerId: selectedLocker, pairInfo, authId } });
        }
    };

    return (
        <Layout style={{ minHeight: '100vh', display: 'flex', flexDirection: 'column' }}>
            <CustomHeader />
            <Content style={{ flex: 1, padding: '50px 50px', display: 'flex', flexDirection: 'column', alignItems: 'center' }}>
                <Card style={{ maxWidth: 800, width: '100%', flexGrow: 1 }}>
                    <Title level={3}>ロッカーを選択してください</Title>

                    <Select
                        value={selectedFloor}
                        onChange={handleFloorChange}
                        style={{ width: 200, marginBottom: 20 }}
                    >
                        <Option value="all">全フロア</Option>
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
                </Card>
            </Content>

            {selectedLocker && (
                <div style={{
                    position: 'fixed',
                    bottom: 0,
                    left: 0,
                    width: '100%',
                    background: '#fff',
                    padding: '10px 20px',
                    boxShadow: '0 -2px 5px rgba(0,0,0,0.1)',
                    textAlign: 'center',
                }}>
                    <Button type="primary" block onClick={handleRegister} style={{ maxWidth: 800 }}>
                        ロッカー登録
                    </Button>
                </div>
            )}

            <CustomFooter />
        </Layout>
    );
};

export default Register;
