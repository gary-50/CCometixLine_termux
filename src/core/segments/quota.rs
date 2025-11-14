use super::{Segment, SegmentData};
use crate::config::{InputData, SegmentId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

// API 响应结构
#[derive(Debug, Deserialize)]
struct C88ApiResponse {
    #[serde(rename = "creditLimit")]
    credit_limit: f64,
    #[serde(rename = "currentCredits")]
    current_credits: f64,
    #[serde(rename = "subscriptionName")]
    subscription_name: Option<String>,
}

// 端点配置
#[derive(Debug, Clone)]
struct EndpointConfig {
    url: String,
    name: String,
}

// 端点缓存
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
struct EndpointCache {
    api_key_hash: u64,
    successful_endpoint: String,
    last_success_time: SystemTime,
    success_count: u32,
}

// 配额数据缓存
#[derive(Debug, Clone, Serialize, Deserialize)]
struct QuotaCache {
    subscription_name: Option<String>,
    remaining: f64,
    total: f64,
    last_update: SystemTime,
    consecutive_failures: u32,
}

// 智能端点检测器
struct SmartEndpointDetector {
    endpoints: Vec<EndpointConfig>,
}

impl SmartEndpointDetector {
    fn new() -> Self {
        let endpoints = vec![EndpointConfig {
            url: "https://www.88code.org/api/usage".to_string(),
            name: "main".to_string(),
        }];

        Self { endpoints }
    }

    #[allow(dead_code)]
    fn get_cache_file_path() -> PathBuf {
        if let Some(home) = dirs::home_dir() {
            home.join(".claude")
                .join("ccline")
                .join("endpoint_cache.json")
        } else {
            PathBuf::from("endpoint_cache.json")
        }
    }

    #[allow(dead_code)]
    fn hash_api_key(api_key: &str) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        api_key.hash(&mut hasher);
        hasher.finish()
    }

    fn try_endpoint(&self, endpoint: &EndpointConfig, api_key: &str) -> Option<C88ApiResponse> {
        let debug = env::var("C88_DEBUG").is_ok();

        if debug {
            eprintln!("[DEBUG] Trying endpoint: {}", endpoint.url);
        }

        let start_time = SystemTime::now();
        let bearer_token = format!("Bearer {}", api_key);
        let result = ureq::post(&endpoint.url)
            .set("accept", "*/*")
            .set("content-type", "application/json")
            .set("Authorization", &bearer_token)
            .timeout(Duration::from_secs(30))
            .call();

        match result {
            Ok(response) => {
                if response.status() == 200 {
                    let elapsed = start_time.elapsed().unwrap_or(Duration::from_secs(0));
                    if debug {
                        eprintln!(
                            "[DEBUG] Success: {} in {}ms",
                            endpoint.name,
                            elapsed.as_millis()
                        );
                    }

                    response.into_json::<C88ApiResponse>().ok()
                } else {
                    if debug {
                        eprintln!(
                            "[DEBUG] Failed: {} status {}",
                            endpoint.name,
                            response.status()
                        );
                    }
                    None
                }
            }
            Err(e) => {
                if debug {
                    eprintln!("[DEBUG] Error: {} - {}", endpoint.name, e);
                }
                None
            }
        }
    }

    fn detect_endpoint(&mut self, api_key: &str) -> Option<(String, C88ApiResponse)> {
        // 尝试所有端点
        let endpoints_clone = self.endpoints.clone();
        for endpoint in &endpoints_clone {
            if let Some(response) = self.try_endpoint(endpoint, api_key) {
                return Some((endpoint.url.clone(), response));
            }
        }

        None
    }

    fn detect_endpoint_static(api_key: &str) -> Option<(String, C88ApiResponse)> {
        let mut detector = SmartEndpointDetector::new();
        detector.detect_endpoint(api_key)
    }
}

#[derive(Default)]
pub struct QuotaSegment;

impl QuotaSegment {
    pub fn new() -> Self {
        Self
    }

    fn get_cache_file_path() -> PathBuf {
        if let Some(home) = dirs::home_dir() {
            home.join(".claude").join("ccline").join("quota_cache.json")
        } else {
            PathBuf::from("quota_cache.json")
        }
    }

    fn load_cache() -> Option<QuotaCache> {
        let cache_path = Self::get_cache_file_path();
        if let Ok(content) = fs::read_to_string(cache_path) {
            serde_json::from_str(&content).ok()
        } else {
            None
        }
    }

    fn save_cache(cache: &QuotaCache) -> Result<(), Box<dyn std::error::Error>> {
        let cache_path = Self::get_cache_file_path();

        // 确保目录存在
        if let Some(parent) = cache_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(cache)?;
        fs::write(cache_path, content)?;
        Ok(())
    }

