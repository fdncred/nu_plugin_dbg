mod dbg;
use nu_plugin::{serve_plugin, EvaluatedCall, LabeledError, MsgPackSerializer, Plugin};
use nu_protocol::engine::EngineState;
// use nu_protocol::engine::Stack;
use nu_protocol::{Category, Signature, Value};

struct Dbg;

impl Dbg {
    fn new() -> Self {
        Self {}
    }
}

impl Plugin for Dbg {
    fn signature(&self) -> Vec<Signature> {
        vec![Signature::build("dbg")
            .usage("View dbg results")
            .category(Category::Experimental)]
    }

    fn run(
        &mut self,
        name: &str,
        _call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        assert_eq!(name, "dbg");
        //////////////////////////
        // let head = call.head;
        let original_input = input.clone();
        let value = input.clone();
        let description = match value {
            Value::CustomValue { ref val, .. } => val.value_string(),
            _ => value.get_type().to_string(),
        };

        // let no_newline = false;
        // let to_stderr = false;
        let engine_state = EngineState::new();
        let config = engine_state.config;
        // let mut stack = Stack::new();

        // eprintln!("{}", value);
        // value
        //     .into_pipeline_data()
        //     .print(&engine_state, &mut stack, no_newline, to_stderr)?;

        eprintln!("input description: {}\n", description);

        // Ok(Value::String {
        //     val: description,
        //     span: head,
        // })
        eprintln!("input value: {}\n", input.clone().debug_string("", &config));

        Ok(original_input)
        // Ok(Value::Nothing { span: head })
        //////////////////////////
        // let param: Option<Spanned<String>> = call.opt(0)?;

        // let ret_val = match input {
        //     Value::String { val, span } => {
        //         crate::dbg::dbg_do_something(param, val, *span)?
        //     }
        //     v => {
        //         return Err(LabeledError {
        //             label: "Expected something from pipeline".into(),
        //             msg: format!("requires some input, got {}", v.get_type()),
        //             span: Some(call.head),
        //         });
        //     }
        // };

        // Ok(ret_val)
    }
}

fn main() {
    serve_plugin(&mut Dbg::new(), MsgPackSerializer);
}
