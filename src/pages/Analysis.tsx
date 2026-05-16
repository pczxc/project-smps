import React, { useState, useEffect } from "react";
import { Card, Table, Button, Modal, Form, Input, Select, message, Tag, DatePicker } from "antd";
import { PlusOutlined } from "@ant-design/icons";
import { invoke } from "@tauri-apps/api/core";
import dayjs from "dayjs";

interface TradeAdvice {
  id: number;
  stock_code: string;
  stock_name: string;
  condition: string;
  current_price: number;
  advice: string;
  created_at: string;
}

const { RangePicker } = DatePicker;

const Analysis: React.FC = () => {
  const [advices, setAdvices] = useState<TradeAdvice[]>([]);
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [form] = Form.useForm();
  const [dateRange, setDateRange] = useState<[dayjs.Dayjs, dayjs.Dayjs] | null>(null);

  useEffect(() => {
    fetchAdvices();
  }, []);

  const fetchAdvices = async () => {
    try {
      const result = await invoke<TradeAdvice[]>("get_trade_advices");
      setAdvices(result);
    } catch (error) {
      message.error("获取交易建议失败");
    }
  };

  const fetchHistory = async () => {
    if (!dateRange) return;
    try {
      const result = await invoke<TradeAdvice[]>("get_trade_advice_history", {
        startDate: dateRange[0].format("YYYY-MM-DD"),
        endDate: dateRange[1].format("YYYY-MM-DD"),
      });
      setAdvices(result);
    } catch (error) {
      message.error("获取历史记录失败");
    }
  };

  const handleSetCondition = async (values: any) => {
    try {
      await invoke("set_monitor_condition", {
        request: values,
      });
      message.success("设置成功");
      setIsModalOpen(false);
      form.resetFields();
    } catch (error) {
      message.error("设置失败");
    }
  };

  const getAdviceColor = (advice: string) => {
    switch (advice) {
      case "buy": return "success";
      case "sell": return "error";
      default: return "default";
    }
  };

  const getAdviceText = (advice: string) => {
    switch (advice) {
      case "buy": return "买入";
      case "sell": return "卖出";
      default: return "持有";
    }
  };

  const columns = [
    { title: "股票代码", dataIndex: "stock_code", key: "stock_code" },
    { title: "股票名称", dataIndex: "stock_name", key: "stock_name" },
    { title: "触发条件", dataIndex: "condition", key: "condition" },
    {
      title: "当前价格",
      dataIndex: "current_price",
      key: "current_price",
      render: (price: number) => `¥${price.toFixed(2)}`,
    },
    {
      title: "建议操作",
      dataIndex: "advice",
      key: "advice",
      render: (advice: string) => (
        <Tag color={getAdviceColor(advice)}>{getAdviceText(advice)}</Tag>
      ),
    },
    {
      title: "时间",
      dataIndex: "created_at",
      key: "created_at",
      render: (date: string) => dayjs(date).format("YYYY-MM-DD HH:mm"),
    },
  ];

  return (
    <div>
      <div style={{ display: "flex", justifyContent: "space-between", marginBottom: "24px" }}>
        <h2 style={{ color: "#f9fafb", margin: 0 }}>智能分析</h2>
        <Button
          type="primary"
          icon={<PlusOutlined />}
          onClick={() => setIsModalOpen(true)}
        >
          设置监控条件
        </Button>
      </div>

      <Card
        title="交易建议"
        style={{ background: "#111827", border: "1px solid #374151", marginBottom: "24px" }}
        headStyle={{ color: "#f9fafb", borderBottom: "1px solid #374151" }}
        extra={
          <div style={{ display: "flex", alignItems: "center", gap: "8px" }}>
            <RangePicker
              value={dateRange}
              onChange={(dates) => setDateRange(dates as [dayjs.Dayjs, dayjs.Dayjs])}
              style={{ background: "#1f2937", borderColor: "#374151" }}
            />
            <Button onClick={fetchHistory}>查询历史</Button>
          </div>
        }
      >
        <Table
          dataSource={advices}
          columns={columns}
          rowKey="id"
          pagination={{ pageSize: 10 }}
          locale={{ emptyText: "暂无交易建议" }}
        />
      </Card>

      <Modal
        title="设置监控条件"
        open={isModalOpen}
        onCancel={() => setIsModalOpen(false)}
        onOk={() => form.submit()}
      >
        <Form form={form} onFinish={handleSetCondition} layout="vertical">
          <Form.Item
            name="stock_id"
            label="股票ID"
            rules={[{ required: true, message: "请输入股票ID" }]}
          >
            <Input type="number" placeholder="股票ID" />
          </Form.Item>
          <Form.Item
            name="condition_type"
            label="条件类型"
            rules={[{ required: true, message: "请选择条件类型" }]}
          >
            <Select placeholder="选择条件类型">
              <Select.Option value="price_up">价格上涨</Select.Option>
              <Select.Option value="price_down">价格下跌</Select.Option>
              <Select.Option value="change_percent">涨跌幅</Select.Option>
              <Select.Option value="volume">成交量</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item
            name="threshold"
            label="阈值"
            rules={[{ required: true, message: "请输入阈值" }]}
          >
            <Input type="number" step="0.01" placeholder="触发阈值" />
          </Form.Item>
          <Form.Item
            name="action"
            label="建议操作"
            rules={[{ required: true, message: "请选择建议操作" }]}
          >
            <Select placeholder="选择建议操作">
              <Select.Option value="buy">买入</Select.Option>
              <Select.Option value="sell">卖出</Select.Option>
              <Select.Option value="hold">持有</Select.Option>
            </Select>
          </Form.Item>
        </Form>
      </Modal>
    </div>
  );
};

export default Analysis;
