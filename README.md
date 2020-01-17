# mmrs 

MatterMost Rust: a very simple MatterMost interface library written in Rust 

Example: 

```rust
use mmrs; 

fn main() {
    let your_handle = "Degausser".to_string();
    let your_name = "Ricky".to_string();
    let mut message: mmrs::MMBody = mmrs::MMBody::new();

    message.username = Some("mmrsBOT".to_string());
    message.channel = Some("Town Square".to_string());

    message.text = Some(format!(
        "{} is known as {}",
        &user,
        &your_name
    ));

    let body = message.to_json().expect("Error converting to json");

    let response = mmrs::send_message("https://localhost:9009/post", body);
    
    match response {
        Ok(code) => {
            if code == 200 {
                println!("{}: Sucess!", code);
            } else {
                println!("Error bad response code: {}\n\n", code);
            }
        }
        Err(e) => {
            println!("Error: {}\n\n", e);
        }
    }
} 
```


