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
        FunctionNotFound(e: String) {
            display("Function {} not found", e)
        }
        ReturnOutOfFunction {
            display("Return was used outside of a function")
        }
        BreakOutOfLoop {
            display("Break was used outside of a loop")
        }
        ContinueOutOfLoop {
            display("Continue was used outside of a loop")
        }
        InvalidIndex {
            display("Invalid index type")
        }
        EvalNotBool {
            display("This expression doesn't evaluate to a boolean")
        }
        NoReturnValue {
            display("This function doesn't return anything")
        }
        ImpossibleOperation {
            display("This is an impossible operation")
        }
    }
}
