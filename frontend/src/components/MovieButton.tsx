import React, { useState } from 'react';
import { Button, Modal } from 'antd';
import { PlayCircleOutlined } from '@ant-design/icons';

const MovieButton: React.FC = () => {
    const [isModalVisible, setIsModalVisible] = useState(false);

    const showModal = () => {
        setIsModalVisible(true);
    };

    const handleCancel = () => {
        setIsModalVisible(false);
    };

    return (
        <>
            <Button
                type="primary"
                shape="circle"
                danger 
                style={{
                    position: 'fixed',
                    right: '20px',
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
                icon={<PlayCircleOutlined style={{ fontSize: '36px' }} />} 
                onClick={showModal}
            />

            <Modal 
                title="ロッカー登録のご案内"
                open={isModalVisible}
                onCancel={handleCancel}
                centered
                closable={false}
                width={'80%'}
                footer={[
                    <Button key="close" onClick={handleCancel}>
                        閉じる
                    </Button>
                ]}
            >
                <div style={{ position: 'relative', paddingBottom: '56.25%', height: 0, overflow: 'hidden' }}>  // アスペクト比
                    <iframe
                        src="https://www.youtube.com/embed/YOUR_VIDEO_ID"
                        title="YouTube動画"
                        allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                        allowFullScreen
                        style={{
                            position: 'absolute',
                            top: 0,
                            left: 0,
                            width: '100%',
                            height: '100%',
                        }}
                    ></iframe>
                </div>
            </Modal>
        </>
    );
};

export default MovieButton;
