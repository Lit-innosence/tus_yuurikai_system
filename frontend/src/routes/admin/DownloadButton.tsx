import React, { useState } from 'react';
import { Button, message, Modal, Input, Tooltip } from 'antd';
import { DownloadOutlined } from '@ant-design/icons';
import axios from 'axios';
import constants from '../constants';

const DownloadButton: React.FC = () => {
    const [isModalVisible, setIsModalVisible] = useState(false);
    const [password, setPassword] = useState('');

    const getFormattedTimestamp = () => {
        const now = new Date();
        const yy = String(now.getFullYear()).slice(-2);
        const mm = String(now.getMonth() + 1).padStart(2, '0');
        const dd = String(now.getDate()).padStart(2, '0');
        const hh = String(now.getHours()).padStart(2, '0');
        const mi = String(now.getMinutes()).padStart(2, '0');
        const ss = String(now.getSeconds()).padStart(2, '0');
        return `${yy}${mm}${dd}${hh}${mi}${ss}`;
    };

    const downloadFile = async (pwd: string) => {
        try {
            const response = await axios.get(
                `${constants.backendApiEndpoint}/api/download?password=${encodeURIComponent(pwd)}`,
                {
                    responseType: 'blob',
                    withCredentials: true
                }
            );

            // 正常なZIPファイルが返ってきた場合のみダウンロードを実行
            if (response.data && response.headers['content-type'] === 'application/zip') {
                const blob = new Blob([response.data], { type: 'application/zip' });
                const url = window.URL.createObjectURL(blob);
                const timestamp = getFormattedTimestamp();
                const filename = `all_data_${timestamp}.zip`;

                const a = document.createElement('a');
                a.href = url;
                a.download = filename;
                document.body.appendChild(a);
                a.click();

                document.body.removeChild(a);
                window.URL.revokeObjectURL(url);
            } else {
                message.error('ダウンロードに失敗しました。');
            }
        } catch (error: any) {
            message.error('ダウンロードに失敗しました。');
        }
    };

    const handleOk = async () => {
        if (!password) {
            message.error('パスワードを入力してください。');
            return;
        }
        setIsModalVisible(false);
        await downloadFile(password);
        setPassword('');
    };

    const handleCancel = () => {
        setIsModalVisible(false);
        setPassword('');
    };

    const showModal = () => {
        setIsModalVisible(true);
    };

    return (
        <>
            <Tooltip title="データベースのバックアップデータをダウンロードします">
                <Button
                    type="primary"
                    shape="circle"
                    danger
                    style={{
                        position: 'fixed',
                        left: '20px',
                        top: '60px',
                        zIndex: 1000,
                        width: '60px',
                        height: '60px',
                        fontSize: '36px',
                        display: 'flex',
                        alignItems: 'center',
                        justifyContent: 'center',
                        boxShadow: '0 4px 6px rgba(0,0,0,0.2)',
                    }}
                    icon={<DownloadOutlined style={{ fontSize: '36px' }} />}
                    onClick={showModal}
                />
            </Tooltip>

            <Modal
                title="パスワード入力"
                visible={isModalVisible}
                onOk={handleOk}
                onCancel={handleCancel}
                okText="ダウンロード"
                cancelText="キャンセル"
            >
                <Input.Password
                    placeholder="パスワードを入力"
                    value={password}
                    onChange={(e) => setPassword(e.target.value)}
                />
            </Modal>
        </>
    );
};

export default DownloadButton;
