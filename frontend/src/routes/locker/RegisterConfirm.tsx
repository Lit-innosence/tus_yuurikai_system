import React, { useState, useEffect } from 'react';
import { useNavigate, useLocation } from 'react-router-dom';
import axios from 'axios';
import { Button, Layout, Card, Checkbox, message } from 'antd';
import CustomHeader from '../../components/CustomHeader';
import CustomFooter from '../../components/CustomFooter';
import MovieButton from '../../components/MovieButton';
import constants from '../constants';

const { Content } = Layout;

const LockerRegisterConfirm: React.FC = () => {
    const navigate = useNavigate();
    const location = useLocation();
    const { lockerId, pairInfo: initialPairInfo, authId } = location.state || {};

    const [isChecked, setIsChecked] = useState(false); // チェックボックスの状態を管理
    const [loading, setLoading] = useState(false); // ローディング状態を管理
    const [pairInfo] = useState(initialPairInfo); // ペア情報を state で管理
    const [lastClicked, setLastClicked] = useState<number | null>(null); // 最後のクリック時刻を記録する state

    // ページ読み込み時に必要なデータが存在しなければ /locker/nopage にリダイレクト
    useEffect(() => {
        if (!lockerId || !initialPairInfo || !authId) {
            navigate('/locker/nopage');
        }
    }, [lockerId, initialPairInfo, authId, navigate]);

    // データが不足している場合は何もレンダリングしない
    if (!lockerId || !initialPairInfo || !authId) {
        return null;
    }

    const handleCheckboxChange = (e: any) => {
        setIsChecked(e.target.checked);
    };

    const handleConfirm = async () => {

        setLoading(true); // Loading状態にする
        
        // クールダウンタイムの確認
        const now = Date.now();
        if (lastClicked && now - lastClicked < 20000) {
            message.warning('20秒のクールダウン中です。しばらくお待ちください。');
            return;
        }
        setLastClicked(now);

        // データの整形
        const postData = {
            data: {
                studentId: pairInfo.mainUser.studentId,
                lockerId: lockerId,
            },
            authId: authId,
        };

        try {
            // POSTリクエストを実行
            await axios.post(`${constants.backendApiEndpoint}/api/locker/locker-register`, postData);
            message.success('ロッカーが正常に登録されました');
            navigate('/locker/register/complete', { state: { lockerId } });
        } catch (error) {
            message.error('ロッカーの登録に失敗しました');
        } finally {
            setLoading(false); 
        }
    };

    return (
        <Layout style={{ minHeight: '100vh' }}>
            <CustomHeader />
            <MovieButton />
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
                                <Button 
                                    type="primary" 
                                    onClick={handleConfirm} 
                                    disabled={!isChecked || loading} 
                                    loading={loading}
                                >
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
