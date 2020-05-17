mod cpu;
mod bus;
mod ppu;
mod rom;

use crate::cpu::{Cpu, Interrupt};
use crate::bus::Bus;
use crate::ppu::Ppu;
use crate::rom::Rom;

use std::{thread, time};

fn main() {
	let rom = Rom::new("roms/vram_access.nes").unwrap();
    //let rom = Rom::new("roms/full_nes_palette.nes").unwrap();
	//let rom = Rom::new("roms/color_test.nes").unwrap();
    //let rom = Rom::new("roms/mario.nes").unwrap();
    //let rom = Rom::new("roms/balloon.nes").unwrap();
    //let rom = Rom::new("roms/color_test.nes").unwrap();
    //let rom = Rom::new("roms/nestest.nes").unwrap();
    //let rom = Rom::new("roms/mario.nes").unwrap();
	//let rom = Rom::new("roms/scanline.nes").unwrap();
	//let rom = Rom::new("/home/quantatic/nes-test-roms/blargg_nes_cpu_test5/cpu.nes").unwrap();
	//let rom = Rom::new("roms/nestest.nes").unwrap();

    let sdl_context = sdl2::init()
        .unwrap();

    let ppu = Ppu::new(&sdl_context);

    let mut bus = Bus::new(rom, ppu);
    let mut cpu = Cpu::new(&mut bus);

    cpu.interrupt(Interrupt::Reset);

    let mut master_clock_ticks: u64 = 0;

    // Master clocks run at 21.477272 MHz
    loop {
        // CPU runs every 12 master ticks
        if master_clock_ticks % 12 == 0 {
            cpu.step();
        }

        // PPU runs every 4 master ticks
        if master_clock_ticks % 4 == 0 {
            cpu.bus.ppu.step();
        }

        // Very slow -- figure out why?
        if master_clock_ticks % 100000 == 0 {
            cpu.bus.ppu.check_for_exit();
			//thread::sleep(time::Duration::from_micros(500));
        }

        master_clock_ticks += 1;
    }
}
