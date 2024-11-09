import React from 'react';
import { Timeline, Layout, Card } from 'antd';
import CustomHeader from '../component/CustomHeader';
import CustomFooter from '../component/CustomFooter';

const { Content } = Layout;

const Process: React.FC = () => {
    return (
        <Layout style={{ minHeight: '100vh' }}>
            <CustomHeader />
            <Content style={{ display: 'flex', justifyContent: 'center', alignItems: 'center', padding: '50px', minHeight: '70vh' }}>
                <Card
                    style={{
                        width: '60%',
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
                                    children: 'システムフォームの入力',
                                },
                                {
                                    children: '現在の代表のメール認証',
                                },
                                {
                                    children: 'GoogleFormの入力',
                                },
                                {
                                    children: '新代表者のメール認証',
                                },
                                {
                                    children: '新副代表のメール認証',
                                },
                                {
                                    children: '更新完了通知',
                                },
                            ]}
                        />
                    </div>
                    <p>認証メールは、指定したメールアドレスに送信されます。</p>
                    <p>※このページは閉じてもかまいません</p>
                </Card>
            </Content>
            <CustomFooter />
        </Layout>
    );
};

export default Process;
