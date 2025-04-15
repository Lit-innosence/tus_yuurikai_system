import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Checkbox, Button, Card, Layout, Typography } from 'antd';
import CustomHeader from '../../components/CustomHeader';
import CustomFooter from '../../components/CustomFooter';
import MovieButton from '../../components/MovieButton';

const { Content } = Layout;
const { Paragraph, Text } = Typography;

const ImportantConfirm: React.FC = () => {
    const navigate = useNavigate();
    
    // チェックボックスの状態を管理
    const [isChecked, setIsChecked] = useState(false);

    const handleCheckboxChange = (e: any) => {
        setIsChecked(e.target.checked);
    };

    const cardStyle: React.CSSProperties = {
        maxHeight: '100%',
        overflowY: 'scroll',
        padding: '16px',
    };

    const contentStyle: React.CSSProperties = {
        padding: '50px 50px',
        height: '70vh',
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'center',
        flexDirection: 'column',
    };

    return (
        <Layout style={{ minHeight: '100vh' }}>
            <CustomHeader />
            <MovieButton />
            <Content style={contentStyle}>
                <Card title="重要確認事項" bordered={true} style={cardStyle}>
                    <Paragraph strong style={{ fontSize: '16px', color: '#ff4d4f' }}>
                        廃棄日までに荷物を回収しない場合、荷物は廃棄されます。
                    </Paragraph>
                    <Paragraph>
                        期限までに荷物を回収しなかった場合、いかなる理由でも荷物は廃棄され、返還されません。
                        また、廃棄に伴う一切の責任は負いかねます。
                    </Paragraph>
                    <Paragraph style={{ fontWeight: 'bold', marginTop: '16px', color: 'black' }}>
                        ※ 廃棄に同意しない場合、ロッカーを申し込むことはできません。
                    </Paragraph>
                </Card>

                <Checkbox onChange={handleCheckboxChange} style={{ marginTop: '16px' }}>
                    期限までに回収しなかった場合、荷物が廃棄されることを了承します。
                </Checkbox>

                <Button
                    type="primary"
                    disabled={!isChecked}
                    style={{ marginTop: '16px' }}
                    onClick={() => { navigate('/locker/form'); }}
                >
                    同意して次に進む
                </Button>
            </Content>
            <CustomFooter />
        </Layout>
    );
};

export default ImportantConfirm;
