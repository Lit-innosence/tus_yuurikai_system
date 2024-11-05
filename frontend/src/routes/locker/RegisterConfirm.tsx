import React, { useState, useEffect } from 'react';
import { useNavigate, useLocation } from 'react-router-dom';
import axios from 'axios';
import { Button, Layout, Card, Checkbox, message } from 'antd';
import CustomHeader from '../component/CustomHeader';
import CustomFooter from '../component/CustomFooter';
import constants from '../constants';

const { Content } = Layout;

const LockerRegisterConfirm: React.FC = () => {
    const navigate = useNavigate();
    const location = useLocation();
    const { lockerId, pairInfo: initialPairInfo, authToken } = location.state || {};

    const [isChecked, setIsChecked] = useState(false);
    const [pairInfo, setPairInfo] = useState(initialPairInfo);

    const mockPairInfo = {
        mainUser: {
            studentId: '9999999',
            familyName: '山田',
            givenName: '太郎',
        },
        coUser: {
            studentId: '8888888',
            familyName: '佐藤',
            givenName: '次郎',
        },
    };

    // pairInfoがundefinedの場合にモックデータをセット
    useEffect(() => {
        if (!initialPairInfo) {
            setPairInfo(mockPairInfo);
        }
    }, [initialPairInfo]);

    const handleCheckboxChange = (e: any) => {
        setIsChecked(e.target.checked);
    };

    const handleConfirm = async () => {
        const postData = {
            data: {
                studentId: pairInfo.mainUser.studentId,
                lockerId: lockerId,
            },
            authToken: authToken,
        };

        try {
            // POSTリクエストを実行
            await axios.post(`${constants.backendApiEndpoint}/api/locker/locker-register`, postData);
            message.success('ロッカーが正常に登録されました');
            navigate('/locker/register/complete', { state: { lockerId } });
        } catch (error) {
            console.error('エラー:', error);
            message.error('ロッカーの登録に失敗しました');
        }
    };

    return (
        <Layout style={{ minHeight: '100vh' }}>
            <CustomHeader />
            <Content style={{ padding: '50px', display: 'flex', justifyContent: 'center', alignItems: 'center' }}>
                <Card
                    title="入力内容の確認"
                    bordered={true}
                    style={{ width: '100%', maxWidth: '600px', textAlign: 'left' }}
                    headStyle={{ fontSize: '1.5em', textAlign: 'center' }}
                >
                    {pairInfo && (
                        <>
                            <h3>ロッカー情報</h3>
                            <p><strong>ロッカーID:</strong> {lockerId}</p>

                            <hr style={{ width: '100%', border: 'none', borderTop: '1px solid #e8e8e8', margin: '20px 0' }} />

                            <h3>申請者</h3>
                            <p><strong>学籍番号:</strong> {pairInfo.mainUser.studentId}</p>
                            <p><strong>氏名:</strong> {pairInfo.mainUser.familyName} {pairInfo.mainUser.givenName}</p>

                            <hr style={{ width: '100%', border: 'none', borderTop: '1px solid #e8e8e8', margin: '20px 0' }} />

                            <h3>共同利用者</h3>
                            <p><strong>学籍番号:</strong> {pairInfo.coUser.studentId}</p>
                            <p><strong>氏名:</strong> {pairInfo.coUser.familyName} {pairInfo.coUser.givenName}</p>

                            <div style={{ display: 'flex', justifyContent: 'center', marginTop: '20px' }}>
                                <Checkbox onChange={handleCheckboxChange}>
                                    入力内容を確認しました。
                                </Checkbox>
                            </div>

                            <div style={{ display: 'flex', justifyContent: 'center', marginTop: '30px' }}>
                                <Button type="primary" onClick={handleConfirm} disabled={!isChecked}>
                                    確認して登録
                                </Button>
                            </div>
                        </>
                    )}
                </Card>
            </Content>
            <CustomFooter />
        </Layout>
    );
};

export default LockerRegisterConfirm;
