mod table;
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

#[cfg(not(windows))]
fn termsize() -> usize {
    use std::os::fd::AsRawFd;

    // because STDOUT is redirected we need to use STDERR descriptor
    let (cols, _) = match terminal_size::terminal_size_using_fd(std::io::stderr().as_raw_fd()) {
        Some((w, h)) => (w.0, h.0),
        None => (0, 0),
    };

    cols as usize
}

#[cfg(windows)]
fn termsize() -> usize {
    use std::os::windows::io::RawHandle;
    use windows_sys::Win32::System::Console::{GetStdHandle, STD_ERROR_HANDLE};

    let stderr = unsafe { GetStdHandle(STD_ERROR_HANDLE) } as RawHandle;

    // because STDOUT is redirected we need to use STDERR descriptor
    let (cols, _) = match terminal_size::terminal_size_using_handle(stderr) {
        Some((w, h)) => (w.0, h.0),
        None => (0, 0),
    };

    cols as usize
}

fn main() {
    serve_plugin(&mut Dbg::new(), MsgPackSerializer);
}

pub fn debug_string_without_formatting(value: &Value) -> String {
    match value {
        Value::Bool { val, .. } => val.to_string(),
        Value::Int { val, .. } => val.to_string(),
        Value::Float { val, .. } => val.to_string(),
        Value::Filesize { val, .. } => val.to_string(),
        Value::Duration { val, .. } => val.to_string(),
        Value::Date { val, .. } => format!("{val:?}"),
        Value::Range { val, .. } => {
            format!(
                "{}..{}",
                debug_string_without_formatting(&val.from),
                debug_string_without_formatting(&val.to)
            )
        }
        Value::String { val, .. } => val.clone(),
        Value::List { vals: val, .. } => format!(
            "[{}]",
            val.iter()
                .map(debug_string_without_formatting)
                .collect::<Vec<_>>()
                .join(" ")
        ),
        Value::Record { cols, vals, .. } => format!(
            "{{{}}}",
            cols.iter()
                .zip(vals.iter())
                .map(|(x, y)| format!("{}: {}", x, debug_string_without_formatting(y)))
                .collect::<Vec<_>>()
                .join(" ")
        ),
        Value::LazyRecord { val, .. } => match val.collect() {
            Ok(val) => debug_string_without_formatting(&val),
            Err(error) => format!("{error:?}"),
        },
        //TODO: It would be good to drill in deeper to blocks and closures.
        Value::Block { val, .. } => format!("<Block {val}>"),
        Value::Closure { val, .. } => format!("<Closure {val}>"),
        Value::Nothing { .. } => String::new(),
        Value::Error { error } => format!("{error:?}"),
        Value::Binary { val, .. } => format!("{val:?}"),
        Value::CellPath { val, .. } => val.into_string(),
        Value::CustomValue { val, .. } => val.value_string(),
    }
}
