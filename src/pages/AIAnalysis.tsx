import React, { useState } from "react";
import { Card, Button, Input, Select, message, Typography, List, Tag } from "antd";
import { RobotOutlined, HistoryOutlined } from "@ant-design/icons";
import { invoke } from "@tauri-apps/api/core";

const { Text } = Typography;

interface AiAnalysis {
  id: number;
  stock_code: string;
  stock_name: string;
  content: string;
  created_at: string;
}

const AIAnalysis: React.FC = () => {
  const [stockCode, setStockCode] = useState("");
  const [stockName, setStockName] = useState("");
  const [modelType, setModelType] = useState("doubao");
  const [loading, setLoading] = useState(false);
  const [analysis, setAnalysis] = useState<AiAnalysis | null>(null);
  const [history, setHistory] = useState<AiAnalysis[]>([]);
  const [showHistory, setShowHistory] = useState(false);

  const handleAnalyze = async () => {
    if (!stockCode || !stockName) {
      message.error("请输入股票代码和名称");
      return;
    }

    setLoading(true);
    try {
      const result = await invoke<AiAnalysis>("analyze_stock_with_ai", {
        request: {
          stock_code: stockCode,
          stock_name: stockName,
          model_type: modelType,
        },
      });
      setAnalysis(result);
      message.success("分析完成");
    } catch (error) {
      message.error("分析失败");
    } finally {
      setLoading(false);
    }
  };

  const fetchHistory = async () => {
    try {
      const result = await invoke<AiAnalysis[]>("get_ai_analyses", {
        stockCode: stockCode || null,
      });
      setHistory(result);
      setShowHistory(true);
    } catch (error) {
      message.error("获取历史记录失败");
    }
  };

  return (
    <div>
      <h2 style={{ color: "#f9fafb", marginBottom: "24px" }}>AI分析</h2>

      <Card style={{ background: "#111827", border: "1px solid #374151", marginBottom: "24px" }}>
        <div style={{ display: "flex", gap: "16px", marginBottom: "16px" }}>
          <Input
            placeholder="股票代码"
            value={stockCode}
            onChange={(e) => setStockCode(e.target.value)}
            style={{ width: "200px", background: "#1f2937", borderColor: "#374151" }}
          />
          <Input
            placeholder="股票名称"
            value={stockName}
            onChange={(e) => setStockName(e.target.value)}
            style={{ width: "200px", background: "#1f2937", borderColor: "#374151" }}
          />
          <Select
            value={modelType}
            onChange={setModelType}
            style={{ width: "200px", background: "#1f2937" }}
            options={[
              { value: "doubao", label: "豆包" },
              { value: "wenxin", label: "文心一言" },
              { value: "qianwen", label: "通义千问" },
            ]}
          />
          <Button
            type="primary"
            icon={<RobotOutlined />}
            onClick={handleAnalyze}
            loading={loading}
          >
            开始分析
          </Button>
          <Button
            icon={<HistoryOutlined />}
            onClick={fetchHistory}
          >
            历史记录
          </Button>
        </div>
      </Card>

      {analysis && (
        <Card
          title={`${analysis.stock_name}(${analysis.stock_code}) 分析报告`}
          style={{ background: "#111827", border: "1px solid #374151", marginBottom: "24px" }}
          headStyle={{ color: "#f9fafb", borderBottom: "1px solid #374151" }}
        >
          <div style={{ color: "#f9fafb", whiteSpace: "pre-wrap" }}>
            {analysis.content}
          </div>
        </Card>
      )}

      {showHistory && (
        <Card
          title="历史分析记录"
          style={{ background: "#111827", border: "1px solid #374151" }}
          headStyle={{ color: "#f9fafb", borderBottom: "1px solid #374151" }}
        >
          <List
            dataSource={history}
            renderItem={(item) => (
              <List.Item style={{ borderBottom: "1px solid #374151" }}>
                <div>
                  <Tag color="blue">{item.stock_code}</Tag>
                  <Text style={{ color: "#f9fafb", marginLeft: "8px" }}>{item.stock_name}</Text>
                  <Text style={{ color: "#9ca3af", marginLeft: "16px" }}>
                    {new Date(item.created_at).toLocaleString()}
                  </Text>
                </div>
              </List.Item>
            )}
            locale={{ emptyText: "暂无历史记录" }}
          />
        </Card>
      )}
    </div>
  );
};

export default AIAnalysis;
