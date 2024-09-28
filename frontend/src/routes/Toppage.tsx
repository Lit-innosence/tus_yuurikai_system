import React from 'react';
import { useNavigate } from 'react-router-dom';
import { Layout, List, Button } from 'antd';

type ToppageProps = {
    children?: React.ReactNode;
};

const content = [
    { title: 'ロッカー空き検索', route: '/locker-terms' },
    { title: 'サークル団体登録', route: '/circle' },
];

const { Header, Content, Footer } = Layout;

const Toppage: React.FC<ToppageProps> = (props) => {
    const navigate = useNavigate();

    return (
    <React.Fragment>
        <Layout style={{ minHeight: '100vh' }}>
            <Header style={{ color: 'white', textAlign: 'center', backgroundColor: '#004ab3' }}>TUS YURIKAI SYSTEM</Header>
            <Content style={{ padding: '50px 50px', minHeight: '80vh' }}>
                <List
                    itemLayout="horizontal"
                    dataSource={content}
                    renderItem={item => (
                        <List.Item style={{ border: 'none', padding: '0' }}>
                        <Button
                            type="primary"
                            block
                            style={{ margin: '4px 0', height: '50px'}} 
                            onClick={() => { navigate(item.route) }}
                        >
                            {item.title}
                        </Button>
                        </List.Item>
                    )}
                />
            </Content>
            <Footer style={{ textAlign: 'center', backgroundColor: 'white' }}>YURUKAI SYSTEM ©2024</Footer>
        </Layout>
    </React.Fragment>
    );
};

export default Toppage;
