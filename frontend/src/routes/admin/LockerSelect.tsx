import React from 'react';
import SelectLayout from '../../components/PageSelectLayout';

const content = [
    { title: 'ロッカー利用者検索', route: '/admin/locker/search' },
    { title: 'ロッカーリセット', route: '/admin/locker/reset' },
];

const LockerSelect: React.FC = () => {
    return <SelectLayout content={content} kind="admin"/>;
};

export default LockerSelect;
