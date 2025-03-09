import React from 'react';
import { useNavigate } from 'react-router-dom';
import { Layout, List, Button } from 'antd';
import CustomHeader from './CustomHeader';
import CustomAdminHeader from './CustomAdminHeader';
import CustomFooter from './CustomFooter';

const { Content } = Layout;

type ContentItem = {
    title: string;
    route: string;
};

type SelectLayoutProps = {
    content: ContentItem[];
    kind?: 'admin' | 'user';
};

const SelectLayout: React.FC<SelectLayoutProps> = ({ content, kind = 'user' }) => {
    const navigate = useNavigate();
    const HeaderComponent = kind === 'admin' ? CustomAdminHeader : CustomHeader;

    return (
        <React.Fragment>
            <Layout style={{ minHeight: '100vh' }}>
                <HeaderComponent />
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

export default SelectLayout;
