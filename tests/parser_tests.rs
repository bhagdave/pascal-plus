use pascal_plus::parser::{PascalParser, PestParser, Rule};
use pest::Parser;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_program() {
        let input = r#"
            program HelloWorld;
            begin
                print("Hello World");
            end.
        "#;
        
        let result = PascalParser::parse(Rule::program, input);
        assert!(result.is_ok(), "Failed to parse simple program");
    }

    #[test]
    fn test_program_with_function() {
        let input = r#"
            program Test;
            
            function greet(name: String): String
            begin
                return "Hello, " + name;
            end
            
            begin
                let message: String := greet("World");
                print(message);
            end.
        "#;
        
        let result = PascalParser::parse(Rule::program, input);
        assert!(result.is_ok(), "Failed to parse program with function");
    }

    #[test]
    fn test_variable_declaration() {
        let input = r#"let x: Integer := 42;"#;
        
        let result = PascalParser::parse(Rule::variable_decl, input);
        assert!(result.is_ok(), "Failed to parse variable declaration");
    }

    #[test]
    fn test_function_declaration() {
        let input = r#"function add(a: Integer, b: Integer): Integer
            begin
                return a + b;
            end"#;
        
        let result = PascalParser::parse(Rule::function_decl, input);
        assert!(result.is_ok(), "Failed to parse function declaration");
    }

    #[test]
    fn test_function_call_statement() {
        let input = r#"print("Hello");"#;
        
        let result = PascalParser::parse(Rule::function_call, input);
        assert!(result.is_ok(), "Failed to parse function call statement");
    }

    #[test]
    fn test_function_call_expression() {
        let input = r#"greet("World")"#;
        
        let result = PascalParser::parse(Rule::function_call_expr, input);
        assert!(result.is_ok(), "Failed to parse function call expression");
    }

    #[test]
    fn test_if_statement() {
        let input = r#"if x > 0 then y := 1; else y := 0;"#;
        
        let result = PascalParser::parse(Rule::if_statement, input);
        assert!(result.is_ok(), "Failed to parse if statement");
    }

    #[test]
    fn test_while_statement() {
        let input = r#"while x > 0 do x := x - 1;"#;
        
        let result = PascalParser::parse(Rule::while_statement, input);
        assert!(result.is_ok(), "Failed to parse while statement");
    }

    #[test]
    fn test_expression_arithmetic() {
        let input = r#"x + y * 2 - 1"#;
        
        let result = PascalParser::parse(Rule::expression, input);
        assert!(result.is_ok(), "Failed to parse arithmetic expression");
    }

    #[test]
    fn test_string_concatenation() {
        let input = r#""Hello, " + name"#;
        
        let result = PascalParser::parse(Rule::expression, input);
        assert!(result.is_ok(), "Failed to parse string concatenation");
    }

    #[test]
    fn test_reference_parameter() {
        let input = r#"function swap(&a: Integer, &b: Integer): Unit begin end"#;
        
        let result = PascalParser::parse(Rule::function_decl, input);
        assert!(result.is_ok(), "Failed to parse function with reference parameters");
    }

    #[test]
    fn test_assignment() {
        let input = r#"x := 42;"#;
        
        let result = PascalParser::parse(Rule::assignment, input);
        assert!(result.is_ok(), "Failed to parse assignment");
    }

    #[test]
    fn test_invalid_program() {
        let input = r#"invalid syntax here"#;
        
        let result = PascalParser::parse(Rule::program, input);
        assert!(result.is_err(), "Should fail to parse invalid syntax");
    }

    #[test]
    fn test_complete_program_parsing() {
        let input = r#"
            program Calculator;
            
            function add(a: Integer, b: Integer): Integer
            begin
                return a + b;
            end
            
            function multiply(x: Integer, y: Integer): Integer
            begin
                return x * y;
            end
            
            begin
                let a: Integer := 5;
                let b: Integer := 3;
                let sum: Integer := add(a, b);
                let product: Integer := multiply(a, b);
                print("Sum: " + sum);
                print("Product: " + product);
            end.
        "#;
        
        let result = PascalParser::parse(Rule::program, input);
        assert!(result.is_ok(), "Failed to parse complete program");
    }
}