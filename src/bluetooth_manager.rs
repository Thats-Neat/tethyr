use zbus::{Connection, Proxy};

pub struct BluetoothManager<'a> {
    proxy: Proxy<'a>,
    pub name: String,
    pub powered: bool,
    pub discoverable: bool,
    pub pairable: bool,
}

impl<'a> BluetoothManager<'a> {
    pub async fn new(conn: &'a Connection) -> zbus::Result<Self> {
        let proxy: zbus::Proxy<'_> = zbus::ProxyBuilder::new_bare(conn)
            .destination("org.bluez")?
            .path("/org/bluez/hci0")?
            .interface("org.bluez.Adapter1")?
            .build()
            .await?;

        let name: String = proxy.get_property("Name").await?;
        let powered: bool = proxy.get_property("Powered").await?;
        let discoverable: bool = proxy.get_property("Discoverable").await?;
        let pairable: bool = proxy.get_property("Pairable").await?;

        Ok(BluetoothManager {
            proxy,
            name,
            powered,
            discoverable,
            pairable,
        })
    }

    pub async fn enable_discoverable(&mut self, device_name: String) -> zbus::Result<()> {
        self.set_name(device_name).await?;
        self.set_powered(true).await?;
        self.set_pairable(true).await?;
        self.set_discoverable(true).await?;
        Ok(())
    }

    pub async fn set_powered(&mut self, powered: bool) -> zbus::Result<()> {
        self.proxy.set_property("Powered", &powered).await?;
        self.powered = powered;
        Ok(())
    }

    pub async fn set_discoverable(&mut self, discoverable: bool) -> zbus::Result<()> {
        self.proxy.set_property("Discoverable", &discoverable).await?;
        self.discoverable = discoverable;
        Ok(())
    }

    pub async fn set_pairable(&mut self, pairable: bool) -> zbus::Result<()> {
        self.proxy.set_property("Pairable", &pairable).await?;
        self.pairable = pairable;
        Ok(())
    }

    pub async fn set_name(&mut self, name: String) -> zbus::Result<()> {
        self.proxy.set_property("Alias", &name).await?;
        self.name = name;
        Ok(())
    }
}