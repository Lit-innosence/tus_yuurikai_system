import React, { useState } from 'react';
import { Button, message, Modal, Input, Tooltip } from 'antd';
import { DownloadOutlined } from '@ant-design/icons';
import axios from 'axios';
import constants from '../constants';

const DownloadButton: React.FC = () => {
    const [isModalVisible, setIsModalVisible] = useState(false);
    const [password, setPassword] = useState('');

    const downloadFile = async (pwd: string) => {
        try {
            // APIリクエスト
            const response = await axios.post(
                `${constants.backendApiEndpoint}/api/admin/download`,
                { password: pwd },
                { withCredentials: true}
            );

            // JSONデータをデコード
            const filename = response.data.filename;
            const zip_data = response.data.zipData;

            // zip_data を Blob に変換
            const blob = new Blob([new Uint8Array(zip_data)], { type: 'application/zip' });
            const url = window.URL.createObjectURL(blob);

            // ダウンロードリンクを作成してクリック
            const a = document.createElement('a');
            a.href = url;
            a.download = filename;
            document.body.appendChild(a);
            a.click();
            document.body.removeChild(a);
            window.URL.revokeObjectURL(url);

            message.success('ダウンロードが完了しました。');
        } catch (error: any) {
            if (error.response) {
                if (error.response.status === 400) {
                    message.error('パスワードが違います');
                } else if (error.response.status === 401) {
                    message.error('認証に失敗しました。ログインしてください。');
                } else {
                    message.error('ダウンロードに失敗しました。');
                }
            } else {
                message.error('ネットワークエラーが発生しました。');
            }
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
                open={isModalVisible}  // Ant Design v5 の `visible` → `open` に変更
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