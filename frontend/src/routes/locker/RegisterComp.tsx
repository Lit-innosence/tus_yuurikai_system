import React from 'react';
import { useLocation } from 'react-router-dom';
import { Result, Typography, Card } from 'antd';
import { NotificationOutlined } from '@ant-design/icons';

const { Paragraph, Text, Link } = Typography;

const RegisterComp: React.FC = () => {
    const location = useLocation();
    const { lockerId } = location.state || {}; // Retrieve lockerId from location state

    return (
        <div style={{ display: 'flex', justifyContent: 'center', alignItems: 'center', minHeight: '100vh', textAlign: 'center' }}>
            <Result
                status="success"
                title={<div style={{ marginBottom: '24px' }}>ロッカー登録に成功しました！</div>} // 上にスペースを開ける
                subTitle={
                    <Card bordered style={{ padding: '16px', display: 'inline-block', textAlign: 'left' }}>
                        <Text strong style={{ fontSize: '18px', color: '#1890ff' }}>
                            ロッカー番号: {lockerId}
                        </Text>
                        <br />
                        <Card bordered style={{ marginTop: '16px', backgroundColor: '#fafafa', padding: '12px' }}>
                            <Paragraph strong style={{ fontSize: '18px', marginBottom: '8px' }}>
                            <NotificationOutlined style={{ marginRight: '12px'}}/>
                                荷物回収・保管・廃棄日について
                            </Paragraph>
                            <ul style={{ paddingLeft: '16px', margin: 0 }}>
                                <li><Text strong>荷物回収期間：</Text>3月中旬</li>
                                <li><Text strong>保管期間：</Text>次年度の4~6月</li>
                                <li><Text strong>廃棄日：</Text>次年度の6月下旬</li>
                            </ul>
                        </Card>
                        <Paragraph strong style={{ fontSize: '16px', marginTop: '16px', color: '#ff4d4f' }}>
                            ロッカー使用時には必ず鍵を使用してください。
                        </Paragraph>
                        <Paragraph>
                            鍵の購入はこちら：
                            <Link href="[URL]" target="_blank" style={{ fontWeight: 'bold' }}>
                                [URL]
                            </Link>
                        </Paragraph>
                    </Card>
                }
            />
        </div>
    );
};

export default RegisterComp;
