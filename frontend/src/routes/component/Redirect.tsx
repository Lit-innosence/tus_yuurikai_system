import React, { useEffect, useState } from 'react';
import { Typography } from 'antd';
import { useParams } from 'react-router-dom';

const Redirect = () => {
    const { '*': path } = useParams();
    const [redirectUrl, setRedirectUrl] = useState('');

    const loadingStyle: React.CSSProperties = {
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'center',
        height: '100vh',
        width: '100vw',
        flexDirection: 'column', 
    };

    useEffect(() => {
        let url;

        switch (path) {
            case 'example':
                url = 'https://example.com';
                break;
            case 'google':
                url = 'https://www.google.com';
                break;
            default:
                url = null;
        }

        if (url) {
            setRedirectUrl(url);
            window.location.href = url;
        } else {
            setRedirectUrl('指定されたURLが見つかりません。');
        }
    }, [path]);

    return (
        <div style={loadingStyle}>
        <Typography.Title level={2}>
            以下のURLにリダイレクトしています、、、
        </Typography.Title>
        {redirectUrl ? (
            <Typography.Link 
                href={redirectUrl} 
                style={{ fontSize: '24px' }} 
            >
                {redirectUrl}
            </Typography.Link>
        ) : (
            <Typography.Text style={{ fontSize: '24px' }}>
                指定したURLが見つかりません。
            </Typography.Text>
        )}
    </div>
    );
};

export default Redirect;
