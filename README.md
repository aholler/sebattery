<!-- SPDX-FileCopyrightText: Copyright (c) 2025 Alexander Holler <holler@ahsoftware.de> -->
<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->

sebattery
---------

I wanted a tool to set the battery control mode of a SolarEdge Home
Battery without having to use a phone.

Here it is, written in Rust.

Just call make to build it.

Feel free to use an AI of your choice in order to translate it to a
language you like.

Alexander Holler


    user@host:~/Source/sebattery.git$ target/release/sebattery

    sebattery v1.0.0

    Usage:
            target/release/sebattery ip:port [new_control_mode]
    Examples:
            target/release/sebattery 127.0.0.1:1502
            target/release/sebattery 127.0.0.1:1502 1
    new_control_mode can be
            0 for Disabled
            1 for Maximize Self Consumption
            2 for Time of Use
            3 for Backup Only
            4 for Remote Control

    user@host:~/privat/sebattery.git$ target/release/sebattery 127.0.0.1:1502

    sebattery v1.1.0

    Fetching ...
    Disconnecting
    storage control mode: Maximize Self Consumption
    Ladezustand: 7.777778%
    Durschnittstemperatur: 19.7°C
    Maximaltemperatur: 0°C
    Spannung aktuell: 778.8882V
    Strom aktuell: 0.15347122A
    Leistung aktuell: -120W
    Lifetime Export Energy Counter: 440Wh
    Lifetime Import Energy Counter: 0Wh
    Maximale Leistung: 4600Wh
    Verfügbare Leistung: 4608Wh
    Health: 100%
    State: Discharge
    State internal: 3
    Storage AC charge policy: 1
    Storage AC charge limit: 0kWh
    Storage backup reserved setting: 0%
    Storage charge/discharge default mode: Off
    remote control command timeout: 3600s
    remote contol command mode: 65535
    remote control charge limit: 5000W
    remote control command discharge limit: 5000W
