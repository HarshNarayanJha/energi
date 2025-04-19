use upower_dbus::{DeviceProxy, UPowerProxy};

use super::BatteryData;

pub async fn get_battery_data() -> Result<BatteryData, zbus::Error> {
    let connection = zbus::Connection::system().await?;
    let upower = UPowerProxy::new(&connection).await?;
    let device_paths = upower.enumerate_devices().await?;

    // let's try to get the first device
    for path_owned in device_paths {
        let path = path_owned.to_string();
        let device = DeviceProxy::new(&connection, path).await?;

        // tracing::debug!("Device: {:#?}", device.path());
        let percentage = device.percentage().await?;
        let state = device.state().await?;
        let temperature = device.temperature().await?;

        let rate: f64 = device.get_property("EnergyRate").await?;
        // let charge_cycles: i64 = device.get_property("ChargeCycles").await?;
        let health: f64 = device.get_property("Capacity").await?;
        let time_to_empty: i64 = device.get_property("TimeToEmpty").await?;
        let time_to_full: i64 = device.get_property("TimeToFull").await?;
        let voltage: f64 = device.get_property("Voltage").await?;
        let vendor: String = device.get_property("Vendor").await?;
        let serial: String = device.get_property("Serial").await?;

        let type_ = device.type_().await?;
        let model = device.model().await?;

        // tracing::debug!("Device path: {:#?}", percentage);
        // tracing::debug!("Device state: {:#?}", state);
        // tracing::debug!("Device temperature: {:#?}", temperature);
        // tracing::debug!("Device rate: {:#?}", rate);
        // // tracing::debug!("Device charge cycles: {:#?}", charge_cycles);
        // tracing::debug!("Device health: {:#?}", health);
        // tracing::debug!("Device time to empty: {:#?}", time_to_empty);
        // tracing::debug!("Device time to full: {:#?}", time_to_full);
        // tracing::debug!("Device voltage: {:#?}", voltage);
        // tracing::debug!("Device type: {:#?}", type_);
        // tracing::debug!("Device model: {:#?}", model);
        // tracing::debug!("Device vendor: {:#?}", vendor);
        // tracing::debug!("Device serial: {:#?}", serial);

        let _history_reply: (Vec<(u32, f64, u32)>,) = connection
            .call_method(
                Some("org.freedesktop.UPower"),
                &path_owned,
                Some("org.freedesktop.UPower.Device"),
                "GetHistory",
                &("charge", 7200u32, 1000u32),
            )
            .await?
            .body()?;

        let _rate_reply: (Vec<(u32, f64, u32)>,) = connection
            .call_method(
                Some("org.freedesktop.UPower"),
                &path_owned,
                Some("org.freedesktop.UPower.Device"),
                "GetHistory",
                &("rate", 1800u32, 500u32),
            )
            .await?
            .body()?;

        let history_percentage: Vec<(u32, f64)> = _history_reply
            .0
            .into_iter()
            .map(|(time, value, _state)| (time, value))
            .rev()
            .skip(4)
            .collect();

        let history_rate: Vec<(u32, f64)> = _rate_reply
            .0
            .into_iter()
            .map(|(time, value, _state)| (time, value))
            .rev()
            .skip(4)
            .collect();

        // tracing::debug!("Device charge history: {:#?}", history_percentage);
        // tracing::debug!("Device rate history: {:#?}", history_rate);

        return Ok(BatteryData {
            percentage,
            state,
            health,
            temperature,
            rate,
            time_to_empty,
            time_to_full,
            serial,
            vendor,
            model,
            voltage,
            charge_cycles: 0,
            history_percentage,
            history_rate,
        });
    }

    Err(zbus::Error::Failure("No battery devices found".into()))
}
