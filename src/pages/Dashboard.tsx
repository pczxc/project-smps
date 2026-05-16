import React from "react";
import { Card, Row, Col, Statistic, Typography } from "antd";
import { ArrowUpOutlined, WalletOutlined, BellOutlined } from "@ant-design/icons";

const { Title } = Typography;

const Dashboard: React.FC = () => {
  return (
    <div>
      <Title level={2} style={{ color: "#f9fafb", marginBottom: "24px" }}>
        仪表盘
      </Title>
      
      <Row gutter={[16, 16]}>
        <Col span={8}>
          <Card style={{ background: "#111827", border: "1px solid #374151" }}>
            <Statistic
              title="总资产"
              value={125800.5}
              precision={2}
              valueStyle={{ color: "#f9fafb" }}
              prefix="¥"
            />
          </Card>
        </Col>
        <Col span={8}>
          <Card style={{ background: "#111827", border: "1px solid #374151" }}>
            <Statistic
              title="今日盈亏"
              value={2580.3}
              precision={2}
              valueStyle={{ color: "#10b981" }}
              prefix={<ArrowUpOutlined />}
              suffix="¥"
            />
          </Card>
        </Col>
        <Col span={8}>
          <Card style={{ background: "#111827", border: "1px solid #374151" }}>
            <Statistic
              title="收益率"
              value={2.05}
              precision={2}
              valueStyle={{ color: "#10b981" }}
              prefix={<ArrowUpOutlined />}
              suffix="%"
            />
          </Card>
        </Col>
      </Row>

      <Row gutter={[16, 16]} style={{ marginTop: "16px" }}>
        <Col span={12}>
          <Card 
            title="持仓概览" 
            style={{ background: "#111827", border: "1px solid #374151" }}
            headStyle={{ color: "#f9fafb", borderBottom: "1px solid #374151" }}
          >
            <div style={{ textAlign: "center", padding: "40px" }}>
              <WalletOutlined style={{ fontSize: "48px", color: "#3b82f6" }} />
              <p style={{ color: "#9ca3af", marginTop: "16px" }}>
                您当前持有 5 只股票
              </p>
            </div>
          </Card>
        </Col>
        <Col span={12}>
          <Card 
            title="今日提醒" 
            style={{ background: "#111827", border: "1px solid #374151" }}
            headStyle={{ color: "#f9fafb", borderBottom: "1px solid #374151" }}
          >
            <div style={{ textAlign: "center", padding: "40px" }}>
              <BellOutlined style={{ fontSize: "48px", color: "#8b5cf6" }} />
              <p style={{ color: "#9ca3af", marginTop: "16px" }}>
                今日有 2 只股票触发监控条件
              </p>
            </div>
          </Card>
        </Col>
      </Row>
    </div>
  );
};

export default Dashboard;
