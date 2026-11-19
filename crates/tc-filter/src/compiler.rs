use crate::query::{FilterNode, FilterQuery, LogicalOp, MatchOp};
use rusqlite::types::ToSql;
use tc_core::{Result, TcError};

pub struct CompiledQuery {
    pub sql: String,
    pub params: Vec<Box<dyn ToSql>>,
}

pub struct QueryCompiler;

impl QueryCompiler {
    pub fn compile_posts(query: &FilterQuery) -> Result<CompiledQuery> {
        let mut where_clause = String::new();
        let mut params: Vec<Box<dyn ToSql>> = Vec::new();

        if let Some(ref root) = query.root {
            let (expr, node_params) = Self::compile_node(root, "p")?;
            if !expr.is_empty() {
                where_clause = format!("WHERE {}", expr);
                params = node_params;
            }
        }

        let order_by = match query.sort.by.as_str() {
            "created_at" => "p.created_at",
            "reply_count" => "COALESCE(p.reply_count, 0)",
            "like_count" => "COALESCE(p.like_count, 0)",
            _ => "p.created_at",
        };

        let dir = if query.sort.dir.eq_ignore_ascii_case("asc") {
            "ASC"
        } else {
            "DESC"
        };

        let sql = format!(
            "SELECT p.id, p.account_id, p.text, p.author, p.created_at, p.permalink, p.media_type, p.reply_count, p.like_count, p.raw_json
             FROM post p {} ORDER BY {} {}, p.id ASC LIMIT {}",
            where_clause, order_by, dir, query.limit
        );

        Ok(CompiledQuery { sql, params })
    }

    fn compile_node(
        node: &FilterNode,
        table_prefix: &str,
    ) -> Result<(String, Vec<Box<dyn ToSql>>)> {
        match node {
            FilterNode::Group { op, children } => {
                if children.is_empty() {
                    return Ok((String::new(), Vec::new()));
                }

                let mut sub_exprs = Vec::new();
                let mut all_params: Vec<Box<dyn ToSql>> = Vec::new();

                for child in children {
                    let (expr, params) = Self::compile_node(child, table_prefix)?;
                    if !expr.is_empty() {
                        sub_exprs.push(expr);
                        all_params.extend(params);
                    }
                }

                if sub_exprs.is_empty() {
                    return Ok((String::new(), Vec::new()));
                }

                let joiner = match op {
                    LogicalOp::And => " AND ",
                    LogicalOp::Or => " OR ",
                    LogicalOp::Not => " AND NOT ",
                };

                if *op == LogicalOp::Not {
                    Ok((format!("NOT ({})", sub_exprs.join(" AND ")), all_params))
                } else {
                    Ok((format!("({})", sub_exprs.join(joiner)), all_params))
                }
            }
            FilterNode::Field {
                field,
                match_op,
                value,
            } => {
                let col = match field.as_str() {
                    "text" => format!("{}.text", table_prefix),
                    "author" => format!("{}.author", table_prefix),
                    "created_at" => format!("{}.created_at", table_prefix),
                    "reply_count" => format!("{}.reply_count", table_prefix),
                    "like_count" => format!("{}.like_count", table_prefix),
                    _ => return Err(TcError::InvalidTimeline(format!("Unknown field: {field}"))),
                };

                match match_op {
                    MatchOp::Contains => {
                        let val_str = value
                            .as_str()
                            .ok_or_else(|| TcError::InvalidTimeline("Expected string value".into()))?;
                        let param = format!("%{}%", val_str);
                        Ok((format!("{} LIKE ?", col), vec![Box::new(param)]))
                    }
                    MatchOp::Exact => {
                        if let Some(s) = value.as_str() {
                            Ok((format!("{} = ?", col), vec![Box::new(s.to_string())]))
                        } else if let Some(n) = value.as_i64() {
                            Ok((format!("{} = ?", col), vec![Box::new(n)]))
                        } else {
                            Err(TcError::InvalidTimeline("Unsupported exact value".into()))
                        }
                    }
                    MatchOp::Gte => {
                        let n = value
                            .as_i64()
                            .ok_or_else(|| TcError::InvalidTimeline("Expected integer".into()))?;
                        Ok((format!("{} >= ?", col), vec![Box::new(n)]))
                    }
                    MatchOp::Lte => {
                        let n = value
                            .as_i64()
                            .ok_or_else(|| TcError::InvalidTimeline("Expected integer".into()))?;
                        Ok((format!("{} <= ?", col), vec![Box::new(n)]))
                    }
                    MatchOp::Between => {
                        let arr = value
                            .as_array()
                            .ok_or_else(|| TcError::InvalidTimeline("Expected array of 2 values".into()))?;
                        if arr.len() != 2 {
                            return Err(TcError::InvalidTimeline("Between requires 2 elements".into()));
                        }
                        let v1 = arr[0].as_i64().unwrap_or(0);
                        let v2 = arr[1].as_i64().unwrap_or(i64::MAX);
                        Ok((
                            format!("{} BETWEEN ? AND ?", col),
                            vec![Box::new(v1), Box::new(v2)],
                        ))
                    }
                    MatchOp::In => {
                        let arr = value
                            .as_array()
                            .ok_or_else(|| TcError::InvalidTimeline("Expected array".into()))?;
                        if arr.is_empty() {
                            return Ok(("1 = 0".to_string(), Vec::new()));
                        }
                        let mut placeholders = Vec::new();
                        let mut params: Vec<Box<dyn ToSql>> = Vec::new();
                        for v in arr {
                            if let Some(s) = v.as_str() {
                                placeholders.push("?");
                                params.push(Box::new(s.to_string()));
                            }
                        }
                        Ok((
                            format!("{} IN ({})", col, placeholders.join(",")),
                            params,
                        ))
                    }
                }
            }
        }
    }
}
