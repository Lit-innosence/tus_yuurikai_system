import React from 'react';
import SelectLayout from '../../components/PageSelectLayout';

const content = [
    { title: '登録情報更新', route: '/circle/update' },
    { title: '新規登録', route: '/redirect/googleform' },
    { title: '公開資料閲覧', route: '/redirect/onedrive' },
    { title: '登録状況照会', route: '/circle/register/status' },
];

const LockerSelect: React.FC = () => {
    return <SelectLayout content={content} />;
};

export default LockerSelect;
