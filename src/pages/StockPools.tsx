import React, { useState, useEffect } from "react";
import { Card, Table, Button, Modal, Form, Input, message, Tabs, Popconfirm, AutoComplete } from "antd";
import { PlusOutlined, DeleteOutlined, SyncOutlined } from "@ant-design/icons";
import { invoke } from "@tauri-apps/api/core";

interface StockPool {
  id: number;
  name: string;
  description?: string;
  created_at: string;
}

interface Stock {
  id: number;
  code: string;
  name: string;
  pool_id: number;
  latest_price?: number;
  change_percent?: number;
  volume?: number;
  monitored: boolean;
}

interface StockInfo {
  id: number;
  code: string;
  name: string;
  market: string;
  exchange: string;
  list_date?: string;
  industry?: string;
  area?: string;
  synced_at: string;
}

const StockPools: React.FC = () => {
  const [pools, setPools] = useState<StockPool[]>([]);
  const [stocks, setStocks] = useState<Stock[]>([]);
  const [activePool, setActivePool] = useState<number | null>(null);
  const [isPoolModalOpen, setIsPoolModalOpen] = useState(false);
  const [isStockModalOpen, setIsStockModalOpen] = useState(false);
  const [stockOptions, setStockOptions] = useState<{ value: string; label: string; code: string; name: string }[]>([]);
  const [form] = Form.useForm();
  const [stockForm] = Form.useForm();

  useEffect(() => {
    fetchPools();
  }, []);

  useEffect(() => {
    if (activePool) {
      fetchStocks(activePool);
    }
  }, [activePool]);

  const fetchPools = async () => {
    try {
      const result = await invoke<StockPool[]>("get_stock_pools");
      setPools(result);
      if (result.length > 0 && !activePool) {
        setActivePool(result[0].id);
      }
    } catch (error) {
      message.error("获取股票池失败");
    }
  };

  const fetchStocks = async (poolId: number) => {
    try {
      const result = await invoke<Stock[]>("get_stocks_in_pool", { poolId });
      setStocks(result);
    } catch (error) {
      message.error("获取股票列表失败");
    }
  };

  const handleCreatePool = async (values: any) => {
    try {
      await invoke("create_stock_pool", { request: values });
      message.success("创建成功");
      setIsPoolModalOpen(false);
      form.resetFields();
      fetchPools();
    } catch (error) {
      message.error("创建失败");
    }
  };

  const handleAddStock = async (values: any) => {
    if (!activePool) return;
    try {
      await invoke("add_stock_to_pool", {
        request: { pool_id: activePool, ...values },
      });
      message.success("添加成功");
      setIsStockModalOpen(false);
      stockForm.resetFields();
      fetchStocks(activePool);
    } catch (error) {
      message.error("添加失败");
    }
  };

  const handleDeletePool = async (id: number) => {
    try {
      await invoke("delete_stock_pool", { id });
      message.success("删除成功");
      fetchPools();
    } catch (error) {
      message.error("删除失败");
    }
  };

  const handleRemoveStock = async (stockId: number) => {
    try {
      await invoke("remove_stock_from_pool", { stockId });
      message.success("移除成功");
      if (activePool) fetchStocks(activePool);
    } catch (error) {
      message.error("移除失败");
    }
  };

  const handleSyncStocks = async () => {
        try {
            message.loading({ content: "正在同步股票数据...", key: "sync" });
            const result = await invoke<string>("sync_stock_data");
            message.success({ content: result, key: "sync" });
        } catch (error) {
            console.error("同步股票数据失败:", error);
            const errorMsg = error instanceof Error ? error.message : String(error);
            message.error({ content: `同步失败: ${errorMsg}`, key: "sync" });
        }
    };

  const handleSearchStock = async (value: string) => {
    if (!value) {
      setStockOptions([]);
      return;
    }
    try {
      const result = await invoke<StockInfo[]>("search_stocks", { query: value });
      const options = result.map((stock) => ({
        value: stock.code,
        label: `${stock.code} - ${stock.name} (${stock.market})`,
        code: stock.code,
        name: stock.name,
      }));
      setStockOptions(options);
    } catch (error) {
      console.error("搜索失败", error);
    }
  };

  const handleSelectStock = (value: string, option: any) => {
    stockForm.setFieldsValue({
      code: option.code,
      name: option.name,
    });
  };

  const stockColumns = [
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
      title: "操作",
      key: "action",
      render: (_: any, record: Stock) => (
        <Popconfirm
          title="确认移除"
          description="确定要从股票池中移除这只股票吗？"
          onConfirm={() => handleRemoveStock(record.id)}
          okText="确定"
          cancelText="取消"
        >
          <Button type="link" danger icon={<DeleteOutlined />}>
            移除
          </Button>
        </Popconfirm>
      ),
    },
  ];

  return (
    <div>
      <div style={{ display: "flex", justifyContent: "space-between", marginBottom: "24px" }}>
        <h2 style={{ color: "#f9fafb", margin: 0 }}>股票池管理</h2>
        <div style={{ display: "flex", gap: "8px" }}>
          <Button
            icon={<SyncOutlined />}
            onClick={handleSyncStocks}
          >
            同步股票数据
          </Button>
          <Button
            type="primary"
            icon={<PlusOutlined />}
            onClick={() => setIsPoolModalOpen(true)}
          >
            新建股票池
          </Button>
        </div>
      </div>

      <Tabs
        activeKey={activePool?.toString()}
        onChange={(key) => setActivePool(Number(key))}
        items={pools.map((pool) => ({
          key: pool.id.toString(),
          label: pool.name,
          children: (
            <Card
              style={{ background: "#111827", border: "1px solid #374151" }}
              extra={
                <div>
                  <Button
                    type="primary"
                    icon={<PlusOutlined />}
                    onClick={() => setIsStockModalOpen(true)}
                    style={{ marginRight: "8px" }}
                  >
                    添加股票
                  </Button>
                  <Popconfirm
                    title="确认删除"
                    description="确定要删除这个股票池吗？"
                    onConfirm={() => handleDeletePool(pool.id)}
                    okText="确定"
                    cancelText="取消"
                  >
                    <Button danger icon={<DeleteOutlined />}>
                      删除
                    </Button>
                  </Popconfirm>
                </div>
              }
            >
              <Table
                dataSource={stocks}
                columns={stockColumns}
                rowKey="id"
                pagination={{ pageSize: 10 }}
              />
            </Card>
          ),
        }))}
      />

      <Modal
        title="新建股票池"
        open={isPoolModalOpen}
        onCancel={() => setIsPoolModalOpen(false)}
        onOk={() => form.submit()}
      >
        <Form form={form} onFinish={handleCreatePool} layout="vertical">
          <Form.Item
            name="name"
            label="股票池名称"
            rules={[{ required: true, message: "请输入股票池名称" }]}
          >
            <Input />
          </Form.Item>
          <Form.Item name="description" label="描述">
            <Input.TextArea />
          </Form.Item>
        </Form>
      </Modal>

      <Modal
        title="添加股票"
        open={isStockModalOpen}
        onCancel={() => setIsStockModalOpen(false)}
        onOk={() => stockForm.submit()}
        width={600}
      >
        <Form form={stockForm} onFinish={handleAddStock} layout="vertical">
          <Form.Item
            name="search"
            label="搜索股票"
            help="输入股票代码或名称进行搜索，选择后自动填充"
          >
            <AutoComplete
              options={stockOptions}
              onSearch={handleSearchStock}
              onSelect={handleSelectStock}
              placeholder="输入股票代码或名称"
              style={{ width: "100%" }}
              showSearch
            />
          </Form.Item>
          <Form.Item
            name="code"
            label="股票代码"
            rules={[{ required: true, message: "请输入股票代码" }]}
          >
            <Input placeholder="如: 000001" />
          </Form.Item>
          <Form.Item
            name="name"
            label="股票名称"
            rules={[{ required: true, message: "请输入股票名称" }]}
          >
            <Input placeholder="如: 平安银行" />
          </Form.Item>
        </Form>
      </Modal>
    </div>
  );
};

export default StockPools;
