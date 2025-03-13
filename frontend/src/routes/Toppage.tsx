import React, { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { Layout, List, Button } from 'antd';
import CustomHeader from '../components/CustomHeader';
import CustomFooter from '../components/CustomFooter';
import axios from 'axios';
import constants from './constants';
import AdImage from '../assets/shinkan_v1.png';

const { Content } = Layout;

const content = [
    { title: 'ロッカー空き検索', route: '/locker/terms' },
    { title: 'サークル団体登録', route: '/circle' },
];

const Toppage: React.FC = () => {
    const navigate = useNavigate();
    // サークル登録ボタンのアクセス可否を管理する state
    const [isCircleRegistrationAllowed, setIsCircleRegistrationAllowed] = useState(false);

    useEffect(() => {
        const fetchAccessSetting = async () => {
            try {
                const response = await axios.get(
                    `${constants.backendApiEndpoint}/api/circle/access/setting`
                );
                const { start, end } = response.data;
                const now = new Date();
                const startTime = new Date(start);
                const endTime = new Date(end);
                
                if (now >= startTime && now <= endTime) {
                    setIsCircleRegistrationAllowed(true);
                } else {
                    setIsCircleRegistrationAllowed(false);
                }
            } catch (error) {
                console.error('Error fetching access setting:', error);
                setIsCircleRegistrationAllowed(false);
            }
        };

        fetchAccessSetting();
    }, []);

    return (
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
                                disabled={item.route === '/circle' && !isCircleRegistrationAllowed}
                                style={{ margin: '4px 0', height: '50px' }}
                                onClick={() => {
                                    if (item.route === '/circle' && !isCircleRegistrationAllowed) return;
                                    navigate(item.route);
                                }}
                            >
                                {item.title}
                            </Button>
                        </List.Item>
                    )}
                />
                {/* 広告画像を配置 */}
                <div style={{ marginTop: '30px', textAlign: 'center' }}>
                    <a
                        href={constants.advertisementUrl}
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                        <img
                            src={AdImage}
                            alt="AdImage"
                            style={{ maxWidth: '70%', height: 'auto' }}
                        />
                    </a>
                </div>
            </Content>
            <CustomFooter />
        </Layout>
    );
};

export default Toppage;
