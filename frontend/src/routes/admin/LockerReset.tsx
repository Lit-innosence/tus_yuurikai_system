import React, { useState } from 'react';
import { Layout, Button, Modal, Input, message } from 'antd';
import axios from 'axios';
import CustomHeader from '../component/CustomHeader';
import CustomFooter from '../component/CustomFooter';

const { Content } = Layout;

const LockerReset: React.FC = () => {
    const [isModalVisible, setIsModalVisible] = useState(false);
    const [password, setPassword] = useState('');
    const [loading, setLoading] = useState(false);

    const showModal = () => {
        setIsModalVisible(true);
    };

    const handleCancel = () => {
        setIsModalVisible(false);
        setPassword('');
    };

    const handleConfirm = async () => {
        if (!password) {
            message.error('パスワードを入力してください');
            return;
        }

        setLoading(true);
        try {
            await axios.post('/api/admin/locker/reset', { password }, { withCredentials: true });
            message.success('リセットが成功しました');
        } catch (error) {
            message.error('リセットに失敗しました');
        } finally {
            setLoading(false);
            setIsModalVisible(false);
            setPassword('');
        }
    };

    return (
        <Layout style={{ minHeight: '100vh' }}>
            <CustomHeader />
            <Content style={{ padding: '50px', display: 'flex', justifyContent: 'center', alignItems: 'center' }}>
                {/* 黄色と黒の縞模様の背景 */}
                <div style={{
                    width: '450px',
                    height: '200px',
                    background: 'repeating-linear-gradient(45deg, yellow, yellow 20px, black 20px, black 40px)',
                    display: 'flex',
                    justifyContent: 'center',
                    alignItems: 'center',
                    position: 'relative',
                }}>
                    {/* 中央の赤い丸ボタン */}
                    <Button
                        type="primary"
                        onClick={showModal}
                        style={{
                            fontSize: '28px',
                            height: '150px',
                            width: '150px',
                            fontWeight: 'bold',
                            color: '#FFFFFF',
                            borderRadius: '50%',
                            backgroundColor: '#FF0000',
                            border: 'none',
                            boxShadow: '0px 15px 30px rgba(0, 0, 0, 0.6), inset 0px 5px 10px rgba(255, 255, 255, 0.2)',
                            position: 'absolute',
                        }}
                    >
                        リセット
                    </Button>
                </div>
                <Modal
                    title="確認"
                    visible={isModalVisible}
                    onOk={handleConfirm}
                    onCancel={handleCancel}
                    confirmLoading={loading}
                    okText="リセットする"
                    cancelText="キャンセル"
                >
                    <p>リセットを実行するにはパスワードを入力してください。</p>
                    <Input.Password
                        placeholder="パスワード"
                        value={password}
                        onChange={(e) => setPassword(e.target.value)}
                    />
                </Modal>
            </Content>
            <CustomFooter />
        </Layout>
    );
};

export default LockerReset;
