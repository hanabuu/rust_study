/// REST URL に安全に含められるようパスの一部をパーセントエンコードする。
///
/// # 引数
/// * `segment` - URL に組み込むパス片。
///
/// # 戻り値
/// エンコード済み文字列。
pub fn encode_segment(segment: &str) -> String {
    urlencoding::encode(segment).into_owned()
}

/// ベース URL と任意のクエリを結合しログ等で表示しやすい形に整える。
///
/// # 引数
/// * `base` - クエリを付与する元の URL。
/// * `query` - `(&str, u32)` 形式のクエリパラメータ集合。
///
/// # 戻り値
/// クエリを含んだ URL 文字列。
pub fn build_requested_url(base: &str, query: &[(&str, u32)]) -> String {
    if query.is_empty() {
        base.to_string()
    } else {
        let query_string = query
            .iter()
            .map(|(key, value)| format!("{}={}", key, value))
            .collect::<Vec<_>>()
            .join("&");
        format!("{}?{}", base, query_string)
    }
}