use once_cell::sync::OnceCell;
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;
use std::sync::RwLock;

/// Global config store — mirrors Laravel's `config('app.name')` helper.
/// Values are loaded from TOML files in `config/` at boot.
pub struct Config {
    data: RwLock<HashMap<String, Value>>,
}

static CONFIG: OnceCell<Config> = OnceCell::new();

impl Config {
    pub fn global() -> &'static Config {
        CONFIG.get_or_init(|| Config {
            data: RwLock::new(HashMap::new()),
        })
    }

    /// Load every `*.toml` file inside `config_dir` — filename (without ext)
    /// becomes the top-level key. `config/app.toml` → `config('app.xxx')`.
    pub fn load_dir<P: AsRef<Path>>(config_dir: P) -> std::io::Result<()> {
        let dir = config_dir.as_ref();
        if !dir.exists() {
            return Ok(());
        }
        let cfg = Self::global();
        let mut data = cfg.data.write().unwrap();
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("toml") {
                let name = path.file_stem().unwrap().to_string_lossy().to_string();
                let contents = std::fs::read_to_string(&path)?;
                let parsed: toml::Value = match toml::from_str(&contents) {
                    Ok(v) => v,
                    Err(e) => {
                        tracing::warn!("Failed to parse {}: {e}", path.display());
                        continue;
                    }
                };
                let json = toml_to_json(parsed);
                data.insert(name, json);
            }
        }
        Ok(())
    }

    /// Read a dotted config path — `get("app.name")` reads `name` from
    /// `config/app.toml`.
    pub fn get(&self, key: &str) -> Option<Value> {
        let data = self.data.read().unwrap();
        let mut parts = key.split('.');
        let first = parts.next()?;
        let mut current = data.get(first)?.clone();
        for p in parts {
            current = current.get(p)?.clone();
        }
        Some(current)
    }

    pub fn get_string(&self, key: &str) -> Option<String> {
        self.get(key).and_then(|v| v.as_str().map(|s| s.to_string()))
    }

    pub fn get_i64(&self, key: &str) -> Option<i64> {
        self.get(key).and_then(|v| v.as_i64())
    }

    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.get(key).and_then(|v| v.as_bool())
    }

    pub fn set(&self, key: &str, value: Value) {
        let mut data = self.data.write().unwrap();
        let mut parts: Vec<&str> = key.split('.').collect();
        let first = parts.remove(0);
        if parts.is_empty() {
            data.insert(first.to_string(), value);
            return;
        }
        let entry = data.entry(first.to_string()).or_insert_with(|| Value::Object(Default::default()));
        insert_at(entry, &parts, value);
    }
}

fn insert_at(current: &mut Value, path: &[&str], value: Value) {
    if path.is_empty() {
        *current = value;
        return;
    }
    if !current.is_object() {
        *current = Value::Object(Default::default());
    }
    let obj = current.as_object_mut().unwrap();
    let head = path[0];
    if path.len() == 1 {
        obj.insert(head.to_string(), value);
    } else {
        let child = obj.entry(head.to_string()).or_insert_with(|| Value::Object(Default::default()));
        insert_at(child, &path[1..], value);
    }
}

fn toml_to_json(v: toml::Value) -> Value {
    match v {
        toml::Value::String(s) => Value::String(s),
        toml::Value::Integer(i) => Value::Number(i.into()),
        toml::Value::Float(f) => serde_json::Number::from_f64(f).map(Value::Number).unwrap_or(Value::Null),
        toml::Value::Boolean(b) => Value::Bool(b),
        toml::Value::Datetime(dt) => Value::String(dt.to_string()),
        toml::Value::Array(a) => Value::Array(a.into_iter().map(toml_to_json).collect()),
        toml::Value::Table(t) => {
            let mut m = serde_json::Map::new();
            for (k, v) in t {
                m.insert(k, toml_to_json(v));
            }
            Value::Object(m)
        }
    }
}

/// Convenience free function — `config("app.name")`.
pub fn config(key: &str) -> Option<Value> {
    Config::global().get(key)
}
