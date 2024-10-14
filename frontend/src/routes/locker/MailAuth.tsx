import React, { useEffect } from 'react';
import axios from 'axios';
import { useNavigate } from 'react-router-dom';
import Loading from '../Loading';
import constants from '../constants';

const MailAuth = () => {
    const navigate = useNavigate();

    // 認証部分作成後削除
    const wait = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));

    useEffect(() => {
        const queryParams = new URLSearchParams(window.location.search);
        const token = queryParams.get('token');
        const method = queryParams.get('method');

        const validMethods = ['0', '1', '2'];
        if (!token || !method || !validMethods.includes(method)) {
            navigate('/locker/nopage');
            return;
        }

        const fetchData = async () => {
            try {
                // 認証部分作成後削除
                await wait(5000);

                let apiUrl = '';
                let redirectUrl = '';

                switch (method) {
                    case '0':
                        apiUrl = '/api/locker/co-auth';
                        redirectUrl = '/locker/auth/complete';
                        break;
                    case '1':
                        apiUrl = '/api/locker/main-auth';
                        redirectUrl = '/locker/auth/complete';
                        break;
                    case '2':
                        apiUrl = '/api/locker/auth-check';
                        break;
                    default:
                        navigate('/locker/nopage');
                        return;
                }

                const response = await axios.post(apiUrl, { token });

                if (response.status === 201) {
                    if (method === '2') {
                        const pairInfo = response.data.data;
                        navigate('/locker/register', { state: { pairInfo } });
                    } else {
                        navigate(redirectUrl);
                    }
                } else {
                    navigate('/locker/nopage');
                }
            } catch (error) {
                console.error('Error fetching data:', error);
                navigate('/locker/nopage');
            }
        };

        fetchData();
    }, [navigate]);

    return (
        <div>
            <Loading />
        </div>
    );
};

export default MailAuth;
