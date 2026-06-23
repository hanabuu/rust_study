use std::sync::LazyLock;

type SchemaStatement = (&'static str, String);

const RAW_SCHEMA: &str = include_str!("schema.sql");

static STATEMENTS: LazyLock<Vec<SchemaStatement>> = LazyLock::new(parse_statements);

pub fn statements() -> &'static [SchemaStatement] {
    STATEMENTS.as_slice()
}

fn parse_statements() -> Vec<SchemaStatement> {
    RAW_SCHEMA
        .split("-- statement: ")
        .filter_map(|block| {
            let block = block.trim();
            if block.is_empty() {
                return None;
            }

            let (name, sql) = block
                .split_once('\n')
                .expect("schema.sql statement block must start with a name line");

            Some((name.trim(), wrap_with_drop_if_exists(sql.trim())))
        })
        .collect()
}

fn wrap_with_drop_if_exists(sql: &str) -> String {
    let trimmed = sql.trim_start();

    if let Some(name) = extract_name(trimmed, "CREATE TABLE ") {
        return format!("DROP TABLE IF EXISTS {name};\n{trimmed}");
    }

    if let Some(name) = extract_name(trimmed, "CREATE VIEW ") {
        return format!("DROP VIEW IF EXISTS {name};\n{trimmed}");
    }

    if let Some(name) = extract_name(trimmed, "CREATE INDEX ") {
        return format!("DROP INDEX IF EXISTS {name};\n{trimmed}");
    }

    trimmed.to_string()
}

fn extract_name<'a>(sql: &'a str, prefix: &str) -> Option<&'a str> {
    let rest = sql.strip_prefix(prefix)?;

    if let Some(stripped) = rest.strip_prefix('[') {
        let end = stripped.find(']')?;
        return Some(&rest[..=end + 1]);
    }

    let end = rest
        .find(|character: char| character.is_ascii_whitespace() || character == '(')
        .unwrap_or(rest.len());
    Some(&rest[..end])
}
