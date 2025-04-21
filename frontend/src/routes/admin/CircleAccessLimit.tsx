import React, { useState, useEffect, useCallback } from 'react';
import { Form, DatePicker, TimePicker, Button, message, Card, Layout } from 'antd';
import axios from 'axios';
import dayjs, { Dayjs } from 'dayjs';
import constants from '../constants';
import CustomAdminHeader from '../../components/CustomAdminHeader';
import CustomFooter from '../../components/CustomFooter';

// ISO 8601 ミリ秒付き UTC 形式チェック用正規表現
const isoDateTimeRegex = /^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}Z$/;

// レスポンスの構造を型ガード
function isValidResponseData(data: any): data is { start: string; end: string } {
  return (
    typeof data === 'object' &&
    (typeof data.start === 'string' || data.start === '') &&
    (typeof data.end   === 'string' || data.end   === '') &&
    (data.start === '' || (dayjs(data.start).isValid() && isoDateTimeRegex.test(data.start))) &&
    (data.end   === '' || (dayjs(data.end).isValid()   && isoDateTimeRegex.test(data.end)))
  );
}

const CircleAccessLimit: React.FC = () => {
  const [loading, setLoading] = useState(false);
  const [form] = Form.useForm();

  // 設定取得 API 呼び出しを useCallback で安定化
  const fetchCurrentSettings = useCallback(async () => {
    try {
      const res = await axios.get(
        `${constants.backendApiEndpoint}/api/circle/access/setting`,
        { withCredentials: true }
      );
      if (res.status === 200 && isValidResponseData(res.data)) {
        const { start, end } = res.data;
        form.setFieldsValue({
          date1: start ? dayjs(start) : null,
          time1: start ? dayjs(start) : null,
          date2: end   ? dayjs(end)   : null,
          time2: end   ? dayjs(end)   : null,
        });
        message.success('現在の設定を取得しました。');
      } else {
        throw new Error('Unexpected response structure');
      }
    } catch {
      message.error('現在の設定を取得できませんでした。');
    }
  }, [form]);

  // 初回レンダー時に設定取得
  useEffect(() => {
    fetchCurrentSettings();
  }, [fetchCurrentSettings]);

  // 開始時刻 < 終了時刻 かどうか
  const validateTimes = (start: string, end: string): boolean => {
    const s = dayjs(start);
    const e = dayjs(end);
    return s.isValid() && e.isValid() && s.isBefore(e);
  };

  // フォーム送信ハンドラ
  const onFinish = async (values: {
    date1: Dayjs;
    time1: Dayjs;
    date2: Dayjs;
    time2: Dayjs;
  }) => {
    setLoading(true);

    const startISO = dayjs(
      `${values.date1.format('YYYY-MM-DD')} ${values.time1.format('HH:mm')}`
    ).toISOString();
    const endISO = dayjs(
      `${values.date2.format('YYYY-MM-DD')} ${values.time2.format('HH:mm')}`
    ).toISOString();

    if (!validateTimes(startISO, endISO)) {
      message.error('無効な時間設定です。開始時刻が終了時刻より前になるよう設定してください。');
      setLoading(false);
      return;
    }

    try {
      const res = await axios.post(
        `${constants.backendApiEndpoint}/api/admin/circle/access/setting`,
        { start: startISO, end: endISO },
        { withCredentials: true }
      );
      if (res.status === 201) {
        message.success('設定が完了しました！');
      } else {
        message.error('サーバーエラーが発生しました。もう一度お試しください。');
      }
    } catch {
      message.error('内部エラーが発生しました。管理者にお問い合わせください。');
    } finally {
      setLoading(false);
    }
  };

  return (
    <Layout style={{ minHeight: '100vh' }}>
      <CustomAdminHeader />
      <Layout
        style={{
          padding: 50,
          display: 'flex',
          justifyContent: 'center',
          alignItems: 'center',
          flexDirection: 'column',
          overflowX: 'auto',
        }}
      >
        <Card title="アクセス制限設定" bordered style={{ maxWidth: 600, width: '100%' }}>
          <Form
            layout="vertical"
            form={form}
            onFinish={onFinish}
            style={{ width: '100%', maxWidth: 800, margin: '0 auto' }}
          >
            <Form.Item
              label="開始日"
              name="date1"
              rules={[{ required: true, message: '開始日を選択してください' }]}
            >
              <DatePicker style={{ width: '100%' }} />
            </Form.Item>

            <Form.Item
              label="開始時刻"
              name="time1"
              rules={[{ required: true, message: '開始時刻を選択してください' }]}
            >
              <TimePicker style={{ width: '100%' }} format="HH:mm" />
            </Form.Item>

            <Form.Item
              label="終了日"
              name="date2"
              rules={[{ required: true, message: '終了日を選択してください' }]}
            >
              <DatePicker style={{ width: '100%' }} />
            </Form.Item>

            <Form.Item
              label="終了時刻"
              name="time2"
              rules={[{ required: true, message: '終了時刻を選択してください' }]}
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
      </Layout>
      <CustomFooter />
    </Layout>
  );
};

export default CircleAccessLimit;
