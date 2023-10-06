use evdev_rs::enums::{BusType, EventCode, EventType, InputProp, EV_ABS, EV_KEY, EV_MSC, EV_SYN};
use evdev_rs::AbsInfo;
use evdev_rs::{DeviceWrapper, InputEvent, UInputDevice, UninitDevice};
use std::any::Any;

macro_rules! skip_none {
    ($res:expr) => {
        match $res {
            Some(val) => val,
            None => continue,
        }
    };
}

fn main() -> Result<(), std::io::Error> {
    // let x11_screen_x = 2560.;
    let x11_screen_y = 1440.;
    let x11_screen_x = 3840.;

    let d_tablet = UninitDevice::new().unwrap();

    let real_screen_x = 20966.;
    let real_screen_y = 15725.;

    // let zoom = 0.5;
    let zoom = 1.2;

    let (size_x, size_y) = if true {
        let size_x = real_screen_x * zoom;
        let size_y = x11_screen_y * size_x / x11_screen_x * zoom;
        (size_x, size_y)
    } else {
        let size_y = real_screen_y * zoom;
        let size_x = x11_screen_x * size_y / x11_screen_y * zoom;
        (size_x, size_y)
    };

    let offset_x = 0;
    let offset_y = 0;

    let abs_x = AbsInfo {
        value: 0,
        minimum: offset_x,
        maximum: size_x as i32 - offset_x,
        resolution: 100,
        fuzz: 0,
        flat: 0,
    };

    let abs_y = AbsInfo {
        value: 0,
        minimum: offset_y,
        maximum: size_y as i32 - offset_y,
        resolution: 100,
        fuzz: 0,
        flat: 0,
    };

    let abs_z = AbsInfo {
        value: 0,
        minimum: -255,
        maximum: 0,
        resolution: 10,
        fuzz: 0,
        flat: 0,
    };

    let abs_pressure = AbsInfo {
        value: 0,
        minimum: 0,
        maximum: 4095,
        resolution: 0,
        fuzz: 0,
        flat: 0,
    };

    let abs_tilt_x = AbsInfo {
        value: 0,
        minimum: -9000,
        maximum: 9000,
        resolution: 5730,
        fuzz: 0,
        flat: 0,
    };

    let abs_tilt_y = AbsInfo {
        value: 0,
        minimum: -9000,
        maximum: 9000,
        resolution: 5730,
        fuzz: 0,
        flat: 0,
    };

    let abs_misc = AbsInfo {
        value: 0,
        minimum: 0,
        maximum: 3,
        resolution: 0,
        fuzz: 0,
        flat: 0,
    };

    d_tablet.set_name("lirecom stylus tablet");
    d_tablet.set_bustype(BusType::BUS_I2C as u16);
    d_tablet.set_vendor_id(0xabcd);
    d_tablet.set_product_id(0xabcd);
    d_tablet.enable(&InputProp::INPUT_PROP_DIRECT)?;

    let event_types = [
        &EventType::EV_SYN,
        &EventType::EV_KEY,
        &EventType::EV_ABS,
        &EventType::EV_MSC,
    ];

    let event_codes: [(&EventCode, Option<&(dyn Any + 'static)>); 14] = [
        (&EventCode::EV_SYN(EV_SYN::SYN_REPORT), None),
        (&EventCode::EV_KEY(EV_KEY::BTN_TOOL_PEN), None),
        (&EventCode::EV_KEY(EV_KEY::BTN_TOOL_RUBBER), None),
        (&EventCode::EV_KEY(EV_KEY::BTN_TOUCH), None),
        (&EventCode::EV_KEY(EV_KEY::BTN_STYLUS), None),
        (&EventCode::EV_KEY(EV_KEY::BTN_STYLUS2), None),
        (&EventCode::EV_ABS(EV_ABS::ABS_X), Some(&abs_x)),
        (&EventCode::EV_ABS(EV_ABS::ABS_Y), Some(&abs_y)),
        (&EventCode::EV_ABS(EV_ABS::ABS_Z), Some(&abs_z)),
        (
            &EventCode::EV_ABS(EV_ABS::ABS_PRESSURE),
            Some(&abs_pressure),
        ),
        (&EventCode::EV_ABS(EV_ABS::ABS_TILT_X), Some(&abs_tilt_x)),
        (&EventCode::EV_ABS(EV_ABS::ABS_TILT_Y), Some(&abs_tilt_y)),
        (&EventCode::EV_ABS(EV_ABS::ABS_MISC), Some(&abs_misc)),
        (&EventCode::EV_MSC(EV_MSC::MSC_SCAN), None),
    ];

    for event_type in event_types {
        d_tablet.enable_event_type(event_type)?;
    }
    for event_code in event_codes {
        d_tablet.enable_event_code(event_code.0, event_code.1)?;
    }

    let u_tablet = UInputDevice::create_from_device(&d_tablet)?;

    let d_pen = UninitDevice::new().unwrap();

    d_pen.set_name("lirecom stylus pen");
    d_pen.set_bustype(BusType::BUS_I2C as u16);
    d_pen.set_vendor_id(0xabcd);
    d_pen.set_product_id(0xabce);
    d_pen.enable(&InputProp::INPUT_PROP_DIRECT)?;

    let event_types = [&EventType::EV_SYN, &EventType::EV_KEY, &EventType::EV_MSC];

    let event_codes: [(&EventCode, Option<&(dyn Any + 'static)>); 10] = [
        (&EventCode::EV_SYN(EV_SYN::SYN_REPORT), None),
        (&EventCode::EV_KEY(EV_KEY::KEY_SLEEP), None),
        (&EventCode::EV_KEY(EV_KEY::BTN_BACK), None),
        (&EventCode::EV_KEY(EV_KEY::BTN_FORWARD), None),
        (&EventCode::EV_KEY(EV_KEY::BTN_TOOL_PEN), None),
        (&EventCode::EV_KEY(EV_KEY::BTN_TOOL_RUBBER), None),
        (&EventCode::EV_KEY(EV_KEY::BTN_STYLUS3), None),
        (&EventCode::EV_KEY(EV_KEY::BTN_STYLUS), None),
        (&EventCode::EV_KEY(EV_KEY::BTN_STYLUS2), None),
        (&EventCode::EV_MSC(EV_MSC::MSC_SCAN), None),
    ];

    for event_type in event_types {
        d_pen.enable_event_type(event_type)?;
    }
    for event_code in event_codes {
        d_pen.enable_event_code(event_code.0, event_code.1)?;
    }

    let u_pen = UInputDevice::create_from_device(&d_pen)?;

    let d_touch = UninitDevice::new().unwrap();

    let real_screen_x_touch = 1862.;
    let real_screen_y_touch = 1398.;

    let zoom = 1.;

    let (size_x_touch, size_y_touch) = if false {
        let size_x = real_screen_x_touch * zoom;
        let size_y = x11_screen_y * size_x / x11_screen_x * zoom;
        (size_x, size_y)
    } else {
        let size_y = real_screen_y_touch * zoom;
        let size_x = x11_screen_x * size_y / x11_screen_y * zoom;
        (size_x, size_y)
    };

    let offset_x = 0;
    let offset_y = 0;

    let abs_x = AbsInfo {
        value: 0,
        minimum: offset_x,
        maximum: size_x_touch as i32 - offset_x,
        resolution: 0,
        fuzz: 0,
        flat: 0,
    };

    let abs_y = AbsInfo {
        value: 0,
        minimum: offset_y,
        maximum: size_y_touch as i32 - offset_y,
        resolution: 0,
        fuzz: 0,
        flat: 0,
    };

    let abs_pressure = AbsInfo {
        value: 0,
        minimum: 0,
        maximum: 46,
        resolution: 0,
        fuzz: 0,
        flat: 0,
    };

    let abs_mt_slot = AbsInfo {
        value: 0,
        minimum: 0,
        maximum: 31,
        resolution: 0,
        fuzz: 0,
        flat: 0,
    };

    let abs_mt_touch_major = AbsInfo {
        value: 0,
        minimum: 0,
        maximum: 255,
        resolution: 0,
        fuzz: 0,
        flat: 0,
    };

    let abs_mt_touch_minor = AbsInfo {
        value: 0,
        minimum: 0,
        maximum: 255,
        resolution: 0,
        fuzz: 0,
        flat: 0,
    };

    let abs_mt_position_x = AbsInfo {
        value: 0,
        minimum: offset_x,
        maximum: size_x_touch as i32 - offset_x,
        resolution: 0,
        fuzz: 0,
        flat: 0,
    };

    let abs_mt_position_y = AbsInfo {
        value: 0,
        minimum: offset_y,
        maximum: size_y_touch as i32 - offset_y,
        resolution: 0,
        fuzz: 0,
        flat: 0,
    };

    let abs_mt_tracking_id = AbsInfo {
        value: 0,
        minimum: 0,
        maximum: 65535,
        resolution: 0,
        fuzz: 0,
        flat: 0,
    };

    let abs_mt_pressure = AbsInfo {
        value: 0,
        minimum: 0,
        maximum: 46,
        resolution: 0,
        fuzz: 0,
        flat: 0,
    };

    d_touch.set_name("lirecom stylus touch");
    d_touch.set_bustype(BusType::BUS_I2C as u16);
    d_touch.set_vendor_id(0xabcf);
    d_touch.set_product_id(0xabcf);
    d_touch.enable(&InputProp::INPUT_PROP_DIRECT)?;

    let event_types = [&EventType::EV_SYN, &EventType::EV_KEY, &EventType::EV_ABS];

    let event_codes: [(&EventCode, Option<&(dyn Any + 'static)>); 15] = [
        (&EventCode::EV_SYN(EV_SYN::SYN_REPORT), None),
        (&EventCode::EV_SYN(EV_SYN::SYN_CONFIG), None),
        (&EventCode::EV_SYN(EV_SYN::SYN_MT_REPORT), None),
        (&EventCode::EV_SYN(EV_SYN::SYN_DROPPED), None),
        (&EventCode::EV_KEY(EV_KEY::BTN_TOUCH), None),
        (&EventCode::EV_ABS(EV_ABS::ABS_X), Some(&abs_x)),
        (&EventCode::EV_ABS(EV_ABS::ABS_Y), Some(&abs_y)),
        (
            &EventCode::EV_ABS(EV_ABS::ABS_PRESSURE),
            Some(&abs_pressure),
        ),
        (&EventCode::EV_ABS(EV_ABS::ABS_MT_SLOT), Some(&abs_mt_slot)),
        (
            &EventCode::EV_ABS(EV_ABS::ABS_MT_TOUCH_MAJOR),
            Some(&abs_mt_touch_major),
        ),
        (
            &EventCode::EV_ABS(EV_ABS::ABS_MT_TOUCH_MINOR),
            Some(&abs_mt_touch_minor),
        ),
        (
            &EventCode::EV_ABS(EV_ABS::ABS_MT_POSITION_X),
            Some(&abs_mt_position_x),
        ),
        (
            &EventCode::EV_ABS(EV_ABS::ABS_MT_POSITION_Y),
            Some(&abs_mt_position_y),
        ),
        (
            &EventCode::EV_ABS(EV_ABS::ABS_MT_TRACKING_ID),
            Some(&abs_mt_tracking_id),
        ),
        (
            &EventCode::EV_ABS(EV_ABS::ABS_MT_PRESSURE),
            Some(&abs_mt_pressure),
        ),
    ];

    for event_type in event_types {
        d_touch.enable_event_type(event_type)?;
    }
    for event_code in event_codes {
        d_touch.enable_event_code(event_code.0, event_code.1)?;
    }

    let u_touch = UInputDevice::create_from_device(&d_touch)?;

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed :(");
        let input = input.trim().split(' ').collect::<Vec<&str>>();
        if input.len() != 6 {
            continue;
        }

        let (device, mut value, event_code, time_f, time_l) = match input[..] {
            [device, value, event_code, _event_type, time_f, time_l] => {
                let event_code = match event_code {
                    "MSC_SCAN" => EventCode::EV_MSC(EV_MSC::MSC_SCAN),
                    "SYN_REPORT" => EventCode::EV_SYN(EV_SYN::SYN_REPORT),
                    "ABS_X" => EventCode::EV_ABS(EV_ABS::ABS_X),
                    "ABS_Y" => EventCode::EV_ABS(EV_ABS::ABS_Y),
                    "ABS_Z" => EventCode::EV_ABS(EV_ABS::ABS_Z),
                    "ABS_PRESSURE" => EventCode::EV_ABS(EV_ABS::ABS_PRESSURE),
                    "ABS_TILT_X" => EventCode::EV_ABS(EV_ABS::ABS_TILT_X),
                    "ABS_TILT_Y" => EventCode::EV_ABS(EV_ABS::ABS_TILT_Y),
                    "BTN_TOOL_PEN" => EventCode::EV_KEY(EV_KEY::BTN_TOOL_PEN),
                    "BTN_TOOL_RUBBER" => EventCode::EV_KEY(EV_KEY::BTN_TOOL_RUBBER),
                    "BTN_STYLUS3" => EventCode::EV_KEY(EV_KEY::BTN_STYLUS3),
                    "BTN_STYLUS2" => EventCode::EV_KEY(EV_KEY::BTN_STYLUS2),
                    "BTN_STYLUS" => EventCode::EV_KEY(EV_KEY::BTN_STYLUS),
                    "BTN_TOUCH" => EventCode::EV_KEY(EV_KEY::BTN_TOUCH),
                    "ABS_MT_SLOT" => EventCode::EV_ABS(EV_ABS::ABS_MT_SLOT),
                    "ABS_MT_TOUCH_MAJOR" => EventCode::EV_ABS(EV_ABS::ABS_MT_TOUCH_MAJOR),
                    "ABS_MT_TOUCH_MINOR" => EventCode::EV_ABS(EV_ABS::ABS_MT_TOUCH_MINOR),
                    "ABS_MT_POSITION_X" => EventCode::EV_ABS(EV_ABS::ABS_MT_POSITION_X),
                    "ABS_MT_POSITION_Y" => EventCode::EV_ABS(EV_ABS::ABS_MT_POSITION_Y),
                    "ABS_MT_TRACKING_ID" => EventCode::EV_ABS(EV_ABS::ABS_MT_TRACKING_ID),
                    "ABS_MT_PRESSURE" => EventCode::EV_ABS(EV_ABS::ABS_MT_PRESSURE),
                    _ => {
                        println!("{event_code}");
                        continue;
                    }
                };
                (
                    skip_none!(device.parse::<u8>().ok()),
                    skip_none!(value.parse::<i32>().ok()),
                    event_code,
                    skip_none!(time_f.parse::<i64>().ok()),
                    skip_none!(time_l.parse::<i64>().ok()),
                )
            }
            _ => continue,
        };

        match device {
            0 => u_tablet.write_event(&InputEvent {
                time: evdev_rs::TimeVal::new(time_f, time_l),
                event_code,
                value,
            })?,
            1 => u_pen.write_event(&InputEvent {
                time: evdev_rs::TimeVal::new(time_f, time_l),
                event_code,
                value,
            })?,
            2 => {
                match event_code {
                    EventCode::EV_ABS(EV_ABS::ABS_X | EV_ABS::ABS_MT_POSITION_X) => {
                        value = size_x_touch as i32 - value
                    }
                    EventCode::EV_ABS(EV_ABS::ABS_Y | EV_ABS::ABS_MT_POSITION_Y) => {
                        value = size_y_touch as i32 - value
                    }
                    _ => (),
                }
                u_touch.write_event(&InputEvent {
                    time: evdev_rs::TimeVal::new(time_f, time_l),
                    event_code,
                    value,
                })?;
            }
            _ => (),
        }
    }
}
