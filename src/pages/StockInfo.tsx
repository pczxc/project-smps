import React, { useState, useEffect } from "react";
import { Card, Table, Button, Input, Space, Tag, message } from "antd";
import { SyncOutlined, SearchOutlined } from "@ant-design/icons";
import { invoke } from "@tauri-apps/api/core";

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

const StockInfoPage: React.FC = () => {
  const [stocks, setStocks] = useState<StockInfo[]>([]);
  const [filteredStocks, setFilteredStocks] = useState<StockInfo[]>([]);
  const [searchText, setSearchText] = useState("");
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    fetchStockInfos();
  }, []);

  useEffect(() => {
    if (searchText) {
      const filtered = stocks.filter(
        (stock) =>
          stock.code.toLowerCase().includes(searchText.toLowerCase()) ||
          stock.name.toLowerCase().includes(searchText.toLowerCase()) ||
          stock.industry?.toLowerCase().includes(searchText.toLowerCase())
      );
      setFilteredStocks(filtered);
    } else {
      setFilteredStocks(stocks);
    }
  }, [searchText, stocks]);

  const fetchStockInfos = async () => {
    try {
      setLoading(true);
      const result = await invoke<StockInfo[]>("get_all_stocks");
      setStocks(result);
      setFilteredStocks(result);
    } catch (error) {
      message.error("获取股票信息失败");
    } finally {
      setLoading(false);
    }
  };

  const handleSyncStocks = async () => {
        try {
            message.loading({ content: "正在同步股票数据...", key: "sync" });
            const result = await invoke<string>("sync_stock_data");
            message.success({ content: result, key: "sync" });
            await fetchStockInfos();
        } catch (error) {
            console.error("同步股票数据失败:", error);
            const errorMsg = error instanceof Error ? error.message : String(error);
            message.error({ content: `同步失败: ${errorMsg}`, key: "sync" });
        }
    };

  const getMarketTag = (market: string) => {
    switch (market) {
      case "A":
        return <Tag color="blue">A股</Tag>;
      case "H":
        return <Tag color="red">港股</Tag>;
      default:
        return <Tag>{market}</Tag>;
    }
  };

  const getExchangeTag = (exchange: string) => {
    switch (exchange) {
      case "SSE":
        return <Tag color="geekblue">上交所</Tag>;
      case "SZSE":
        return <Tag color="purple">深交所</Tag>;
      case "HKEX":
        return <Tag color="volcano">港交所</Tag>;
      default:
        return <Tag>{exchange}</Tag>;
    }
  };

  const columns = [
    { title: "股票代码", dataIndex: "code", key: "code", width: 120 },
    { title: "股票名称", dataIndex: "name", key: "name", width: 150 },
    {
      title: "市场",
      dataIndex: "market",
      key: "market",
      width: 80,
      render: (market: string) => getMarketTag(market),
    },
    {
      title: "交易所",
      dataIndex: "exchange",
      key: "exchange",
      width: 100,
      render: (exchange: string) => getExchangeTag(exchange),
    },
    {
      title: "行业",
      dataIndex: "industry",
      key: "industry",
      width: 150,
      render: (industry?: string) => industry || "-",
    },
    {
      title: "地区",
      dataIndex: "area",
      key: "area",
      width: 100,
      render: (area?: string) => area || "-",
    },
    {
      title: "上市日期",
      dataIndex: "list_date",
      key: "list_date",
      width: 120,
      render: (date?: string) => date || "-",
    },
  ];

  return (
    <div>
      <div
        style={{
          display: "flex",
          justifyContent: "space-between",
          marginBottom: "24px",
          alignItems: "center",
        }}
      >
        <h2 style={{ color: "#f9fafb", margin: 0 }}>股票信息管理</h2>
        <Space>
          <Input
            placeholder="搜索股票代码、名称或行业"
            prefix={<SearchOutlined />}
            onChange={(e) => setSearchText(e.target.value)}
            style={{ width: 300 }}
            allowClear
          />
          <Button type="primary" icon={<SyncOutlined />} onClick={handleSyncStocks}>
            同步股票数据
          </Button>
        </Space>
      </div>

      <Card style={{ background: "#111827", border: "1px solid #374151" }}>
        <Table
          dataSource={filteredStocks}
          columns={columns}
          rowKey="id"
          loading={loading}
          pagination={{
            pageSize: 20,
            showSizeChanger: true,
            showTotal: (total) => `共 ${total} 只股票`,
          }}
          scroll={{ x: 1000 }}
        />
      </Card>
    </div>
  );
};

export default StockInfoPage;
