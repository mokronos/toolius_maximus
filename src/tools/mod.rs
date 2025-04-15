use serde_json::{json, Value};

pub struct Tool {
    name: String,
    description: String,
    function: fn(String) -> String,
}

impl Tool {
    fn new(name: String, description: String, function: fn(String) -> String) -> Tool {
        Tool {
            name,
            description,
            function,
        }
    }

    fn run(&self, input: String) -> String {
        (self.function)(input)
    }

    fn get_json(&self) -> Value {
        return json!({
            "type": "function",
            "function": {
                "name": self.name,
                "description": self.description,
                "parameters": {
                    "type": "object",
                    "properties": {
                        "input": {
                            "type": "string",
                            "description": "The input to the function"
                        }
                    },
                    "required": ["input"]
                }
            }
        })
    }
}
