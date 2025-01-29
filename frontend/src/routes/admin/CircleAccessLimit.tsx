import React, { useState, useEffect } from 'react';
import { Form, DatePicker, TimePicker, Button, message, Card, Layout, Row, Col } from 'antd';
import axios from 'axios';
import dayjs, { Dayjs } from 'dayjs';
import constants from '../constants';
import CustomHeader from '../component/CustomHeader';
import CustomFooter from '../component/CustomFooter';

const AccessRestrictionPage: React.FC = () => {
    const [loading, setLoading] = useState(false);
    const [form] = Form.useForm();

    // ISO形式かどうかをチェックする正規表現
    const isoDateTimeRegex = /^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}Z$/;

    // レスポンスデータが有効かどうかをチェックする関数
    const isValidResponseData = (data: any): data is { start: string; end: string } => {
        return (
            typeof data === 'object' &&
            (typeof data.start === 'string' || data.start === '') &&
            (typeof data.end === 'string' || data.end === '') &&
            (data.start === '' || dayjs(data.start).isValid()) &&
            (data.end === '' || dayjs(data.end).isValid()) &&
            (data.start === '' || isoDateTimeRegex.test(data.start)) &&
            (data.end === '' || isoDateTimeRegex.test(data.end))
        );
    };

    // 現在設定されている時刻を取得する関数
    const fetchCurrentSettings = async () => {
        try {
            const response = await axios.get(`${constants.backendApiEndpoint}/api/circle/access/setting`, { withCredentials: true });
            if (response.status === 200 && response.data && isValidResponseData(response.data)) {
                const { start, end } = response.data;

                // フォームに取得した値を設定
                form.setFieldsValue({
                    date1: start ? dayjs(start) : null,
                    time1: start ? dayjs(start) : null,
                    date2: end ? dayjs(end) : null,
                    time2: end ? dayjs(end) : null,
                });

                message.success('現在の設定を取得しました。');
            } else {
                throw new Error('Unexpected response structure');
            }
        } catch (error) {
            console.error('Failed to fetch current settings:', error);
            message.error('現在の設定を取得できませんでした。');
        }
    };

    useEffect(() => {
        fetchCurrentSettings();
    }, []);

    // 時間が正しいかをチェックする関数
    const validateTimes = (start: string, end: string): boolean => {
        const startTime = dayjs(start);
        const endTime = dayjs(end);

        // 開始時刻が終了時刻より前であることを確認
        return startTime.isValid() && endTime.isValid() && startTime.isBefore(endTime);
    };

    // フォーム提出ハンドラー
    const onFinish = async (values: { date1: Dayjs; time1: Dayjs; date2: Dayjs; time2: Dayjs }) => {
        setLoading(true);

        // 日付と時刻を結合する
        const startDateTime = dayjs(`${values.date1.format('YYYY-MM-DD')} ${values.time1.format('HH:mm')}`).toISOString();
        const endDateTime = dayjs(`${values.date2.format('YYYY-MM-DD')} ${values.time2.format('HH:mm')}`).toISOString();

        // 無効な時間の場合はエラーメッセージを表示
        if (!validateTimes(startDateTime, endDateTime)) {
            message.error('無効な時間設定です。時間範囲を再度確認してください。');
            setLoading(false);
            return;
        }

        try {
            const response = await axios.post(`${constants.backendApiEndpoint}/api/admin/circle/access/setting`, {
                start: startDateTime,
                end: endDateTime,
            }, { withCredentials: true });

            if (response.status === 201) {
                message.success('設定が完了しました！');
            } else {
                message.error('サーバーエラーが発生しました。もう一度実行してください。');
            }
        } catch (error) {
            console.error(error);
            message.error('内部エラーが生じました。管理者にお問い合わせください。');
        } finally {
            setLoading(false);
        }
    };

    return (
        <Layout style={{ minHeight: '100vh' }}>
            <CustomHeader />
            <Layout
                style={{
                    padding: '50px',
                    display: 'flex',
                    justifyContent: 'center',
                    alignItems: 'center',
                    flexDirection: 'column',
                    overflowX: 'auto',
                }}
            >
                <Card
                    title="アクセス制限設定"
                    bordered={true}
                    style={{
                        maxWidth: '600px',
                        width: '100%',
                    }}
                >
                    <Form
                        layout="vertical"
                        form={form}
                        onFinish={onFinish}
                        style={{
                            width: '100%',
                            maxWidth: '800px',
                            margin: '0 auto',
                        }}
                    >
                        {/* 開始日 */}
                        <Form.Item
                            label="開始日"
                            name="date1"
                            rules={[{ required: true, message: '開始日を設定してください' }]}
                        >
                            <DatePicker style={{ width: '100%' }} />
                        </Form.Item>
    
                        {/* 開始時刻 */}
                        <Form.Item
                            label="開始時刻"
                            name="time1"
                            rules={[{ required: true, message: '開始時刻を設定してください' }]}
                        >
                            <TimePicker style={{ width: '100%' }} format="HH:mm" />
                        </Form.Item>
    
                        {/* 終了日 */}
                        <Form.Item
                            label="終了日"
                            name="date2"
                            rules={[{ required: true, message: '終了日を設定してください' }]}
                        >
                            <DatePicker style={{ width: '100%' }} />
                        </Form.Item>
    
                        {/* 終了時刻 */}
                        <Form.Item
                            label="終了時刻"
                            name="time2"
                            rules={[{ required: true, message: '終了時刻を設定してください' }]}
                        >
                            <TimePicker style={{ width: '100%' }} format="HH:mm" />
                        </Form.Item>
    
                        {/* ボタン */}
                        <Form.Item>
                            <Button type="primary" htmlType="submit" loading={loading} block>
                                設定する
                            </Button>
                        </Form.Item>
                    </Form>
                </Card>
            </Layout>
            <CustomFooter />
        </Layout>
    );
};    

export default AccessRestrictionPage;
