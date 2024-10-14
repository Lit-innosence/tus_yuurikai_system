import React, { useState, useEffect } from 'react';
import { useNavigate, useLocation } from 'react-router-dom';
import axios from 'axios';
import { Button, Layout, Card, Checkbox, message } from 'antd';
import CustomHeader from '../component/CustomHeader';
import CustomFooter from '../component/CustomFooter';

const { Content } = Layout;

const LockerRegisterConfirm: React.FC = () => {
    const navigate = useNavigate();
    const location = useLocation();
    const { lockerId, pairInfo } = location.state || {};

    const [isChecked, setIsChecked] = useState(false);

    // pairInfoがundefinedの場合に/nopageにリダイレクト
    useEffect(() => {
        if (!pairInfo) {
            navigate('/nopage');
        }
    }, [pairInfo, navigate]);

    const handleCheckboxChange = (e: any) => {
        setIsChecked(e.target.checked);
    };

    const handleConfirm = async () => {
        const postData = {
            data: {
                student_id: pairInfo.main_user.student_id,
                locker_id: lockerId,
            },
        };

        try {
            // POSTリクエストを実行
            await axios.post('/locker/locker-register', postData);
            message.success('ロッカーが正常に登録されました');
            navigate('/locker/register/complete');
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
                            <h3>申請者</h3>
                            <p><strong>学籍番号:</strong> {pairInfo.main_user.student_id}</p>
                            <p><strong>氏名:</strong> {pairInfo.main_user.family_name} {pairInfo.main_user.given_name}</p>
                            <p><strong>ロッカーID:</strong> {lockerId}</p>
                            
                            <hr style={{ width: '100%', border: 'none', borderTop: '1px solid #e8e8e8', margin: '20px 0' }} />

                            <h3>共同利用者</h3>
                            <p><strong>学籍番号:</strong> {pairInfo.co_user.student_id}</p>
                            <p><strong>氏名:</strong> {pairInfo.co_user.family_name} {pairInfo.co_user.given_name}</p>

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
