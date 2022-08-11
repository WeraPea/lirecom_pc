#![deny(clippy::pedantic)]
use evdev_rs::enums::{BusType, InputProp, EventCode, EventType, EV_KEY, EV_ABS, EV_MSC};
use evdev_rs::AbsInfo;
use evdev_rs::{DeviceWrapper, InputEvent, UInputDevice, UninitDevice};

// fn main() {
fn main() -> Result<(), std::io::Error> {
    let u = UninitDevice::new().unwrap();

    // let size_x = 0.32;
    // let size_y = 0.30;
    // let offset_x = -1500;
    // let offset_y = -1500;
    let size_x = 0.01;
    let size_y = 0.12;
    let offset_x = 0;
    let offset_y = 0;
    let abs_x = AbsInfo{value: 0,
        minimum: (size_x*20966.0) as i32 + offset_x,
        maximum: ((1.0-size_x)*20966.0) as i32 + offset_x, // 20966,
        resolution: 100,
        fuzz: 0,
        flat: 0};

    let abs_y = AbsInfo{value: 0,
        minimum: (size_y*15725.0) as i32 + offset_y,
        maximum: ((1.0-size_y)*15725.0) as i32 + offset_y, // 15725,
        resolution: 100,
        fuzz: 0,
        flat: 0};

    let abs_z = AbsInfo{value: 0,
        minimum: -255,
        maximum: 0,
        resolution: 10,
        fuzz: 0,
        flat: 0};

    let abs_pressure = AbsInfo{value: 0,
        minimum: 0,
        maximum: 4095,
        resolution: 0,
        fuzz: 0,
        flat: 0};

    let abs_tilt_x = AbsInfo{value: -1800,
        minimum: -9000,
        maximum: 9000,
        resolution: 5730,
        fuzz: 0,
        flat: 0};

    let abs_tilt_y = AbsInfo{value: -1100,
        minimum: -9000,
        maximum: 9000,
        resolution: 5730,
        fuzz: 0,
        flat: 0};

    let abs_misc = AbsInfo{value: 0,
        minimum: 0,
        maximum: 3,
        resolution: 0,
        fuzz: 0,
        flat: 0};

    // let abs_y = AbsInfo{value: 0, minimum: 0, maximum: 15725, resolution: 100};
    // let abs_z = AbsInfo{value: -255, minimum: -255, maximum: 0, resolution: 10};

    u.set_name("lirecom stylus");
    u.set_bustype(BusType::BUS_I2C as u16);
    u.set_vendor_id(0xabcd);
    u.set_product_id(0xabcd);
    
    // u.enable_event_type(&EventType::EV_SYN);
    // u.enable_event_code(&EventCode::);
    u.enable_event_type(&EventType::EV_KEY)?;
    u.enable_event_code(&EventCode::EV_KEY(EV_KEY::BTN_TOOL_PEN), None)?;
    u.enable_event_code(&EventCode::EV_KEY(EV_KEY::BTN_TOOL_RUBBER), None)?;
    u.enable_event_code(&EventCode::EV_KEY(EV_KEY::BTN_TOUCH), None)?;
    u.enable_event_code(&EventCode::EV_KEY(EV_KEY::BTN_STYLUS), None)?;
    u.enable_event_code(&EventCode::EV_KEY(EV_KEY::BTN_STYLUS2), None)?;

    u.enable_event_type(&EventType::EV_ABS)?;
    u.enable_event_code(&EventCode::EV_ABS(EV_ABS::ABS_X), Some(&abs_x))?;
    u.enable_event_code(&EventCode::EV_ABS(EV_ABS::ABS_Y), Some(&abs_y))?;
    u.enable_event_code(&EventCode::EV_ABS(EV_ABS::ABS_Z), Some(&abs_z))?;
    u.enable_event_code(&EventCode::EV_ABS(EV_ABS::ABS_PRESSURE), Some(&abs_pressure))?;
    u.enable_event_code(&EventCode::EV_ABS(EV_ABS::ABS_TILT_X), Some(&abs_tilt_x))?;
    u.enable_event_code(&EventCode::EV_ABS(EV_ABS::ABS_TILT_Y), Some(&abs_tilt_y))?;
    u.enable_event_code(&EventCode::EV_ABS(EV_ABS::ABS_MISC), Some(&abs_misc))?;

    u.enable_event_type(&EventType::EV_MSC)?;
    u.enable_event_code(&EventCode::EV_MSC(EV_MSC::MSC_SCAN), None)?;

    u.enable(&InputProp::INPUT_PROP_DIRECT)?;

    let v = UInputDevice::create_from_device(&u)?;

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed :(");
        let input = input.trim().split(' ').collect::<Vec<&str>>();
        if input.len() != 5 {
            continue;
        }
        let e = match input[..] {
            [value, "ABS_X", ..] => Some((EventCode::EV_ABS(EV_ABS::ABS_X), value)),
            [value, "ABS_Y", ..] => Some((EventCode::EV_ABS(EV_ABS::ABS_Y), value)),
            // [value, "ABS_Z", ..] => Some((EventCode::EV_ABS(EV_ABS::ABS_Z), value)),
            [value, "ABS_PRESSURE", ..] =>
                Some((EventCode::EV_ABS(EV_ABS::ABS_PRESSURE), value)),
            [value, "ABS_TILT_X", ..] =>
                Some((EventCode::EV_ABS(EV_ABS::ABS_TILT_X), value)),
            [value, "ABS_TILT_Y", ..] =>
                Some((EventCode::EV_ABS(EV_ABS::ABS_TILT_Y), value)),
            _ => None
        };
        if let Some((e, value)) = e {
            let time_f = match input[3].parse::<i64>() {
                Ok(v) => v,
                Err(_) => continue
            };
            let time_l = match input[4].parse::<i64>() {
                Ok(v) => v,
                Err(_) => continue
            };
            match value.parse::<i64>() {
                Ok(_) => (),
                Err(_) => continue
            };
            v.write_event(&InputEvent {
                time: evdev_rs::TimeVal::new(time_f, time_l),
                event_code: e,
                value: value.parse().unwrap(),
            })?;
        };
    }
}
