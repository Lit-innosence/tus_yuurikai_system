import React from 'react';
import { useNavigate } from 'react-router-dom';
import { Layout, List, Button } from 'antd';
import CustomHeader from '../component/CustomHeader'; 
import CustomFooter from '../component/CustomFooter'; 

type AdminProps = {
    children?: React.ReactNode;
};

const content = [
    { title: 'ロッカー利用者検索', route: '/admin/locker/search' },
];

const { Content } = Layout;

const Admin: React.FC<AdminProps> = (props) => {
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

export default Admin;
