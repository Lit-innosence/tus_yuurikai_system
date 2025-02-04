import React from 'react';
import AdminLayout from './component/PageSelectLayout';

const content = [
    { title: 'ロッカー空き検索', route: '/locker/terms' },
    { title: 'サークル団体登録', route: '/circle' },
];

const Toppage: React.FC = () => {
    return <AdminLayout content={content} />;
};

export default Toppage;
