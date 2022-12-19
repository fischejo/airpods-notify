mod proximity;

use crate::proximity::{ProximityEvent, PairedMessage, Color, Model, Lid, Battery};
use std::env;
use btleplug::api::{Central, CentralEvent, Manager as _, ScanFilter, Peripheral};
use btleplug::platform::{Adapter, Manager};
use std::error::Error;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use futures::{Stream, StreamExt};


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    Monitor,
    Nearby,
}

const MIN_RSSI: i16 = -60;

fn image_from(msg: &PairedMessage) -> Option<&str> {
    match (&msg.model, &msg.color) {
        (Model::AirPodsPro, _) => Some("airpodspro.png"),
        (Model::AirPodsPro2, _) => Some("airpodspro.png"),
        (Model::AirPods1, _) => Some("airpods.png"),
        (Model::AirPods2, _) => Some("airpods.png"),
        (Model::AirPods3, _) => Some("airpodspro.png"),
        (Model::BeatsFlex, _) => Some("beatsflex-black.png"), // black, yellow, blue, gray
        (Model::BeatsSolo3, _) => Some("beatssolo3-white.png"), 
        (Model::BeatsStudio3, _) => Some("beatsstudio3-black.png"), 
        (Model::BeatsX, _) => Some("beatsx-black.png"),
        (Model::AirPodsMax, _) => Some("airpodsmax-white.png"), // space-gray, pink green, sky-blue
        (Model::PowerbeatsPro, _) => Some("powerbeatspro-black.png"), // ivory, navy, black
        (Model::PowerBeats3, _) => Some("powerbeats3-black.png"),
        (Model::Unknown(_), _) => None,
        _ => None
    }
}

fn summary_from(msg: &PairedMessage) -> String {
    format!("{:?}", msg.model)
}

fn body_from(msg: &PairedMessage) -> String {
    let level_symbol = |battery| {
        match battery {
            Battery::Level(0) | Battery::Level(1) => "",
            Battery::Level(2) | Battery::Level(3) => "",
            Battery::Level(4) | Battery::Level(5) => "",
            Battery::Level(6) | Battery::Level(7) => "",
            Battery::Level(8) | Battery::Level(9) | Battery::Level(10) => "",
            _ => "",
        }
    };

    let charge_symbol = |is_charging: bool| if is_charging {
        ""
    } else {
        ""
    };

    let level_text = |battery| {
        match battery {
            Battery::Level(v) => format!("{}% ", v*10),
            Battery::None => "".to_string()
        }
    };

    let mut body = String::from("");
    if msg.left_battery_level != Battery::None {
        body.push_str(format!("Left: {}{} {}\n", 
            level_text(msg.left_battery_level), 
            level_symbol(msg.left_battery_level), 
            charge_symbol(msg.left_charging)).as_str());
    };
    if msg.right_battery_level != Battery::None {
        body.push_str(format!("Right: {}{} {}\n", 
            level_text(msg.right_battery_level), 
            level_symbol(msg.right_battery_level), 
            charge_symbol(msg.right_charging)).as_str());
    };
    if msg.case_battery_level != Battery::None {
        body.push_str(format!("Case: {}{} {}\n", 
            level_text(msg.case_battery_level), 
            level_symbol(msg.case_battery_level), 
            charge_symbol(msg.case_charging)).as_str());
    };
    body
}


async fn nearby_mode(adapter: Adapter) -> Result<(), Box<dyn Error>> {
    let mut events = filter_events(adapter, Mode::Nearby).await?;
    let mut current_handle: Option<notify_rust::NotificationHandle> = None;

    while let Some(event) = events.next().await {
        match event {
            ProximityEvent::Paired(msg) => {
                match msg.lid {
                    Lid::Open(_) => {
                        if let Some(mut handle) = current_handle {
                            // notification update
                            handle.body(body_from(&msg).as_str());
                            handle.summary(summary_from(&msg).as_str());
                            if let Some(path) = image_from(&msg) {            
                                handle.image(format!("./res/{}", path));
                            }
                            handle.update();
                            current_handle = Some(handle);
                        } else {
                            // new notification
                            let mut notification = notify_rust::Notification::new();
                            
                            if let Some(path) = image_from(&msg) {            
                                notification.image(format!("./res/{}", path));
                            }                    
                            notification.summary(summary_from(&msg).as_str());
                            notification.body(body_from(&msg).as_str());
                            notification.timeout(notify_rust::Timeout::Default);
                            current_handle = Some(notification.show().unwrap());            
                        }
                    }
                    Lid::Closed(_) => {
                        if let Some(handle) = current_handle {
                            handle.close();                            
                        }
                        current_handle = None;
                    }
                }
            },
            ProximityEvent::Pairing(msg) => {
                    // new notification
                    let mut notification = notify_rust::Notification::new();
                    notification.action("connect", "Connect");
                    notification.action("default", "default");
                    notification.hint(notify_rust::Hint::Resident(true));
                    notification.summary("");
                    notification.body("New device! Want to connect?");
                    notification.timeout(notify_rust::Timeout::Default);
                    notification.show()
                    .unwrap()
                    .wait_for_action(|action| match action {
                        "default" => println!("default"),
                        "connect" => println!("connect"),
                        _ => (),
                    });
            }
            _ => ()
        }
    }    
    Ok(())
}

