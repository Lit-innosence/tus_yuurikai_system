import React from 'react';
import AdminLayout from '../component/PageSelectLayout';

const content = [
    { title: 'ロッカー設定', route: '/admin/locker' },
    { title: '団体登録設定', route: '/admin/circle' },
];

const Admin: React.FC = () => {
    return <AdminLayout content={content} />;
};

export default Admin;
