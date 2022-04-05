use lambda_runtime::{service_fn, LambdaEvent, Error};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(service_fn(func)).await?;
    Ok(())
}

async fn func(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (event, _context) = event.into_parts();
    let first_name = event["firstName"].as_str().unwrap_or("world");

    Ok(json!({ "message": format!("Hello, {}!", first_name) }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_runtime::Context;

    #[tokio::test]
    async fn handler_default_parameter() {
        let event = LambdaEvent::new(json!({}), Context::default());
        assert_eq!(
            func(event.clone())
                .await
                .expect("expected Ok(_) value"),
            json!({"message": "Hello, world!"})
        )
    }

    #[tokio::test]
    async fn handler_custom_parameter() {
        let event = LambdaEvent::new(json!({"firstName": "Jake"}), Context::default());
        assert_eq!(
            func(event.clone())
                .await
                .expect("expected Ok(_) value"),
            json!({"message": "Hello, Jake!"})
        )
    }
}
