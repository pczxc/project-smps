import React from "react";
import { HashRouter as Router, Routes, Route } from "react-router-dom";
import Layout from "./components/Layout";
import Dashboard from "./pages/Dashboard";
import StockPools from "./pages/StockPools";
import StockFilter from "./pages/StockFilter";
import Analysis from "./pages/Analysis";
import Positions from "./pages/Positions";
import AIAnalysis from "./pages/AIAnalysis";
import Settings from "./pages/Settings";
import StockInfo from "./pages/StockInfo";

const App: React.FC = () => {
  return (
    <Router>
      <Layout>
        <Routes>
          <Route path="/" element={<Dashboard />} />
          <Route path="/stock-info" element={<StockInfo />} />
          <Route path="/pools" element={<StockPools />} />
          <Route path="/filter" element={<StockFilter />} />
          <Route path="/analysis" element={<Analysis />} />
          <Route path="/positions" element={<Positions />} />
          <Route path="/ai-analysis" element={<AIAnalysis />} />
          <Route path="/settings" element={<Settings />} />
        </Routes>
      </Layout>
    </Router>
  );
};

export default App;
