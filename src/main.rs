mod bluetooth_manager;

use bluetooth_manager::BluetoothManager;
use zbus::Connection;

#[tokio::main]
async fn main() -> zbus::Result<()> {
    let conn: Connection = Connection::system().await?;
    let mut adapter: BluetoothManager<'_> = BluetoothManager::new(&conn).await?;

    adapter.enable_discoverable(String::from("tethyr-device")).await?;
    adapter.register_agent(&conn).await?;
    
    println!("Bluetooth adapter is now discoverable as {}", adapter.name);
    println!("Waiting for pairing requests... Press Ctrl+C to exit");

    std::future::pending::<()>().await;
    Ok(())
}