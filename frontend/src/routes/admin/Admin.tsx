import React from 'react';
import SelectLayout from '../../components/PageSelectLayout';
import DownloadButton from './DownloadButton';

const content = [
    { title: 'ロッカー設定', route: '/admin/locker' },
    { title: '団体登録設定', route: '/admin/circle' },
];

const Admin: React.FC = () => {
    return (
        <>
        <DownloadButton/>
        <SelectLayout content={content} kind="admin"/>
        </>
    );
};

export default Admin;
