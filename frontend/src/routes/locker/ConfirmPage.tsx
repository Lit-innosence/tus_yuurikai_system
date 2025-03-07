import React, { useState, useEffect } from 'react';
import { useNavigate, useLocation } from 'react-router-dom';
import axios from 'axios';
import { Button, Layout, Card, Checkbox, message } from 'antd';
import { useGoogleReCaptcha } from 'react-google-recaptcha-v3';
import CustomHeader from '../component/CustomHeader';
import CustomFooter from '../component/CustomFooter';
import constants from '../constants';

const { Content } = Layout;

const ConfirmPage: React.FC = () => {
    const navigate = useNavigate();
    const location = useLocation();
    const formData = location.state?.formData; 

    const { executeRecaptcha } = useGoogleReCaptcha(); // reCAPTCHA v3 のフック
    const [isChecked, setIsChecked] = useState(false); // チェックボックスの状態を管理
    const [loading, setLoading] = useState(false); // ローディング状態を管理
    const [lastClicked, setLastClicked] = useState<number | null>(null); // 最後のクリック時刻を記録する state

    // ページ読み込み時に formData が存在しなければ /nopage へ遷移
    useEffect(() => {
        if (!formData) {
            navigate('/locker/nopage');
        }
    }, [formData, navigate]);

    if (!formData) return null;

    // チェックボックスの状態が変更された時の処理
    const handleCheckboxChange = (e: any) => {
        setIsChecked(e.target.checked);
    };

    // 確認ボタンがクリックされた時の処理
    const handleConfirm = async () => {

        setLoading(true);

        // reCAPTCHA v3 を実行してトークンを取得
        if (!executeRecaptcha) {
            message.error("reCAPTCHAがまだ読み込まれていません。");
            return;
        }
        const token = await executeRecaptcha('confirm_page');

        // クールダウンタイムの確認
        const now = Date.now();
        if (lastClicked && now - lastClicked < 20000) {
            message.warning('20秒のクールダウン中です。しばらくお待ちください。');
            return;
        }
        setLastClicked(now);

        // データの整形 (reCAPTCHAトークンを含む)
        const formattedData = {
            data: {
                mainUser: {
                    studentId: formData.studentId,
                    familyName: formData.lastName,
                    givenName: formData.firstName,
                },
                coUser: {
                    studentId: formData.coUserStudentId,
                    familyName: formData.coUserLastName,
                    givenName: formData.coUserFirstName,
                },
            },
            recaptchaToken: token,
        };

        try {
            await axios.post(`${constants.backendApiEndpoint}/api/locker/token-gen`, formattedData);
            message.success('フォームが正常に送信されました');
            navigate('/locker/form/complete');
        } catch (error) {
            message.error('送信に失敗しました');
        } finally {
            setLoading(false);
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
                    <h3>申請者</h3>
                    <p><strong>学籍番号:</strong> {formData.studentId}</p>
                    <p><strong>氏名:</strong> {formData.lastName} {formData.firstName}</p>
                    
                    <hr style={{ width: '100%', border: 'none', borderTop: '1px solid #e8e8e8', margin: '20px 0' }} />

                    <h3>共同利用者</h3>
                    <p><strong>学籍番号:</strong> {formData.coUserStudentId}</p>
                    <p><strong>氏名:</strong> {formData.coUserLastName} {formData.coUserFirstName}</p>
                    
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
                </Card>
            </Content>
            <CustomFooter />
        </Layout>
    );
};

export default ConfirmPage;
