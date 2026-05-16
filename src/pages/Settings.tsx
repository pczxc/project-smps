import React from "react";
import { Card, Form, Input, Button, Switch, message, Tabs } from "antd";
import { LockOutlined, DatabaseOutlined, SkinOutlined, UploadOutlined, DownloadOutlined } from "@ant-design/icons";
import { invoke } from "@tauri-apps/api/core";

const Settings: React.FC = () => {
  const [passwordForm] = Form.useForm();
  const [apiKeyForm] = Form.useForm();

  const handleSetPassword = async (values: any) => {
    try {
      await invoke("set_password", { password: values.password });
      message.success("密码设置成功");
      passwordForm.resetFields();
    } catch (error) {
      message.error("密码设置失败");
    }
  };

  const handleBackup = async () => {
    try {
      const path = await invoke<string>("backup_database");
      message.success(`备份成功: ${path}`);
    } catch (error) {
      message.error("备份失败");
    }
  };

  const handleSaveApiKey = async (values: any) => {
    try {
      await invoke("save_api_key", {
        modelType: values.model_type,
        apiKey: values.api_key,
      });
      message.success("API密钥保存成功");
      apiKeyForm.resetFields();
    } catch (error) {
      message.error("保存失败");
    }
  };

  const items = [
    {
      key: "security",
      label: "安全设置",
      icon: <LockOutlined />,
      children: (
        <Card style={{ background: "#111827", border: "1px solid #374151" }}>
          <h3 style={{ color: "#f9fafb", marginBottom: "16px" }}>启动密码</h3>
          <Form form={passwordForm} onFinish={handleSetPassword} layout="vertical">
            <Form.Item
              name="password"
              label="新密码"
              rules={[{ required: true, message: "请输入密码" }]}
            >
              <Input.Password placeholder="请输入启动密码" />
            </Form.Item>
            <Form.Item
              name="confirm_password"
              label="确认密码"
              rules={[
                { required: true, message: "请确认密码" },
                ({ getFieldValue }) => ({
                  validator(_, value) {
                    if (!value || getFieldValue("password") === value) {
                      return Promise.resolve();
                    }
                    return Promise.reject(new Error("两次输入的密码不一致"));
                  },
                }),
              ]}
            >
              <Input.Password placeholder="请再次输入密码" />
            </Form.Item>
            <Form.Item>
              <Button type="primary" htmlType="submit">
                设置密码
              </Button>
            </Form.Item>
          </Form>
        </Card>
      ),
    },
    {
      key: "api",
      label: "API配置",
      icon: <DatabaseOutlined />,
      children: (
        <Card style={{ background: "#111827", border: "1px solid #374151" }}>
          <h3 style={{ color: "#f9fafb", marginBottom: "16px" }}>大模型API密钥</h3>
          <Form form={apiKeyForm} onFinish={handleSaveApiKey} layout="vertical">
            <Form.Item
              name="model_type"
              label="模型类型"
              rules={[{ required: true, message: "请选择模型类型" }]}
            >
              <Input placeholder="如: doubao, wenxin, qianwen" />
            </Form.Item>
            <Form.Item
              name="api_key"
              label="API密钥"
              rules={[{ required: true, message: "请输入API密钥" }]}
            >
              <Input.Password placeholder="请输入API密钥" />
            </Form.Item>
            <Form.Item>
              <Button type="primary" htmlType="submit">
                保存密钥
              </Button>
            </Form.Item>
          </Form>
        </Card>
      ),
    },
    {
      key: "data",
      label: "数据管理",
      icon: <DatabaseOutlined />,
      children: (
        <Card style={{ background: "#111827", border: "1px solid #374151" }}>
          <h3 style={{ color: "#f9fafb", marginBottom: "16px" }}>数据备份与恢复</h3>
          <div style={{ display: "flex", gap: "16px" }}>
            <Button type="primary" icon={<DownloadOutlined />} onClick={handleBackup}>
              备份数据库
            </Button>
            <Button icon={<UploadOutlined />}>
              恢复数据库
            </Button>
          </div>
        </Card>
      ),
    },
    {
      key: "appearance",
      label: "外观设置",
      icon: <SkinOutlined />,
      children: (
        <Card style={{ background: "#111827", border: "1px solid #374151" }}>
          <h3 style={{ color: "#f9fafb", marginBottom: "16px" }}>主题设置</h3>
          <div style={{ display: "flex", alignItems: "center", gap: "16px" }}>
            <span style={{ color: "#f9fafb" }}>深色模式</span>
            <Switch defaultChecked disabled />
            <span style={{ color: "#9ca3af" }}>当前仅支持深色模式</span>
          </div>
        </Card>
      ),
    },
  ];

  return (
    <div>
      <h2 style={{ color: "#f9fafb", marginBottom: "24px" }}>系统设置</h2>
      <Tabs
        items={items}
        style={{ color: "#f9fafb" }}
      />
    </div>
  );
};

export default Settings;