    fn load_api_key(&self) -> Option<String> {
        // 优先级：环境变量 > Claude Code settings.json > api_key 文件

        // 1. 环境变量
        if let Ok(key) = env::var("C88_API_KEY") {
            return Some(key);
        }

        if let Ok(key) = env::var("ANTHROPIC_API_KEY") {
            return Some(key);
        }

        if let Ok(key) = env::var("ANTHROPIC_AUTH_TOKEN") {
            return Some(key);
        }

        // 2. Claude Code settings.json
        if let Some(key) = self.load_from_settings() {
            return Some(key);
        }

        // 3. api_key 文件
        if let Some(home) = dirs::home_dir() {
            let api_key_path = home.join(".claude").join("api_key");
            if let Ok(key) = fs::read_to_string(api_key_path) {
                return Some(key.trim().to_string());
            }
        }

        None
    }

    fn load_from_settings(&self) -> Option<String> {
        if let Some(home) = dirs::home_dir() {
            let settings_path = home.join(".claude").join("settings.json");
            if let Ok(content) = fs::read_to_string(settings_path) {
                if let Ok(settings) = serde_json::from_str::<serde_json::Value>(&content) {
                    if let Some(env) = settings.get("env") {
                        if let Some(token) = env.get("ANTHROPIC_AUTH_TOKEN") {
                            if let Some(token_str) = token.as_str() {
                                return Some(token_str.to_string());
                            }
                        }
                        if let Some(key) = env.get("ANTHROPIC_API_KEY") {
                            if let Some(key_str) = key.as_str() {
                                return Some(key_str.to_string());
                            }
                        }
                    }
                }
            }
        }
        None
    }

    fn format_quota(&self, subscription_name: Option<&str>, remaining: f64) -> String {
        if let Some(name) = subscription_name {
            format!("{} ${:.2}", name, remaining)
        } else {
            format!("${:.2}", remaining)
        }
    }

    fn calculate_used(&self, response: &C88ApiResponse) -> f64 {
        response.credit_limit - response.current_credits
    }
}

impl Segment for QuotaSegment {
    fn collect(&self, _input: &InputData) -> Option<SegmentData> {
        #[cfg(not(feature = "quota"))]
        {
            return None;
        }

        #[cfg(feature = "quota")]
        {
            let api_key = self.load_api_key()?;

            // 加载缓存
            let mut cache = Self::load_cache();

            // 尝试调用API
            if let Some((endpoint_url, response)) =
                SmartEndpointDetector::detect_endpoint_static(&api_key)
            {
                // API调用成功
                let remaining = response.current_credits;
                let total = response.credit_limit;
                let quota_display =
                    self.format_quota(response.subscription_name.as_deref(), remaining);

                // 更新缓存（重置失败计数）
                let new_cache = QuotaCache {
                    subscription_name: response.subscription_name.clone(),
                    remaining,
                    total,
                    last_update: SystemTime::now(),
                    consecutive_failures: 0,
                };
                let _ = Self::save_cache(&new_cache);

                let mut metadata = HashMap::new();
                metadata.insert("remaining".to_string(), remaining.to_string());
                metadata.insert("total".to_string(), total.to_string());
                metadata.insert("endpoint_used".to_string(), endpoint_url);
                if let Some(name) = &response.subscription_name {
                    metadata.insert("subscription_name".to_string(), name.clone());
                }

                Some(SegmentData {
                    primary: quota_display,
                    secondary: String::new(),
                    metadata,
                })
            } else {
                // API调用失败
                // 增加失败计数
                if let Some(ref mut cached) = cache {
                    cached.consecutive_failures += 1;
                    let _ = Self::save_cache(cached);

                    // 如果连续失败少于3次，显示缓存的数据
                    if cached.consecutive_failures < 3 {
                        let quota_display = self
                            .format_quota(cached.subscription_name.as_deref(), cached.remaining);

                        let mut metadata = HashMap::new();
                        metadata.insert("remaining".to_string(), cached.remaining.to_string());
                        metadata.insert("total".to_string(), cached.total.to_string());
                        metadata.insert("cached".to_string(), "true".to_string());
                        metadata.insert(
                            "failures".to_string(),
                            cached.consecutive_failures.to_string(),
                        );
                        if let Some(name) = &cached.subscription_name {
                            metadata.insert("subscription_name".to_string(), name.clone());
                        }

                        return Some(SegmentData {
                            primary: quota_display,
                            secondary: String::new(),
                            metadata,
                        });
                    }
                }

                // 连续失败3次或没有缓存，显示Offline
                let mut metadata = HashMap::new();
                metadata.insert("status".to_string(), "offline".to_string());

                Some(SegmentData {
                    primary: "Offline".to_string(),
                    secondary: String::new(),
                    metadata,
                })
            }
        }
    }

    fn id(&self) -> SegmentId {
        SegmentId::Quota
    }
}
