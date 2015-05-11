use kits::dk::bsp;

use core::intrinsics::{volatile_load, volatile_store};
static mut ms_ticks: u32 = 0;

extern {
    fn on_systick(ms_ticks: u32);
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern fn SysTick_Handler() {
    let ticks = volatile_load(&ms_ticks as *const u32) + 1;
    volatile_store(&mut ms_ticks as *mut u32, ticks);

    on_systick(ticks);
}

pub fn delay(num_ticks: u32) {
    unsafe {
        let cur_ticks = volatile_load(&ms_ticks as *const u32);
        while volatile_load(&ms_ticks as *const u32) - cur_ticks < num_ticks {}
    }
}

pub fn blink(n: u32) {
    for _ in 0 .. n {
        bsp::leds_set(0xffff);
        delay(100);
        bsp::leds_set(0x0000);
        delay(100);
    }
}
