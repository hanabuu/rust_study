/// 果物を表す構造体とその実装

/// 属性
pub struct Fruits {
    name: String,
    color: String,
}

/// Fruits構造体の実装
impl Fruits {
    /// コンストラクタ
    pub fn new(name: &str, color: &str) -> Self {
        Self {
            name: name.to_string(),
            color: color.to_string(),
        }
    }

    /// 説明文を返すメソッド
    pub fn describe(&self) -> String {
        format!("This fruit is a {} and its color is {}.", self.name, self.color)
    }
}
