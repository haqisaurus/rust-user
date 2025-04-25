use std::collections::HashMap;
use validator::ValidationErrors;

pub fn serialize_error(errors: &ValidationErrors) -> HashMap<String, String> {
    let mut messages: HashMap<String, String> = HashMap::new();

    for (field, errors) in errors.field_errors().iter() {
        if let Some(first_error) = errors.first() {
            if let Some(msg) = &first_error.message {
                messages.insert(field.to_string(), msg.to_string());
            } else {
                messages.insert(field.to_string(), "Invalid value".to_string());
            }
        }
    }

    messages
}