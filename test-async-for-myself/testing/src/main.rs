// async fn hey() {
//     println!("hello from async");
// }

// #[tokio::main]
// async fn main() {
//     let op = hey();

//     println!("hello from main!");

//     op.await;
// }

use tokio::task::yield_now;
use std::rc::Rc;

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        // The scope forces `rc` to drop before `.await`.
        {
            let rc = Rc::new("hello");
            println!("{}", rc);
        }

        // `rc` is no longer used. It is **not** persisted when
        // the task yields to the scheduler
        yield_now().await;
    });
}