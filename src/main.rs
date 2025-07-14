use zbus::Connection;

#[tokio::main]
async fn main() -> zbus::Result<()> {
    let conn = Connection::system().await?;

    let proxy: zbus::Proxy<'_> = zbus::ProxyBuilder::new_bare(&conn)
        .destination("org.bluez")?
        .path("/org/bluez/hci0")?
        .interface("org.bluez.Adapter1")?
        .build()
        .await?;

    proxy.set_property("Powered", &true).await?;
    proxy.set_property("Discoverable", &true).await?;
    proxy.set_property("Pairable", &true).await?;

    let name: String = proxy.get_property("Name").await?;
    println!("Bluetooth adapter is now discoverable as {name}");
    Ok(())
}