import React, { createContext, useContext, useState, useEffect } from 'react';
import { Navigate } from 'react-router-dom';
import axios from 'axios';
import constants from '../routes/constants';
import Loading from '../routes/Loading';

type AccessSettings = {
    start: string;
    end: string;
};

type TimeContextState = {
    accessible: boolean;
    isLoading: boolean;
    accessSettings: AccessSettings | null;
};

const TimeContext = createContext<TimeContextState>({
    accessible: false,
    isLoading: true,
    accessSettings: null,
});

export const useTimeContext = () => useContext(TimeContext);

type TimeRouterProps = {
    children?: React.ReactNode;
};

const TimeRouter: React.FC<TimeRouterProps> = ({ children }) => {
const [accessible, setAccessible] = useState<boolean>(false);
const [isLoading, setIsLoading] = useState<boolean>(true);
const [accessSettings, setAccessSettings] = useState<AccessSettings | null>(null);

useEffect(() => {
    const fetchAccessSettings = async () => {
    setIsLoading(true);
    try {
        const response = await axios.get(`${constants.backendApiEndpoint}/api/circle/access/setting`);
        if (response.data && response.data.start && response.data.end) {
        setAccessSettings(response.data);
        const startTime = new Date(response.data.start).getTime();
        const endTime = new Date(response.data.end).getTime();
        const nowTime = Date.now();
        setAccessible(nowTime >= startTime && nowTime <= endTime);
        } else {
        setAccessible(false);
        setAccessSettings(null);
        }
    } catch (error) {
        setAccessible(false);
        setAccessSettings(null);
    } finally {
        setIsLoading(false);
    }
    };

    fetchAccessSettings();
}, []);

// 読み込み中は Loading を表示
if (isLoading) {
    return <Loading />;
}

// アクセス不可の場合は /circle/nopage へリダイレクト
if (!accessible) {
    return <Navigate to="/circle/nopage" />;
}

// 内部のコンテキストを提供しつつ、子コンポーネントを表示
return (
    <TimeContext.Provider value={{ accessible, isLoading, accessSettings }}>
    {children}
    </TimeContext.Provider>
);
};

export default TimeRouter;