async fn monitor_mode(adapter: Adapter) -> Result<(), Box<dyn Error>> {
    print!("{0:^16} | ", "Model");
    print!("{0:^10} | ", "Color");
    print!("{0:^10} | ", "Lid");
    print!("{0:^10} | ", "Case Bat.");
    print!("{0:^10} | ", "Left Bat.");
    print!("{0:^10} | ", "Right Bat.");
    print!("{0:^13} | ", "Case Charging");
    print!("{0:^13} | ", "Left Charging");
    print!("{0:^14} | ", "Right Charging");
    print!("{0:^8} | ", "in Ear");
    print!("{0:^8} | ", "in Case");
    println!("{0:^14} | ", "Part");        

    let mut events = filter_events(adapter, Mode::Monitor).await?;
    while let Some(event) = events.next().await {
        match event {
            ProximityEvent::Paired(msg) => {
                print!("{0:^16} | ", msg.model.to_string());
                print!("{0:^10} | ", msg.color.to_string());
                print!("{0:^10} | ", msg.lid.to_string());
                print!("{0:^10} | ", msg.case_battery_level.to_string());
                print!("{0:^10} | ", msg.left_battery_level.to_string());
                print!("{0:^10} | ", msg.right_battery_level.to_string());
                print!("{0:^13} | ", msg.case_charging);
                print!("{0:^13} | ", msg.left_charging);
                print!("{0:^14} | ", msg.right_charging);
                print!("{0:^8} | ", msg.plugged_in_ear.to_string());
                print!("{0:^8} | ", msg.plugged_in_case.to_string());
                println!("{0:^14} | ", msg.part.to_string());
            }
            _ => ()
            
        }
    }
    Ok(())
}

async fn get_central(manager: &Manager) -> Adapter {
    let adapters = manager.adapters().await.unwrap();
    adapters.into_iter().nth(0).unwrap()
}

pub async fn filter_events(adapter: Adapter, mode: Mode) -> Result<impl Stream<Item = ProximityEvent>, Box<dyn Error>> {
    let (tx, rx) = mpsc::channel(1);
    //let mut known_devices: HashMap<bluer::Address, PairedMessage> = HashMap::new();
    let mut events = adapter.events().await?;

    tokio::spawn(async move {
        while let Some(event) = events.next().await {
            /* device found or propery changed events */            
            if let CentralEvent::ManufacturerDataAdvertisement{
                id, 
                manufacturer_data} = event {

                /* get device address and RSSI */
                let peripheral = adapter.peripheral(&id).await
                    .unwrap();
                    
                let property = peripheral.properties().await
                    .unwrap()
                    .unwrap().rssi;

                if let Some(rssi) = property {
                    if mode == Mode::Monitor || rssi >= MIN_RSSI {
                        /* unpack manufacturer data and parse it */
                        if let Some(msg) = ProximityEvent::from_manufacturer_data(manufacturer_data) {
                            let _ = tx.send(msg).await;
                        }
                    }
                }
            }
        }
    });
    Ok(ReceiverStream::new(rx))
}


#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>>  {

    let manager = Manager::new().await?;
    let central = get_central(&manager).await;
    let with_monitor = env::args().any(|arg| arg == "--monitor");    

    // start scanning for devices
    central.start_scan(ScanFilter::default()).await?;
    if with_monitor {
        monitor_mode(central).await?;
    } else {
        nearby_mode(central).await?;
    }
    Ok(())
}