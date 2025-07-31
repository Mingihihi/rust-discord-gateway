~~create multiple owners to self.writer ([reference](https://stackoverflow.com/questions/77277773/multiple-owners-to-a-tokio-tungstenite-wss-stream))~~ DONE

register discord identity (send opcode 2 to discord gateway)
example:
```rust
let identifyPayload = r#"{
    op: 2,
    d: {
        token,
        // intents: 1 << 9 | (1 << 12) | (1 << 15),
        properties: {
            os: "windows",
            browser: "chrome",
            device: "pc"
        },
    }
}"#;

writer.send(Message::Text(identifyPayload.to_string().into())).await;
```

event handle for websocket
idea:
```rust
#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            println!("do what ever you want here!")
        }
    }
}
```