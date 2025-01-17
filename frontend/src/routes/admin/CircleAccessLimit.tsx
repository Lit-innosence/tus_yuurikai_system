import React, { useState, useEffect } from 'react';
import { Form, DatePicker, TimePicker, Button, message, Card, Row, Col } from 'antd';
import axios from 'axios';
import moment, { Moment } from 'moment';

const AccessRestrictionPage: React.FC = () => {
const [loading, setLoading] = useState(false);
const [form] = Form.useForm();

// ISO形式かどうかをチェックする正規表現
const isoDateTimeRegex = /^\\d{4}-\\d{2}-\\d{2}T\\d{2}:\\d{2}:\\d{2}\\.\\d{3}Z$/;

// レスポンスデータが有効かどうかをチェックする関数
const isValidResponseData = (data: any): data is { start: string; end: string } => {
    return (
    typeof data === 'object' &&
    typeof data.start === 'string' &&
    typeof data.end === 'string' &&
    moment(data.start).isValid() &&
    moment(data.end).isValid() &&
    isoDateTimeRegex.test(data.start) &&
    isoDateTimeRegex.test(data.end)
    );
};

// 現在設定されている時刻を取得する関数
const fetchCurrentSettings = async () => {
    try {
    const response = await axios.get('/api/circle/access/setting', { withCredentials: true });
    if (response.status === 200 && response.data && isValidResponseData(response.data)) {
        console.log(response.data);
        const { start, end } = response.data;

        // フォームに取得した値を設定
        form.setFieldsValue({
        date1: moment(start),
        time1: moment(start),
        date2: moment(end),
        time2: moment(end),
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
    const startTime = moment(start);
    const endTime = moment(end);

    // 開始時刻が終了時刻より前であることを確認
    return startTime.isValid() && endTime.isValid() && startTime.isBefore(endTime);
};

// フォーム提出ハンドラー
const onFinish = async (values: { date1: Moment; time1: Moment; date2: Moment; time2: Moment }) => {
    setLoading(true);

    // 日付と時刻を結合する
    const startDateTime = moment(`${values.date1.format('YYYY-MM-DD')} ${values.time1.format('HH:mm')}`).toISOString();
    const endDateTime = moment(`${values.date2.format('YYYY-MM-DD')} ${values.time2.format('HH:mm')}`).toISOString();

    console.log(startDateTime, endDateTime);

    // 無効な時間の場合はエラーメッセージを表示
    if (!validateTimes(startDateTime, endDateTime)) {
        message.error('無効な時間設定です。時間範囲を再度確認してください。');
        setLoading(false);
    return;
    }

    try {
    const response = await axios.post('/api/circle/access/setting', {
        start: startDateTime,
        end: endDateTime,
    });

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
    <Row justify="center" style={{ marginTop: '50px' }}>
    <Col xs={24} sm={20} md={16} lg={12} xl={10}>
        <Card title="アクセス制限設定" bordered={true}>
        <Form layout="vertical" form={form} onFinish={onFinish}>
            <Form.Item
            label="開始日"
            name="date1"
            rules={[{ required: true, message: '開始日を設定してください' }]}
            >
            <DatePicker style={{ width: '100%' }} />
            </Form.Item>

            <Form.Item
            label="開始時刻"
            name="time1"
            rules={[{ required: true, message: '開始時刻を設定してください' }]}
            >
            <TimePicker style={{ width: '100%' }} format="HH:mm" />
            </Form.Item>

            <Form.Item
            label="終了日"
            name="date2"
            rules={[{ required: true, message: '終了日を設定してください' }]}
            >
            <DatePicker style={{ width: '100%' }} />
            </Form.Item>

            <Form.Item
            label="終了時刻"
            name="time2"
            rules={[{ required: true, message: '終了時刻を設定してください' }]}
            >
            <TimePicker style={{ width: '100%' }} format="HH:mm" />
            </Form.Item>

            <Form.Item>
            <Button type="primary" htmlType="submit" loading={loading} block>
                設定する
            </Button>
            </Form.Item>
        </Form>
        </Card>
    </Col>
    </Row>
);
};

export default AccessRestrictionPage;
