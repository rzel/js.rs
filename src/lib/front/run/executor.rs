use front::stdlib::value::{
    Value, 
    ResultValue
};
use std::default::Default;

/// An execution engine which runs whatever is generated by the `Compiler`
pub trait Executor<Compiled> {
    /// Create a new execution engine with the given configuration
    fn new(config:&ExecutorConfig) -> Self;
    /// Get the global object associated with this executor
    fn get_global_obj(&self) -> Value;
    #[inline]
    /// Get a field of the global object
    fn get_global<'a>(&self, field: &'a str) -> Value {
        self.get_global_obj().get_field(field)
    }
    #[inline]
    /// Set a field of the global object
    fn set_global<'a>(&self, field: &'a str, value:Value) -> Value {
        self.get_global_obj().set_field(field, value)
    }
    /// Execute a compiled expression
    fn execute(&self, comp:&Compiled) -> ResultValue;
}
/// Configuration for the executor
pub struct ExecutorConfig {
    /// The initial global value
    pub global: Value
}
impl Default for ExecutorConfig {
    #[inline(always)]
    fn default() -> ExecutorConfig {
        ExecutorConfig {
            global: Value::new_global()
        }
    }
}