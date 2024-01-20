### Debug Project: Bluetooth: bt_att: Unable to allocate buffer

This repo consists of an example Rust project to help illustrate the issue in:
https://github.com/zephyrproject-rtos/zephyr/issues/67273

To replicate the buffer issue, first flash the `peripheral_hr` example project onto your device. For example:

```bash
west build samples/bluetooth/peripheral_hr -b nrf52dk_nrf52832
west flash
```

Then use this project to connect to your device:

```bash
cargo run
```

An example of the output is provided:

```text
****************************************
Starting iteration loop: 0
****************************************
Step 0: Characteristics of interest: [0000180d-0000-1000-8000-00805f9b34fb, 00002a37-0000-1000-8000-00805f9b34fb]
Step 1: Initialise a bluetooth adaptor
Step 2: Scan for the device with characteristic: 0000180d-0000-1000-8000-00805f9b34fb
Step 3: Connect to the device
Step 4: Attempt to discover hr_measurement_characteristic: 00002a37-0000-1000-8000-00805f9b34fb
Step 5: Discovered characteristic: 00002a37-0000-1000-8000-00805f9b34fb
Step 6: Enable notifications on characteristic: 00002a37-0000-1000-8000-00805f9b34fb
Step 7: Disconnect from the device

****************************************
Starting iteration loop: 1
****************************************
Step 0: Characteristics of interest: [0000180d-0000-1000-8000-00805f9b34fb, 00002a37-0000-1000-8000-00805f9b34fb]
Step 1: Initialise a bluetooth adaptor
Step 2: Scan for the device with characteristic: 0000180d-0000-1000-8000-00805f9b34fb
Step 3: Connect to the device
Step 4: Attempt to discover hr_measurement_characteristic: 00002a37-0000-1000-8000-00805f9b34fb
Step 5: Discovered characteristic: 00002a37-0000-1000-8000-00805f9b34fb
Step 6: Enable notifications on characteristic: 00002a37-0000-1000-8000-00805f9b34fb
Step 7: Disconnect from the device

****************************************
Starting iteration loop: 2
****************************************
Step 0: Characteristics of interest: [0000180d-0000-1000-8000-00805f9b34fb, 00002a37-0000-1000-8000-00805f9b34fb]
Step 1: Initialise a bluetooth adaptor
Step 2: Scan for the device with characteristic: 0000180d-0000-1000-8000-00805f9b34fb
Step 3: Connect to the device
Step 4: Attempt to discover hr_measurement_characteristic: 00002a37-0000-1000-8000-00805f9b34fb
Step 5: Discovered characteristic: 00002a37-0000-1000-8000-00805f9b34fb
Step 6: Enable notifications on characteristic: 00002a37-0000-1000-8000-00805f9b34fb
Step 7: Disconnect from the device

****************************************
Starting iteration loop: 3
****************************************
Step 0: Characteristics of interest: [0000180d-0000-1000-8000-00805f9b34fb, 00002a37-0000-1000-8000-00805f9b34fb]
Step 1: Initialise a bluetooth adaptor
Step 2: Scan for the device with characteristic: 0000180d-0000-1000-8000-00805f9b34fb
Step 3: Connect to the device
Step 4: Attempt to discover hr_measurement_characteristic: 00002a37-0000-1000-8000-00805f9b34fb <--- Output hangs here before the Rust process panics
thread 'main' panicked at src/main.rs:29:52:
Failed to parse services: Error { kind: NotConnected, source: None, message: "" }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

The output of the serial logs should indicate:

```text
*** Booting Zephyr OS build zephyr-v3.5.0-4318-g9569e4410475 ***
[00:00:00.255,310] <inf> bt_hci_core: HW Platform: Nordic Semiconductor (0x0002)
[00:00:00.255,340] <inf> bt_hci_core: HW Variant: nRF52x (0x0002)
[00:00:00.255,371] <inf> bt_hci_core: Firmware: Standard Bluetooth controller (0x00) Version 3.5 Build 99
[00:00:00.256,103] <inf> bt_hci_core: Identity: D2:11:DE:BE:8F:2B (random)
[00:00:00.256,134] <inf> bt_hci_core: HCI: version 5.4 (0x0d) revision 0x0000, manufacturer 0x05f1
[00:00:00.256,164] <inf> bt_hci_core: LMP: version 5.4 (0x0d) subver 0xffff
Bluetooth initialized
Advertising successfully started
Connected
[00:00:04.750,335] <inf> hrs: HRS notifications enabled
HRS notification status changed: enabled
[00:00:04.795,623] <inf> hrs: HRS notifications disabled
HRS notification status changed: disabled
Disconnected (reason 0x13)
Connected
[00:00:06.122,772] <inf> hrs: HRS notifications enabled
HRS notification status changed: enabled
[00:00:06.168,060] <inf> hrs: HRS notifications disabled
HRS notification status changed: disabled
Disconnected (reason 0x13)
Connected
[00:00:07.202,728] <inf> hrs: HRS notifications enabled
HRS notification status changed: enabled
[00:00:07.247,985] <inf> hrs: HRS notifications disabled
HRS notification status changed: disabled
Disconnected (reason 0x13)
Connected
[00:00:37.877,563] <wrn> bt_conn: Unable to allocate buffer within timeout
[00:00:37.877,593] <err> bt_att: Unable to allocate buffer for op 0x03
[00:01:07.877,655] <wrn> bt_conn: Unable to allocate buffer within timeout
[00:01:07.877,685] <err> bt_att: Unable to allocate buffer for op 0x01
Disconnected (reason 0x13)
```

In the output example, I have used the latest Zephyr [origin/main commit](https://github.com/zephyrproject-rtos/zephyr/commit/9569e4410475fb62125e769a13222882902d8c75) at the time to compile the example `peripheral_hr` project.

```bash
commit 9569e4410475fb62125e769a13222882902d8c75 (HEAD -> main, origin/main, origin/HEAD)
Author: Martí Bolívar <mbolivar@amperecomputing.com>
Date:   Wed Jan 17 12:22:40 2024 -0800

    MAINTAINERS: retire myself as DT maintainer

    Unfortunately, I don't have the cycles to do this job well at the
    moment.

    Signed-off-by: Martí Bolívar <mbolivar@amperecomputing.com>
```

A copy of the packet trace is included in the `packet-trace-data` folder.