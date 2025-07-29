mod parser;

fn main() {
    let code = r#"
        program HelloWorld;
        
        function greet(name: String): String
        begin
            return "Hello, " + name;
        end
        
        begin
            let message: String := greet("World");
            print(message);
        end.
    "#;

    parser::parse_program(code);
}

