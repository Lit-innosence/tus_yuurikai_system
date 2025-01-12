import { useEffect } from 'react';
import axios from 'axios';
import { useNavigate } from 'react-router-dom';
import constants from '../constants';
import { Spin } from 'antd';
import { LoadingOutlined } from '@ant-design/icons';

const CircleMailAuth = () => {
    const navigate = useNavigate();

    useEffect(() => {
        const queryParams = new URLSearchParams(window.location.search);
        const token = queryParams.get('token');
        const method = queryParams.get('method');
        const id = queryParams.get('id');

        const validMethods = ['0', '1'];
        if (!token || !method || !id || !validMethods.includes(method)) {
            navigate('/circle/nopage');
            return;
        }

        const fetchData = async () => {
            try {
                let apiUrl = '';
                let redirectUrl = '/circle/update/complete';

                switch (method) {
                    case '0':
                        apiUrl = constants.backendApiEndpoint + '/api/circle/co-auth?token=' + token + '&id=' + id;
                        break;
                    case '1':
                        apiUrl = constants.backendApiEndpoint + '/api/circle/main-auth?token=' + token + '&id=' + id;
                        break;
                    default:
                        navigate('/circle/nopage');
                        return;
                }

                const response = await axios.post(apiUrl);

                if (response.status === 200 || response.status === 201) {
                    navigate(redirectUrl);
                } else {
                    navigate('/circle/nopage');
                }
            } catch (error) {
                console.error('Error fetching data:', error);
                navigate('/circle/nopage');
            }
        };

        fetchData();
    }, [navigate]);

    return (
        <div style={{
            display: 'flex',
            justifyContent: 'center',
            alignItems: 'center',
            height: '100vh',
            flexDirection: 'column',
            textAlign: 'center',
        }}>
            <Spin
                indicator={
                    <LoadingOutlined
                        spin
                        style={{ fontSize: '64px' }} 
                    />
                }
            />
            <p style={{
                marginTop: '16px',
                fontSize: '16px',
                fontWeight: 'bold', 
            }}>
                １分ぐらいかかります、、、
            </p>
        </div>
    );
};

export default CircleMailAuth;
