import React, { useState } from "react";
import { Card, Form, InputNumber, Button, Table, message, Row, Col } from "antd";
import { SearchOutlined, SaveOutlined } from "@ant-design/icons";
import { invoke } from "@tauri-apps/api/core";

interface Stock {
  id: number;
  code: string;
  name: string;
  latest_price?: number;
  change_percent?: number;
  volume?: number;
}

const StockFilter: React.FC = () => {
  const [form] = Form.useForm();
  const [stocks, setStocks] = useState<Stock[]>([]);
  const [loading, setLoading] = useState(false);

  const handleFilter = async (values: any) => {
    setLoading(true);
    try {
      const conditions = {
        price_min: values.price_min,
        price_max: values.price_max,
        change_percent_min: values.change_percent_min,
        change_percent_max: values.change_percent_max,
        volume_min: values.volume_min,
        volume_max: values.volume_max,
      };
      const result = await invoke<Stock[]>("filter_stocks", { conditions });
      setStocks(result);
    } catch (error) {
      message.error("筛选失败");
    } finally {
      setLoading(false);
    }
  };

  const handleSaveCondition = async () => {
    const values = form.getFieldsValue();
    try {
      await invoke("save_filter_conditions", {
        name: "自定义条件",
        conditions: values,
      });
      message.success("保存成功");
    } catch (error) {
      message.error("保存失败");
    }
  };

  const columns = [
    { title: "股票代码", dataIndex: "code", key: "code" },
    { title: "股票名称", dataIndex: "name", key: "name" },
    {
      title: "最新价",
      dataIndex: "latest_price",
      key: "latest_price",
      render: (price?: number) => (price ? `¥${price.toFixed(2)}` : "-"),
    },
    {
      title: "涨跌幅",
      dataIndex: "change_percent",
      key: "change_percent",
      render: (percent?: number) => {
        if (!percent) return "-";
        const color = percent >= 0 ? "#10b981" : "#ef4444";
        return <span style={{ color }}>{percent >= 0 ? "+" : ""}{percent.toFixed(2)}%</span>;
      },
    },
    {
      title: "成交量",
      dataIndex: "volume",
      key: "volume",
      render: (volume?: number) => (volume ? volume.toLocaleString() : "-"),
    },
  ];

  return (
    <div>
      <h2 style={{ color: "#f9fafb", marginBottom: "24px" }}>股票筛选</h2>

      <Card style={{ background: "#111827", border: "1px solid #374151", marginBottom: "24px" }}>
        <Form form={form} onFinish={handleFilter} layout="vertical">
          <Row gutter={16}>
            <Col span={8}>
              <Form.Item name="price_min" label="最低价格">
                <InputNumber style={{ width: "100%" }} placeholder="0" />
              </Form.Item>
            </Col>
            <Col span={8}>
              <Form.Item name="price_max" label="最高价格">
                <InputNumber style={{ width: "100%" }} placeholder="不限" />
              </Form.Item>
            </Col>
            <Col span={8}>
              <Form.Item name="change_percent_min" label="最小涨跌幅(%)">
                <InputNumber style={{ width: "100%" }} placeholder="-10" />
              </Form.Item>
            </Col>
          </Row>
          <Row gutter={16}>
            <Col span={8}>
              <Form.Item name="change_percent_max" label="最大涨跌幅(%)">
                <InputNumber style={{ width: "100%" }} placeholder="10" />
              </Form.Item>
            </Col>
            <Col span={8}>
              <Form.Item name="volume_min" label="最小成交量">
                <InputNumber style={{ width: "100%" }} placeholder="0" />
              </Form.Item>
            </Col>
            <Col span={8}>
              <Form.Item name="volume_max" label="最大成交量">
                <InputNumber style={{ width: "100%" }} placeholder="不限" />
              </Form.Item>
            </Col>
          </Row>
          <Row>
            <Col span={24} style={{ textAlign: "right" }}>
              <Button
                type="default"
                icon={<SaveOutlined />}
                onClick={handleSaveCondition}
                style={{ marginRight: "8px" }}
              >
                保存条件
              </Button>
              <Button
                type="primary"
                icon={<SearchOutlined />}
                htmlType="submit"
                loading={loading}
              >
                开始筛选
              </Button>
            </Col>
          </Row>
        </Form>
      </Card>

      <Card style={{ background: "#111827", border: "1px solid #374151" }}>
        <Table
          dataSource={stocks}
          columns={columns}
          rowKey="id"
          pagination={{ pageSize: 10 }}
          locale={{ emptyText: "暂无数据，请设置筛选条件" }}
        />
      </Card>
    </div>
  );
};

export default StockFilter;
