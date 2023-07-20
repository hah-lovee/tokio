async fn hey() {
    println!("hello from async");
}

#[tokio::main]
async fn main() {
    let op = hey();

    println!("hello from main!");

    op.await;
}