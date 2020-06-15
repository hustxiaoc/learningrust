#[warn(dead_code)]
use async_init::{ async_init };

#[async_init]
async fn test_volt() -> Result<usize, Box<dyn std::error::Error>> {
    let a = async {
        let b: usize = 10;
        b
    };
    let aa: usize = a.await;
    Ok(aa)
}

fn main() {
    println!("ok");
}
