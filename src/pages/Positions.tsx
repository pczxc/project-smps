import React, { useState, useEffect } from "react";
import { Card, Table, Button, Modal, Form, Input, InputNumber, Select, message, Statistic, Row, Col } from "antd";
import { PlusOutlined, ArrowUpOutlined, ArrowDownOutlined } from "@ant-design/icons";
import { invoke } from "@tauri-apps/api/core";

interface Position {
  id: number;
  stock_code: string;
  stock_name: string;
  quantity: number;
  cost_price: number;
  latest_price?: number;
  floating_profit?: number;
  profit_rate?: number;
}

interface PositionSummary {
  total_market_value: number;
  total_floating_profit: number;
  total_assets: number;
  total_return_rate: number;
}

const Positions: React.FC = () => {
  const [positions, setPositions] = useState<Position[]>([]);
  const [summary, setSummary] = useState<PositionSummary | null>(null);
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [form] = Form.useForm();

  useEffect(() => {
    fetchPositions();
    fetchSummary();
  }, []);

  const fetchPositions = async () => {
    try {
      const result = await invoke<Position[]>("get_positions");
      setPositions(result);
    } catch (error) {
      message.error("获取持仓失败");
    }
  };

  const fetchSummary = async () => {
    try {
      const result = await invoke<PositionSummary>("get_position_summary");
      setSummary(result);
    } catch (error) {
      message.error("获取资产概览失败");
    }
  };

  const handleAddTransaction = async (values: any) => {
    try {
      await invoke("add_transaction", {
        request: {
          stock_code: values.stock_code,
          stock_name: values.stock_name,
          type: values.type,
          quantity: values.quantity,
          price: values.price,
          transaction_date: values.transaction_date,
        },
      });
      message.success("交易记录添加成功");
      setIsModalOpen(false);
      form.resetFields();
      fetchPositions();
      fetchSummary();
    } catch (error) {
      message.error("添加失败");
    }
  };

  const columns = [
    { title: "股票代码", dataIndex: "stock_code", key: "stock_code" },
    { title: "股票名称", dataIndex: "stock_name", key: "stock_name" },
    { title: "持仓数量", dataIndex: "quantity", key: "quantity" },
    {
      title: "成本价",
      dataIndex: "cost_price",
      key: "cost_price",
      render: (price: number) => `¥${price.toFixed(2)}`,
    },
    {
      title: "最新价",
      dataIndex: "latest_price",
      key: "latest_price",
      render: (price?: number) => (price ? `¥${price.toFixed(2)}` : "-"),
    },
    {
      title: "浮动盈亏",
      dataIndex: "floating_profit",
      key: "floating_profit",
      render: (profit?: number) => {
        if (!profit) return "-";
        const color = profit >= 0 ? "#10b981" : "#ef4444";
        return <span style={{ color }}>{profit >= 0 ? "+" : ""}¥{profit.toFixed(2)}</span>;
      },
    },
    {
      title: "收益率",
      dataIndex: "profit_rate",
      key: "profit_rate",
      render: (rate?: number) => {
        if (!rate) return "-";
        const color = rate >= 0 ? "#10b981" : "#ef4444";
        return <span style={{ color }}>{rate >= 0 ? "+" : ""}{rate.toFixed(2)}%</span>;
      },
    },
  ];

  return (
    <div>
      <div style={{ display: "flex", justifyContent: "space-between", marginBottom: "24px" }}>
        <h2 style={{ color: "#f9fafb", margin: 0 }}>持仓管理</h2>
        <Button
          type="primary"
          icon={<PlusOutlined />}
          onClick={() => setIsModalOpen(true)}
        >
          录入交易
        </Button>
      </div>

      {summary && (
        <Row gutter={[16, 16]} style={{ marginBottom: "24px" }}>
          <Col span={6}>
            <Card style={{ background: "#111827", border: "1px solid #374151" }}>
              <Statistic
                title="总市值"
                value={summary.total_market_value}
                precision={2}
                valueStyle={{ color: "#f9fafb" }}
                prefix="¥"
              />
            </Card>
          </Col>
          <Col span={6}>
            <Card style={{ background: "#111827", border: "1px solid #374151" }}>
              <Statistic
                title="总浮动盈亏"
                value={summary.total_floating_profit}
                precision={2}
                valueStyle={{ color: summary.total_floating_profit >= 0 ? "#10b981" : "#ef4444" }}
                prefix={summary.total_floating_profit >= 0 ? <ArrowUpOutlined /> : <ArrowDownOutlined />}
                suffix="¥"
              />
            </Card>
          </Col>
          <Col span={6}>
            <Card style={{ background: "#111827", border: "1px solid #374151" }}>
              <Statistic
                title="总资产"
                value={summary.total_assets}
                precision={2}
                valueStyle={{ color: "#f9fafb" }}
                prefix="¥"
              />
            </Card>
          </Col>
          <Col span={6}>
            <Card style={{ background: "#111827", border: "1px solid #374151" }}>
              <Statistic
                title="总收益率"
                value={summary.total_return_rate}
                precision={2}
                valueStyle={{ color: summary.total_return_rate >= 0 ? "#10b981" : "#ef4444" }}
                prefix={summary.total_return_rate >= 0 ? <ArrowUpOutlined /> : <ArrowDownOutlined />}
                suffix="%"
              />
            </Card>
          </Col>
        </Row>
      )}

      <Card style={{ background: "#111827", border: "1px solid #374151" }}>
        <Table
          dataSource={positions}
          columns={columns}
          rowKey="id"
          pagination={{ pageSize: 10 }}
          locale={{ emptyText: "暂无持仓" }}
        />
      </Card>

      <Modal
        title="录入交易"
        open={isModalOpen}
        onCancel={() => setIsModalOpen(false)}
        onOk={() => form.submit()}
        width={600}
      >
        <Form form={form} onFinish={handleAddTransaction} layout="vertical">
          <Form.Item
            name="stock_code"
            label="股票代码"
            rules={[{ required: true, message: "请输入股票代码" }]}
          >
            <Input placeholder="如: 000001" />
          </Form.Item>
          <Form.Item
            name="stock_name"
            label="股票名称"
            rules={[{ required: true, message: "请输入股票名称" }]}
          >
            <Input placeholder="如: 平安银行" />
          </Form.Item>
          <Form.Item
            name="type"
            label="交易类型"
            rules={[{ required: true, message: "请选择交易类型" }]}
          >
            <Select placeholder="选择交易类型">
              <Select.Option value="buy">买入</Select.Option>
              <Select.Option value="sell">卖出</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item
            name="quantity"
            label="交易数量"
            rules={[{ required: true, message: "请输入交易数量" }]}
          >
            <InputNumber style={{ width: "100%" }} min={1} placeholder="100" />
          </Form.Item>
          <Form.Item
            name="price"
            label="交易价格"
            rules={[{ required: true, message: "请输入交易价格" }]}
          >
            <InputNumber style={{ width: "100%" }} min={0.01} step={0.01} placeholder="10.00" />
          </Form.Item>
          <Form.Item
            name="transaction_date"
            label="交易日期"
            rules={[{ required: true, message: "请输入交易日期" }]}
          >
            <Input type="date" />
          </Form.Item>
        </Form>
      </Modal>
    </div>
  );
};

export default Positions;
