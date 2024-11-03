import React from 'react';
import SelectLayout from '../component/PageSelectLayout';

const content = [
    { title: 'アクセス制限', route: '/admin/circle/access-limit' },
    { title: '団体情報照会', route: '/admin/circle/info' },
];

const LockerSelect: React.FC = () => {
    return <SelectLayout content={content} />;
};

export default LockerSelect;
