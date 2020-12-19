// Receive NATS power messages, and forward to InfluxDB.
use influxdb::InfluxDbWriteable;
use chrono::{DateTime, TimeZone, Utc};

#[derive(InfluxDbWriteable)]
    struct PowerReading {
        time: DateTime<Utc>,
        watts: Option<f64>,
        kwh: Option<f64>,
    }

#[async_std::main]
async fn main() {
    println!("Connecting to NATS");
    let ncres = nats::connect("nats.wellorder.net");
    let nc = match ncres {
        Ok(conn) => conn,
        Err(e) => {
            println!("Could not connect, bailing");
            std::process::exit(1);
        }
    };
    println!("Subscribing to iot.power topic");
    let subres = nc.subscribe("iot.power");
    let sub = match subres {
        Ok(s) => s,
        Err(e) => {
            println!("Could not get subscription, bailing");
            std::process::exit(1);
        }
    };
    // Connect to influxdb
    println!("Connecting to InfluxDB");
    let client = influxdb::Client::new("http://ektar.wellorder.net:8086", "iot");
    for msg in sub.messages() {
        println!("Received Message!");
        println!("This message subject is: {}", msg.subject);
        let utf8res = std::str::from_utf8(&msg.data);
        let msgstr = match utf8res {
            Ok(s) => s,
            Err(e) => { std::process::exit(1) }
        };
        println!("Message is: {}", msgstr);
        // Build a JSON deserializer for the message
        let event : cloudevents::event::Event = serde_json::from_str(msgstr).unwrap();
        println!("{}", event);
        let payload = match event.data().unwrap() {
            cloudevents::Data::Json(v) => v,
            _ => { 
                println!("Did not match JSON payload");
                std::process::exit(1);
            }
        }; 
        println!("{}", payload);
        // extract fields from payload
        let mainobj = match payload {
            serde_json::value::Value::Object(m) => m,
            _ => {
                println!("Expected a top-level object");
                std::process::exit(1);
            }
        };
        // extract temp from mainobj
        // power
        let poweropt = mainobj.get("watts");
        let power = match poweropt {
            Some(v) => v.as_f64(),
                _ => None
        };
        // energy
        let energyopt = mainobj.get("kwh");
        let energy = match energyopt {
            Some(v) => v.as_f64(),
                _ => None
        };
        // parse the date
//        let dt = Utc.timestamp(mainobj.get("dt").unwrap().as_f64().unwrap(), 0); 
        let dtflt = mainobj.get("dt").unwrap().as_f64().unwrap();
        // Get second component
        let dtsec = dtflt as i64;
        // Get nanoseconds
        let dtnano = ((dtflt - (dtsec as f64)) * 1e9) as u32;
        let dt = Utc.timestamp(dtsec, dtnano);
        println!("{}", dt);
        let pr = PowerReading {
            time: dt,
            watts: power,
            kwh: energy,
        }; 
        let write_result = client
            .query(&pr.into_query("power")).await;
        assert!(write_result.is_ok(), "Write result to influxdb was not okay");
        //let vr: Result<serde_json::Value, serde_json::error::Error> = serde_json::from_str(event.data().unwrap());
//        event.deserialize(msgstr)
//        let parsed_event = serde::from_str(msgstr).unwrap();

        // Need to run iter_attributes over the parsed Event
    }

}
