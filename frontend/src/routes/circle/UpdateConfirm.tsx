import React, { useState, useEffect } from 'react';
import { useNavigate, useLocation } from 'react-router-dom';
import { Button, Layout, Card, Checkbox, message } from 'antd';
import axios from 'axios';
import CustomHeader from '../../components/CustomHeader';
import CustomFooter from '../../components/CustomFooter';

const { Content } = Layout;

const CircleUpdateConfirm: React.FC = () => {
    const navigate = useNavigate();
    const location = useLocation();
    const formData = location.state?.formData;

    const [isChecked, setIsChecked] = useState(false);
    const [loading, setLoading] = useState(false);
    const [lastClicked, setLastClicked] = useState<number | null>(null);

    useEffect(() => {
        if (!formData) {
            navigate('/circle/nopage');
        }
    }, [formData, navigate]);

    if (!formData) {
        return null;
    }

    const handleCheckboxChange = (e: any) => {
        setIsChecked(e.target.checked);
    };

    const handleConfirm = async () => {
        const now = Date.now();
        if (lastClicked && now - lastClicked < 20000) {
            message.warning('20秒のクールダウン中です。しばらくお待ちください。');
            return;
        }
        setLastClicked(now);
        setLoading(true);
        try {
            const response = await axios.post('/api/circle/update/entry', formData);
            if (response.status === 200) {
                message.success('フォームの提出に成功しました');
                navigate('/circle/update/complete');
            }
        } catch (error) {
            message.error('フォームの提出に失敗しました');
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
                    <h3>団体情報</h3>
                    <p><strong>団体ID:</strong> {formData.organizationId}</p>
                    <p><strong>団体名:</strong> {formData.organizationName}</p>

                    <hr style={{ width: '100%', border: 'none', borderTop: '1px solid #e8e8e8', margin: '20px 0' }} />

                    <h3>旧代表者情報</h3>
                    <p><strong>代表者名:</strong> {formData.familyName} {formData.givenName}</p>
                    <p><strong>代表者の学籍番号:</strong> {formData.studentId}</p>
                    <p><strong>代表者のメールアドレス:</strong> {formData.email}</p>

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
                            確認して更新
                        </Button>
                    </div>
                </Card>
            </Content>
            <CustomFooter />
        </Layout>
    );
};

export default CircleUpdateConfirm;
