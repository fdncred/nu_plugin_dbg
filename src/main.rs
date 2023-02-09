mod table;
use std::os::fd::AsRawFd;

use nu_plugin::{serve_plugin, EvaluatedCall, LabeledError, MsgPackSerializer, Plugin};
use nu_protocol::{Category, PluginExample, PluginSignature, Value};

struct Dbg;

impl Dbg {
    fn new() -> Self {
        Self {}
    }
}

impl Plugin for Dbg {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![PluginSignature::build("dbg")
            .usage("View dbg results")
            .category(Category::Experimental)
            .plugin_examples(vec![PluginExample {
                description: "View dbg results in a pipeline".into(),
                example: "ls | dbg | get name | dbg".into(),
                result: None,
            }])]
    }

    fn run(
        &mut self,
        name: &str,
        _call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        assert_eq!(name, "dbg");

        let original_input = input.clone();

        let value = input.clone();
        let description = match value {
            Value::CustomValue { ref val, .. } => val.value_string(),
            _ => value.get_type().to_string(),
        };

        let termsize = termsize();

        let table = table::build_table(value, description, termsize);

        eprintln!("{table}\n");

        Ok(original_input)
    }
}

fn termsize() -> usize {
    // because STDOUT is redirected we need to use STDERR descriptor
    let (cols, _) = match terminal_size::terminal_size_using_fd(std::io::stderr().as_raw_fd()) {
        Some((w, h)) => (w.0, h.0),
        None => (0, 0),
    };

    cols as usize
}

fn main() {
    serve_plugin(&mut Dbg::new(), MsgPackSerializer);
}
