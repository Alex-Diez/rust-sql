extern crate sql_ast;

use sql_ast::SQLQueryParser;
use sql_ast::SQLNode;
use sql_ast::SQLParseError;

#[test]
fn test_create_sql_query_parser() {
    SQLQueryParser::new("test".to_string());
}

#[test]
fn test_parse_string_to_sql_ast() {
    let result = SQLQueryParser::new("test".to_string())
            .parse_query("select t1.c1 from table1 t1".to_string());
    assert!(result.is_ok());
}

#[test]
fn test_create_sql_node() {
    SQLNode::new("value".to_string());
}
