#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    println!("PhysicsTree server — will be wired in Plan 03");
}

#[cfg(not(feature = "ssr"))]
fn main() {
    // Client-side entry point — not used directly
}
