import React from 'react';
import { useNavigate } from 'react-router-dom';
import { Layout, List, Button } from 'antd';
import CustomHeader from './CustomHeader';
import CustomFooter from './CustomFooter';

const { Content } = Layout;

type ContentItem = {
    title: string;
    route: string;
};

type AdminLayoutProps = {
    content: ContentItem[];
};

const AdminLayout: React.FC<AdminLayoutProps> = ({ content }) => {
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
                                    style={{ margin: '4px 0', height: '50px' }}
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

export default AdminLayout;
