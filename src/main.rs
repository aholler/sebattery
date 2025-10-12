//
// Written 2025 by Alexander Holler
//
// SPDX-FileCopyrightText: Copyright (c) 2025 Alexander Holler <holler@ahsoftware.de>
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::env;
use tokio;

fn v_u16_to_u64(data: &Vec<u16>) -> u64 {
    let bytes: Vec<u8> = data.iter().fold(vec![], |mut x, elem| {
        x.push((elem & 0xff) as u8);
        x.push((elem >> 8) as u8);
        x
    });
    let b: [u8; 8] = bytes.try_into().unwrap();
    return u64::from_ne_bytes(b);
}


fn v_u16_to_f32(data: &Vec<u16>) -> f32 {
    let bytes: Vec<u8> = data.iter().fold(vec![], |mut x, elem| {
        x.push((elem & 0xff) as u8);
        x.push((elem >> 8) as u8);
        x
    });
    let b: [u8; 4] = bytes.try_into().unwrap();
    return f32::from_ne_bytes(b);
}

fn v_u16_to_u32(data: &Vec<u16>) -> u32 {
    let mut rc0: u32 = data[1].into();
    rc0 >>= 16;
    let rc1: u32 = data[0].into();
    return rc0 + rc1;
}

fn print_help(my_name: &str) {
        eprintln!("Usage:");
        eprintln!("\t{} ip:port [new_control_mode]", my_name);
        eprintln!("Examples:");
        eprintln!("\t{} 127.0.0.1:1502", my_name);
        eprintln!("\t{} 127.0.0.1:1502 1", my_name);
        eprintln!("new_control_mode can be");
        eprintln!("\t0 for Disabled");
        eprintln!("\t1 for Maximize Self Consumption");
        eprintln!("\t2 for Time of Use");
        eprintln!("\t3 for Backup Only");
        eprintln!("\t4 for Remote Control\n");
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use tokio_modbus::prelude::*;

    println!("\nsebattery v{}\n", env!("CARGO_PKG_VERSION"));

    let args: Vec<String> = env::args().collect();

    let mut new_control_mode : u16 = 5;

    if args.len() == 3 {
        new_control_mode = args[2].parse().unwrap();
        if new_control_mode > 4 {
            print_help(&args[0]);
            std::process::exit(1)
        }
    } else if args.len() < 2 || args.len() > 3 {
        print_help(&args[0]);
        std::process::exit(1)
    }

    let socket_addr = args[1].parse().unwrap();

    let slave = Slave(0x01);
    let mut ctx = tcp::connect_slave(socket_addr, slave).await?;

    println!("Fetching ...");
    let data = ctx.read_holding_registers(0xf56c, 30).await??;

    let data_storage_control_mode = ctx.read_holding_registers(0xf704, 1).await??;

    if new_control_mode < 5 {
        println!("Setting new mode");
        ctx.write_single_register(0xf704, new_control_mode).await??;
    }
    println!("Disconnecting");

    ctx.disconnect().await?;

    let control_modes: [&str; 5] = ["Disabled", "Maximize Self Consumption", "Time of Use", "Backup Only", "Remote Control"];
    if data_storage_control_mode[0] < 5 {
        println!("storage control mode: {}", control_modes[usize::try_from(data_storage_control_mode[0]).unwrap()]);
    } else {
        println!("storage control mode: unknown");
    }

    println!("Ladezustand: {}%", v_u16_to_f32(&data[24..26].to_vec()));
    println!("Durschnittstemperatur: {}°C", v_u16_to_f32(&data[0..2].to_vec()));
    println!("Maximaltemperatur: {}°C", v_u16_to_f32(&data[2..4].to_vec()));
    println!("Spannung aktuell: {}V", v_u16_to_f32(&data[4..6].to_vec()));
    println!("Strom aktuell: {}A", v_u16_to_f32(&data[6..8].to_vec()));
    println!("Leistung aktuell: {}W", v_u16_to_f32(&data[8..10].to_vec()));
    println!("Lifetime Export Energy Counter: {}Wh", v_u16_to_u64(&data[10..14].to_vec()));
    println!("Lifetime Import Energy Counter: {}Wh", v_u16_to_u64(&data[14..18].to_vec()));
    println!("Maximale Leistung: {}Wh", v_u16_to_f32(&data[18..20].to_vec()));
    println!("Verfügbare Leistung: {}Wh", v_u16_to_f32(&data[20..22].to_vec()));
    println!("Health: {}%", v_u16_to_f32(&data[22..24].to_vec()));
    let states: [&str; 7] = ["Off", "Standby", "Init", "Charge", "Discharge", "Fault", "Idle"];
    let state = v_u16_to_u32(&data[26..28].to_vec());
    if state < 7 {
        println!("State: {}", states[usize::try_from(state).unwrap()]);
    } else {
        println!("State: unknown");
    }
    println!("State internal: {}", v_u16_to_u32(&data[28..30].to_vec()));

    Ok(())
}
