import React from 'react';
import { useNavigate } from 'react-router-dom';
import { Layout, List, Button } from 'antd';
import CustomHeader from './component/CustomHeader'; 
import CustomFooter from './component/CustomFooter'; 

type ToppageProps = {
    children?: React.ReactNode;
};

const content = [
    { title: 'ロッカー空き検索', route: '/locker/terms' },
    { title: 'サークル団体登録', route: '/circle' },
];

const { Content } = Layout;

const Toppage: React.FC<ToppageProps> = (props) => {
    const navigate = useNavigate();

    return (
    <React.Fragment>
        <Layout style={{ minHeight: '100vh' }}>
            <CustomHeader />
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
            <CustomFooter />
        </Layout>
    </React.Fragment>
    );
};

export default Toppage;
