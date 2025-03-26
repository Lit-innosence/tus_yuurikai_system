import React from 'react';
import { useLocation } from 'react-router-dom';
import { Result, Typography, Card } from 'antd';
import { NotificationOutlined, LockOutlined, FieldNumberOutlined } from '@ant-design/icons';

const { Paragraph, Text, Link } = Typography;

const RegisterComp: React.FC = () => {
    const location = useLocation();
    const { lockerId } = location.state || {}; // Retrieve lockerId from location state

    return (
        <div style={{ display: 'flex', justifyContent: 'center', alignItems: 'center', minHeight: '100vh', textAlign: 'center' }}>
            <Result
                status="success"
                title={<Text strong style={{ fontSize: '20px' }}>ロッカー登録に成功しました！</Text>} // フォントサイズ統一
                subTitle={
                    <div style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}>
                        {/* ロッカー番号 (1行で表示) */}
                        <Card bordered style={{ padding: '16px', textAlign: 'left', display: 'flex', alignItems: 'center', marginTop: '24px' }}>
                            <Text strong style={{ fontSize: '18px' }}>ロッカー番号：</Text>
                            <Text strong style={{ fontSize: '18px', color: '#1890ff', marginLeft: '8px' }}>
                                {lockerId}
                            </Text>
                        </Card>

                        {/* 内容物の回収・保管・廃棄について */}
                        <Card bordered style={{ padding: '16px', textAlign: 'left', backgroundColor: '#fafafa' }}>
                            <Paragraph strong style={{ fontSize: '18px' }}>
                                <NotificationOutlined style={{ marginRight: '12px' }} />
                                内容物の回収・保管・廃棄日について
                            </Paragraph>
                            <ul style={{ paddingLeft: '16px', margin: 0 }}>
                                <li><Text strong>内容物回収期間：</Text>3月中旬</li>
                                <li><Text strong>保管期間：</Text>次年度の4~6月</li>
                                <li><Text strong>廃棄日：</Text>次年度の6月下旬</li>
                            </ul>
                        </Card>

                        {/* カギについて */}
                        <Card bordered style={{ padding: '16px', textAlign: 'left' }}>
                            <Paragraph strong style={{ fontSize: '18px', color: '#ff4d4f' }}>
                                <LockOutlined style={{ marginRight: '12px' }} />
                                ロッカー使用時には必ず鍵を使用してください。
                            </Paragraph>
                            <Paragraph>
                                鍵の購入はこちら：
                                <Link href="https://www.univcoop.jp/rikadai/time/index.html#s02" target="_blank" style={{ fontWeight: 'bold' }}>
                                    東京理科大学消費生活協同組合 葛飾店
                                </Link>
                            </Paragraph>
                            <Paragraph>
                            在庫が無い場合もあります。その場合は他店でご購入ください。
                            </Paragraph>
                        </Card>
                    </div>
                }
            />
        </div>
    );
};

export default RegisterComp;
