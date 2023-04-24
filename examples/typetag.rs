// cargo run --example web_event

use serde::{Deserialize, Serialize};

#[typetag::serde]
trait WebEvent {
    fn inspect(&self);
}

#[derive(Serialize, Deserialize)]
struct PageLoad;

#[typetag::serde]
impl WebEvent for PageLoad {
    fn inspect(&self) {
        println!("200 milliseconds or bust");
    }
}

#[derive(Serialize, Deserialize)]
struct Click {
    x: i32,
    y: i32,
}

#[typetag::serde]
impl WebEvent for Click {
    fn inspect(&self) {
        println!("negative space between the ads: x={} y={}", self.x, self.y);
    }
}

fn main() -> serde_json::Result<()> {
    let page_load = PageLoad;
    let event = &page_load as &dyn WebEvent;
    let json1 = serde_json::to_string(event)?;
    println!("PageLoad json: {}", json1);
    let de1: Box<dyn WebEvent> = serde_json::from_str(&json1)?;
    de1.inspect();

    println!();

    let click = Click { x: 10, y: 10 };
    let event = &click as &dyn WebEvent;
    let json2 = serde_json::to_string(event)?;
    println!("Click json: {}", json2);
    let de2: Box<dyn WebEvent> = serde_json::from_str(&json2)?;
    de2.inspect();
    de2.inspect();

    let v = [de1, de2];

    v.iter().for_each(|d| d.inspect());

    Ok(())
}
