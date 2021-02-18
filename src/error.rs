use smartstring::alias::String;
quick_error! {
    #[derive(Debug)]
    pub enum Error {
        File(e: std::io::Error) {
            from()
            display("Failed to find file")
        }
        Syntax(e: pest_consume::Error<crate::Rule>) {
            from()
            display("Syntax Error")
        }
        NotImplemented(e: String) {
            display("Not Implemented {:?}", e)
        }
        IndexError {
            display("This index doesn't exist")
        }
        VariableNotFound(e: String) {
            display("Variable {} not found", e)
        }
    }
}
