// Copyright (c) 2022 The Quantii Contributors
//
// This file is part of Quantii.
//
// Quantii is free software: you can redistribute
// it and/or modify it under the terms of the GNU
// Lesser General Public License as published by
// the Free Software Foundation, either version 3
// of the License, or (at your option) any later
// version.
//
// Quantii is distributed in the hope that it
// will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU Lesser General Public
// License for more details.
//
// You should have received a copy of the GNU
// Lesser General Public License along with
// Quantii. If not, see <https://www.gnu.org/licenses/>.

#![no_std]

extern crate novuskinc;
// Missing intrinsics patch.
extern crate compiler_builtins_patch;

use novuskinc::kernel::syscalls;

/// Novusk config
pub mod novusk;

/// The executable WASM for the kernel's initially running App
const APP_EXE: &[u8] = include_bytes!("hello_world.wasm");

struct System;

impl ardaku::System for System {
    /// Sleep until an event interrupt occurs.
    fn sleep(&self) -> (ardaku::Event, u32) {
        let byte = novuskinc::kernel::io::safe_sys_read();
        (ardaku::Event::Read, byte.into())
    }

    /// Write to the console.
    ///
    /// Note that the console is not shown to the actual onscreen
    /// OS (QEMU/emulators, VMs, or real hardware), but rather
    /// in a separate console. For QEMU, this would be the
    /// console used to start the QEMU process.
    fn write(&self, line: &[u8]) {
        for c in line {
            novuskinc::kernel::io::safe_sys_write(*c);
        }
        novuskinc::kernel::io::safe_sys_write(b'\n');
    }

    /// Return version of the kernel.
    fn version(&self) -> u32 {
        #[cfg(not(target_arch = "riscv32"))]
        unsafe {
            syscalls::syscall(syscalls::VERSION, 0).into()
        }

        #[cfg(target_arch = "riscv32")]
        {
            // TODO: Implement a better versioning system that works for RISC-V.
            3
        }
    }

    /// Reboot the system.
    ///
    /// This does not work on RISC-V targets.
    fn reboot(&self){
        // This is a no-op on RISC-V.
        #[cfg(target_arch = "riscv32")]
        Self::write(self, b"Reboot not supported on RISC-V");

        // On all other targets, proceed to reboot.
        #[cfg(not(target_arch = "riscv32"))]
        unsafe {
            syscalls::syscall(syscalls::REBOOT, 0);
        };
    }
}

pub fn setup() -> ! {
    use ardaku::System;

    System.write(b"\n\n=== ARDAKU STARTED ===\n");

    ardaku::start(System, APP_EXE).unwrap();

    System.write(b"\n=== ARDAKU STOPPED ===\n");

    // TODO: Do something more proper after the app has stopped.
    loop {
        System.sleep();
    }
}
