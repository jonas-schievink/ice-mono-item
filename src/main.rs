#![no_std]

#[macro_use]
extern crate nb;
extern crate nrf51_hal;
extern crate nrf51;
extern crate embedded_hal;
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
extern crate cortex_m_rtfm as rtfm;
extern crate fpa;
extern crate byteorder;
extern crate panic_halt;

pub mod ble;

#[macro_use]
mod macros;
mod temp;
mod radio;

use ble::link::{LinkLayer, AddressKind, DeviceAddress};
use ble::link::ad_structure::{AdStructure, Flags};
pub use ble::link::MAX_PDU_SIZE;

use temp::Temp;
use radio::{BleRadio, Baseband};

use cortex_m::asm;
use rtfm::{app, Threshold};
use byteorder::{ByteOrder, LittleEndian};

use core::time::Duration;
use core::u32;

app! {
    device: nrf51,

    resources: {
        static BLE_TX_BUF: ::radio::PacketBuffer = [0; ::MAX_PDU_SIZE + 1];
        static BLE_RX_BUF: ::radio::PacketBuffer = [0; ::MAX_PDU_SIZE + 1];
        static BASEBAND: Baseband;
        static BLE_TIMER: nrf51::TIMER0;
    },

    init: {
        resources: [BLE_TX_BUF, BLE_RX_BUF],
    },

    idle: {
        resources: [BASEBAND],
    },

    tasks: {
        RADIO: {
            path: radio,
            resources: [BASEBAND, BLE_TIMER],
        },

        TIMER0: {
            path: radio_timer,
            resources: [BASEBAND, BLE_TIMER],
        }
    },
}

fn init(p: init::Peripherals, res: init::Resources) -> init::LateResources {
    // On reset, internal 16MHz RC oscillator is active. Switch to ext. 16MHz crystal. This is
    // needed for Bluetooth to work (but is apparently done on radio activation, too?).

    // Ext. clock freq. defaults to 32 MHz for some reason
    p.device.CLOCK.xtalfreq.write(|w| w.xtalfreq()._16mhz());
    p.device.CLOCK.tasks_hfclkstart.write(|w| unsafe { w.bits(1) });
    while p.device.CLOCK.events_hfclkstarted.read().bits() == 0 {}

    // TIMER0 cfg, 32 bit @ 1 MHz
    // Mostly copied from the `nrf51-hal` crate.
    p.device.TIMER0.bitmode.write(|w| w.bitmode()._32bit());
    p.device.TIMER0.prescaler.write(|w| unsafe { w.prescaler().bits(4) });
    p.device.TIMER0.intenset.write(|w| w.compare0().set());
    p.device.TIMER0.shorts.write(|w| w
        .compare0_clear().enabled()
        .compare0_stop().enabled()
    );

    let mut devaddr = [0u8; 6];
    let devaddr_lo = p.device.FICR.deviceaddr[0].read().bits();
    let devaddr_hi = p.device.FICR.deviceaddr[1].read().bits() as u16;
    LittleEndian::write_u32(&mut devaddr, devaddr_lo);
    LittleEndian::write_u16(&mut devaddr[4..], devaddr_hi);

    let devaddr_type = if p.device.FICR.deviceaddrtype.read().deviceaddrtype().is_public() {
        AddressKind::Public
    } else {
        AddressKind::Random
    };
    let device_address = DeviceAddress::new(devaddr, devaddr_type);

    let mut ll = LinkLayer::new(device_address);
    ll.start_advertise(Duration::from_millis(100), &[
        AdStructure::Flags(Flags::discoverable()),
        AdStructure::CompleteLocalName("CONCVRRENS CERTA CELERIS"),
    ]);

    // Queue first baseband update
    cfg_timer(&p.device.TIMER0, Some(Duration::from_millis(1)));

    let mut temp = Temp::new(p.device.TEMP);
    temp.start_measurement();
    let temp = block!(temp.read()).unwrap();
    heprintln!("{}°C", temp);

    init::LateResources {
        BASEBAND: Baseband::new(BleRadio::new(p.device.RADIO, &p.device.FICR, res.BLE_TX_BUF), res.BLE_RX_BUF, ll),
        BLE_TIMER: p.device.TIMER0,
    }
}

fn idle(_t: &mut Threshold, _res: idle::Resources) -> ! {
    loop {
        asm::wfi();
    }
}

fn radio(_t: &mut Threshold, mut res: RADIO::Resources) {
    if let Some(new_timeout) = res.BASEBAND.interrupt() {
        cfg_timer(&res.BLE_TIMER, Some(new_timeout));
    }
}

fn radio_timer(_t: &mut Threshold, mut res: TIMER0::Resources) {
    heprint!("T");
    let maybe_next_update = res.BASEBAND.update();
    cfg_timer(&res.BLE_TIMER, maybe_next_update);
}

/// Reconfigures TIMER0 to raise an interrupt after `duration` has elapsed.
///
/// TIMER0 is stopped if `duration` is `None`.
///
/// Note that if the timer has already queued an interrupt, the task will still be run after the
/// timer is stopped by this function.
fn cfg_timer(t: &nrf51::TIMER0, duration: Option<Duration>) {
    // Timer activation code is also copied from the `nrf51-hal` crate.
    if let Some(duration) = duration {
        assert!(duration.as_secs() < ((u32::MAX - duration.subsec_micros()) / 1_000_000) as u64);
        let us = (duration.as_secs() as u32) * 1_000_000 + duration.subsec_micros();
        t.cc[0].write(|w| unsafe { w.bits(us) });
        t.events_compare[0].reset();   // acknowledge last compare event (FIXME unnecessary?)
        t.tasks_clear.write(|w| unsafe { w.bits(1) });
        t.tasks_start.write(|w| unsafe { w.bits(1) });
    } else {
        t.tasks_stop.write(|w| unsafe { w.bits(1) });
        t.tasks_clear.write(|w| unsafe { w.bits(1) });
        t.events_compare[0].reset();   // acknowledge last compare event (FIXME unnecessary?)
    }
}
