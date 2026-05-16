use crate::models::StockInfo;
use reqwest;
use serde::{Deserialize, Serialize};
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize)]
pub struct RawStockData {
    pub code: String,
    pub name: String,
    pub market: String,
    pub exchange: String,
    pub list_date: Option<String>,
    pub industry: Option<String>,
    pub area: Option<String>,
}

pub async fn fetch_all_astocks() -> Result<Vec<RawStockData>, String> {
    // 获取A股股票列表 - 使用模拟数据作为替代
    // 实际项目中可以对接雪球、东方财富、新浪财经等公开数据源
    let stocks = vec![
        RawStockData {
            code: "000001".to_string(),
            name: "平安银行".to_string(),
            market: "A".to_string(),
            exchange: "SZSE".to_string(),
            list_date: Some("1991-04-03".to_string()),
            industry: Some("银行".to_string()),
            area: Some("广东".to_string()),
        },
        RawStockData {
            code: "000002".to_string(),
            name: "万科A".to_string(),
            market: "A".to_string(),
            exchange: "SZSE".to_string(),
            list_date: Some("1991-01-29".to_string()),
            industry: Some("房地产".to_string()),
            area: Some("广东".to_string()),
        },
        RawStockData {
            code: "600000".to_string(),
            name: "浦发银行".to_string(),
            market: "A".to_string(),
            exchange: "SSE".to_string(),
            list_date: Some("1999-11-10".to_string()),
            industry: Some("银行".to_string()),
            area: Some("上海".to_string()),
        },
        RawStockData {
            code: "600036".to_string(),
            name: "招商银行".to_string(),
            market: "A".to_string(),
            exchange: "SSE".to_string(),
            list_date: Some("2002-04-09".to_string()),
            industry: Some("银行".to_string()),
            area: Some("广东".to_string()),
        },
        RawStockData {
            code: "600519".to_string(),
            name: "贵州茅台".to_string(),
            market: "A".to_string(),
            exchange: "SSE".to_string(),
            list_date: Some("2001-08-27".to_string()),
            industry: Some("白酒".to_string()),
            area: Some("贵州".to_string()),
        },
        RawStockData {
            code: "000858".to_string(),
            name: "五粮液".to_string(),
            market: "A".to_string(),
            exchange: "SZSE".to_string(),
            list_date: Some("1998-04-27".to_string()),
            industry: Some("白酒".to_string()),
            area: Some("四川".to_string()),
        },
        RawStockData {
            code: "000063".to_string(),
            name: "中兴通讯".to_string(),
            market: "A".to_string(),
            exchange: "SZSE".to_string(),
            list_date: Some("1997-11-18".to_string()),
            industry: Some("通信设备".to_string()),
            area: Some("广东".to_string()),
        },
        RawStockData {
            code: "600050".to_string(),
            name: "中国联通".to_string(),
            market: "A".to_string(),
            exchange: "SSE".to_string(),
            list_date: Some("2002-10-09".to_string()),
            industry: Some("通信服务".to_string()),
            area: Some("北京".to_string()),
        },
    ];
    
    Ok(stocks)
}

pub async fn fetch_all_hstocks() -> Result<Vec<RawStockData>, String> {
    // 获取H股股票列表 - 使用模拟数据作为替代
    let stocks = vec![
        RawStockData {
            code: "00700".to_string(),
            name: "腾讯控股".to_string(),
            market: "H".to_string(),
            exchange: "HKEX".to_string(),
            list_date: Some("2004-06-16".to_string()),
            industry: Some("互联网".to_string()),
            area: Some("广东".to_string()),
        },
        RawStockData {
            code: "00941".to_string(),
            name: "中国移动".to_string(),
            market: "H".to_string(),
            exchange: "HKEX".to_string(),
            list_date: Some("1997-10-23".to_string()),
            industry: Some("通信服务".to_string()),
            area: Some("北京".to_string()),
        },
        RawStockData {
            code: "00001".to_string(),
            name: "长和".to_string(),
            market: "H".to_string(),
            exchange: "HKEX".to_string(),
            list_date: Some("2015-06-03".to_string()),
            industry: Some("综合企业".to_string()),
            area: Some("香港".to_string()),
        },
        RawStockData {
            code: "00939".to_string(),
            name: "建设银行".to_string(),
            market: "H".to_string(),
            exchange: "HKEX".to_string(),
            list_date: Some("2005-10-27".to_string()),
            industry: Some("银行".to_string()),
            area: Some("北京".to_string()),
        },
        RawStockData {
            code: "01398".to_string(),
            name: "工商银行".to_string(),
            market: "H".to_string(),
            exchange: "HKEX".to_string(),
            list_date: Some("2006-10-27".to_string()),
            industry: Some("银行".to_string()),
            area: Some("北京".to_string()),
        },
    ];
    
    Ok(stocks)
}

pub fn raw_to_stock_info(raw: RawStockData) -> StockInfo {
    StockInfo {
        id: 0,
        code: raw.code,
        name: raw.name,
        market: raw.market,
        exchange: raw.exchange,
        list_date: raw.list_date,
        industry: raw.industry,
        area: raw.area,
        synced_at: Utc::now().to_rfc3339(),
    }
}
