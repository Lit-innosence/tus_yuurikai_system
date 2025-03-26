import React from 'react';
import { Timeline, Layout, Card } from 'antd';
import CustomHeader from '../../components/CustomHeader';
import CustomFooter from '../../components/CustomFooter';
import MovieButton from '../../components/MovieButton';

const { Content } = Layout;

const Process: React.FC = () => {
    return (
        <Layout style={{ minHeight: '100vh' }}>
            <CustomHeader />
            <MovieButton />
            <Content style={{ display: 'flex', justifyContent: 'center', alignItems: 'center', padding: '50px', minHeight: '70vh' }}>
                <Card
                    style={{
                        width: '95%',
                        maxWidth: '800px',
                        backgroundColor: 'white',
                        boxShadow: '0 4px 12px rgba(0, 0, 0, 0.1)',
                        borderRadius: '10px',
                        padding: '30px',
                    }}
                >
                    <div style={{ display: 'flex', flexDirection: 'column', alignItems: 'flex-start', justifyContent: 'center', gap: '20px' }}>
                        <h2 style={{ textAlign: 'center', width: '100%', marginBottom: '10px' }}>ロッカー登録の流れ</h2>
                        <hr style={{ width: '100%', marginBottom: '30px', marginTop: '0', border: 'none', borderTop: '2px solid #e8e8e8' }} />
                        <Timeline
                            style={{ width: '100%' }}
                            items={[
                                {
                                    children: '利用規約に同意',
                                },
                                {
                                    children: 'フォームの入力',
                                },
                                {
                                    children: '申請者のメール認証',
                                },
                                {
                                    children: '共同利用者のメール認証',
                                },
                                {
                                    children: '申請者に認証完了のメール通知',
                                },
                                {
                                    children: 'ロッカー番号の選択',
                                },
                                {
                                    children: 'ロッカーの利用開始',
                                },
                            ]}
                        />
                    </div>
                    <p>認証メールは、[学籍番号]@ed.tus.ac.jp宛てに送られます。</p>
                    <p>※このページは閉じてもかまいません</p>
                </Card>
            </Content>
            <CustomFooter />
        </Layout>
    );
};

export default Process;
