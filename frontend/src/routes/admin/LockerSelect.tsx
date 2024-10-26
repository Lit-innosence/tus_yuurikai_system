import React from 'react';
import AdminLayout from '../component/PageSelectLayout';

const content = [
    { title: 'ロッカー利用者検索', route: '/admin/locker/search' },
    { title: 'ロッカーリセット', route: '/admin/locker/reset' },
];

const LockerSelect: React.FC = () => {
    return <AdminLayout content={content} />;
};

export default LockerSelect;
