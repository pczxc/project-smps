import React from "react";
import { Layout as AntLayout, Menu, Typography } from "antd";
import {
  DashboardOutlined,
  AppstoreOutlined,
  FilterOutlined,
  LineChartOutlined,
  WalletOutlined,
  RobotOutlined,
  SettingOutlined,
  StockOutlined,
} from "@ant-design/icons";
import { useNavigate, useLocation } from "react-router-dom";

const { Sider, Content } = AntLayout;
const { Title } = Typography;

interface LayoutProps {
  children: React.ReactNode;
}

const Layout: React.FC<LayoutProps> = ({ children }) => {
  const navigate = useNavigate();
  const location = useLocation();

  const menuItems = [
    { key: "/", icon: <DashboardOutlined />, label: "仪表盘" },
    { key: "/stock-info", icon: <StockOutlined />, label: "股票信息" },
    { key: "/pools", icon: <AppstoreOutlined />, label: "股票池" },
    { key: "/filter", icon: <FilterOutlined />, label: "股票筛选" },
    { key: "/analysis", icon: <LineChartOutlined />, label: "智能分析" },
    { key: "/positions", icon: <WalletOutlined />, label: "持仓管理" },
    { key: "/ai-analysis", icon: <RobotOutlined />, label: "AI分析" },
    { key: "/settings", icon: <SettingOutlined />, label: "系统设置" },
  ];

  return (
    <AntLayout style={{ minHeight: "100vh", background: "#0a0e1a" }}>
      <Sider
        width={200}
        style={{
          background: "#111827",
          borderRight: "1px solid #374151",
        }}
      >
        <div style={{ padding: "20px", textAlign: "center" }}>
          <Title level={4} style={{ color: "#3b82f6", margin: 0 }}>
            股票投资助手
          </Title>
        </div>
        <Menu
          mode="inline"
          selectedKeys={[location.pathname]}
          style={{
            background: "transparent",
            borderRight: "none",
          }}
          items={menuItems.map((item) => ({
            key: item.key,
            icon: item.icon,
            label: item.label,
            onClick: () => navigate(item.key),
          }))}
        />
      </Sider>
      <Content style={{ padding: "24px", background: "#0a0e1a" }}>
        {children}
      </Content>
    </AntLayout>
  );
};

export default Layout;
