import React from 'react';
import { Navigate } from 'react-router-dom';
import { useAuth } from './AuthContext';

type PrivateRouteProps = {
    children?: React.ReactNode;
};

const PrivateRouter: React.FC<PrivateRouteProps> = (props) => {
    const { loggedIn } = useAuth();

    if (!loggedIn) {
        return <Navigate to='/login' />;
    }

    return <>{props.children}</>;
};

export default PrivateRouter;
