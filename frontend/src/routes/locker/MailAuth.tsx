import React, { useEffect } from 'react';
import axios from 'axios';
import { useNavigate } from 'react-router-dom';
import Loading from '../Loading';
import constants from '../constants';

const LockerMailAuth = () => {
    const navigate = useNavigate();

    useEffect(() => {
        const queryParams = new URLSearchParams(window.location.search);
        const token = queryParams.get('token');
        const method = queryParams.get('method');

        const tokenRegex = /^[a-zA-Z0-9]{16}$/;
        const validMethods = ['0', '1', '2'];
        if (!token || !tokenRegex.test(token) || !method || !validMethods.includes(method)) {
            navigate('/locker/nopage');
            return;
        }

        const fetchData = async () => {
            try {

                let apiUrl = '';
                let redirectUrl = '';

                switch (method) {
                    case '0':
                        apiUrl = constants.backendApiEndpoint + '/api/locker/co-auth?token=' + token;
                        redirectUrl = '/locker/auth/complete';
                        break;
                    case '1':
                        apiUrl = constants.backendApiEndpoint + '/api/locker/main-auth?token=' + token;
                        redirectUrl = '/locker/auth/complete';
                        break;
                    case '2':
                        apiUrl = constants.backendApiEndpoint + '/api/locker/auth-check?token=' + token;
                        break;
                    default:
                        navigate('/locker/nopage');
                        return;
                }

                const response = await axios.get(apiUrl);

                if (response.status === 200 || response.status === 201) {
                    if (method === '2') {
                        const pairInfo = response.data.data;
                        const authId = response.data.authId;
                        navigate('/locker/register', { state: { pairInfo, authId } });
                    } else {
                        navigate(redirectUrl);
                    }
                } else {
                    navigate('/locker/nopage');
                }
            } catch (error) {
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

export default LockerMailAuth;
