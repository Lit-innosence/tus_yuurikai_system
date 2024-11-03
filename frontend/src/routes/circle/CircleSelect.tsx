import React from 'react';
import SelectLayout from '../component/PageSelectLayout';

const content = [
    { title: '登録情報更新', route: '/circle/register/update' },
    { title: '新規登録', route: '/redirect/example' }, // Google Drive Link
    { title: '公開資料閲覧', route: '/redirect/google' }, // Google Drive Link
    { title: '登録状況照会', route: '/circle/register/status' },
];

const LockerSelect: React.FC = () => {
    return <SelectLayout content={content} />;
};

export default LockerSelect;
