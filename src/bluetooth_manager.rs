use zbus::{Connection, Proxy, fdo};
use zbus::zvariant::ObjectPath;

pub struct BluetoothManager<'a> {
    proxy: Proxy<'a>,
    pub name: String,
    pub powered: bool,
    pub discoverable: bool,
    pub pairable: bool,
    agent_manager_proxy: Option<Proxy<'a>>,
}

pub struct BluetoothAgent;

#[zbus::dbus_interface(name = "org.bluez.Agent1")]
impl BluetoothAgent {
    async fn release(&self) {}

    async fn display_pin_code(&self, device: ObjectPath<'_>, pincode: &str) {
        let mac = Self::extract_mac_from_path(&device);
        println!("PIN: {} for device, {}", pincode, mac);
    }

    async fn request_confirmation(&self, device: ObjectPath<'_>, passkey: u32) -> fdo::Result<()> {
        let mac = Self::extract_mac_from_path(&device);
        println!("Pairing code: {} for device, {}", passkey, mac);
        Ok(())
    }

    async fn request_authorization(&self, device: ObjectPath<'_>) -> fdo::Result<()> {
        let mac = Self::extract_mac_from_path(&device);
        println!("Authorizing device: {}", mac);
        Ok(())
    }

    async fn cancel(&self) {}
}

impl BluetoothAgent {
    fn extract_mac_from_path(device_path: &ObjectPath) -> String {
        let path_str = device_path.as_str();
        if let Some(dev_part) = path_str.split("/dev_").nth(1) {
            dev_part.replace('_', ":")
        } else {
            path_str.to_string()
        }
    }
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

        let agent_manager_proxy = zbus::ProxyBuilder::new_bare(conn)
            .destination("org.bluez")?
            .path("/org/bluez")?
            .interface("org.bluez.AgentManager1")?
            .build()
            .await
            .ok();

        Ok(BluetoothManager {
            proxy,
            name,
            powered,
            discoverable,
            pairable,
            agent_manager_proxy,
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

    pub async fn register_agent(&self, conn: &Connection) -> zbus::Result<()> {
        if let Some(agent_proxy) = &self.agent_manager_proxy {
            let agent_path = "/org/bluez/agent";
            conn.object_server().at(agent_path, BluetoothAgent).await?;
            
            agent_proxy.call_method("RegisterAgent", &(ObjectPath::try_from(agent_path)?, "DisplayYesNo")).await?;
            agent_proxy.call_method("RequestDefaultAgent", &(ObjectPath::try_from(agent_path)?,)).await?;
            println!("Agent registered successfully");
        }
        Ok(())
    }
}