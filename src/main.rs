mod bluetooth_manager;

use bluetooth_manager::BluetoothManager;
use zbus::Connection;
use tokio::signal;

#[tokio::main]
async fn main() -> zbus::Result<()> {
    let conn: Connection = Connection::system().await?;
    let mut adapter: BluetoothManager<'_> = BluetoothManager::new(&conn).await?;

    adapter.enable_discoverable(String::from("tethyr-device")).await?;
    adapter.register_agent(&conn).await?;
    println!("Bluetooth adapter is now discoverable as {}", adapter.name);
    
    println!("\nTethyr Running... Press Ctrl+C to exit\n");
    println!("Pairing requests will appear below:");

    signal::ctrl_c().await.expect("Shutting down...");
    println!("Shutting down gracefully...");
    
    adapter.shutdown(&conn).await?;
    Ok(())
}