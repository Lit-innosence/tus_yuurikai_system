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
        padding: '0 20px', 
        textAlign: 'center', 
    };

    const linkStyle: React.CSSProperties = {
        fontSize: '24px',
        wordWrap: 'break-word', 
        maxWidth: '100%', 
    };

    useEffect(() => {
        let url;

        switch (path) {
            case 'googleform':
                url = 'https://www.google.co.jp/';
                break;
            case 'onedrive':
                url = 'https://onedrive.live.com/?authkey=%21AHAUgQpyo9KUOJA&id=7A895461B26453F8%21123&cid=7A895461B26453F8';
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
                    style={linkStyle}
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
