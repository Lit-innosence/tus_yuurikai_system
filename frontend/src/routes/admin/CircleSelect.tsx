import React from 'react';
import SelectLayout from '../component/PageSelectLayout';

const content = [
    { title: 'アクセス制限', route: '/admin/circle/access' },
    { title: '団体情報照会', route: '/admin/circle/list' },
];

const CircleSelect: React.FC = () => {
    return <SelectLayout content={content} kind="admin"/>;
};

export default CircleSelect;
