import React, { useEffect } from 'react';
import axios from 'axios';
import { useNavigate } from 'react-router-dom';
import Loading from '../Loading';
import constants from '../constants';

const CircleMailAuth = () => {
    const navigate = useNavigate();

    useEffect(() => {
        const queryParams = new URLSearchParams(window.location.search);
        const token = queryParams.get('token');
        const method = queryParams.get('method');

        const validMethods = ['0', '1'];
        if (!token || !method || !validMethods.includes(method)) {
            navigate('/circle/nopage');
            return;
        }

        const fetchData = async () => {
            try {

                let apiUrl = '';
                let redirectUrl = '/circle/register/complete';

                switch (method) {
                    case '0':
                        apiUrl = constants.backendApiEndpoint + '/api/circle/co-auth?token=' + token;
                        break;
                    case '1':
                        apiUrl = constants.backendApiEndpoint + '/api/circle/main-auth?token=' + token;
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
        <div>
            <Loading />
        </div>
    );
};

export default CircleMailAuth;
