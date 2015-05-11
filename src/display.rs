use core::intrinsics::volatile_store;
use collections::string::String;

use emlib::ebi;
use emlib::ebi::{TFTInit};

use emdrv::tft;

use cmsis::nvic;

use utils;

use {Circle, Obstacle};

use font16x28::FONT_16X28;

pub const WIDTH: usize = 320;
pub const HEIGHT: usize = 240;

// Virtual width and height
pub const V_WIDTH: usize = 672;
pub const V_HEIGHT: usize = 240;

pub const CIRCLE_SAMPLES: usize = 4 + 33 * 4;

pub const CIRCLE_OFFSETS: [i32; CIRCLE_SAMPLES] = [-24,-696,-1368,-2040,-2712,-3383,-4055,-4727,-5399,-6070,-6742,-7413,-8085,-8756,-9428,-10099,-10770,-11441,-12112,-12783,-13454,-13453,-14124,-14123,-14794,-14793,-15464,-15463,-15462,-15461,-16132,-16131,-16130,-16129,-16128,-16127,-16126,-16125,-16124,-15451,-15450,-15449,-15448,-14775,-14774,-14101,-14100,-13427,-13426,-12753,-12080,-11407,-10734,-10061,-9388,-8716,-8043,-7371,-6698,-6026,-5353,-4681,-4009,-3337,-2664,-1992,-1320,-648,24,696,1368,2040,2712,3383,4055,4727,5399,6070,6742,7413,8085,8756,9428,10099,10770,11441,12112,12783,13454,13453,14124,14123,14794,14793,15464,15463,15462,15461,16132,16131,16130,16129,16128,16127,16126,16125,16124,15451,15450,15449,15448,14775,14774,14101,14100,13427,13426,12753,12080,11407,10734,10061,9388,8716,8043,7371,6698,6026,5353,4681,4009,3337,2664,1992,1320,648];

pub static TFT_INIT: TFTInit = TFTInit {
    bank:            ebi::TFTBank::_2,
    width:           ebi::TFTWidth::HalfWord,
    colsrc:          ebi::TFTColorSrc::Mem,
    interleave:      ebi::TFTInterleave::Unlimited,
    fb_trigger:      ebi::TFTFrameBufTrigger::HSync,
    shift_dclk:      false,
    mask_blend:      ebi::TFTMaskBlend::Disabled,
    drive_mode:      ebi::TFTDDMode::External,
    cs_polarity:     ebi::Polarity::ActiveLow,
    dclk_polarity:   ebi::Polarity::ActiveHigh,
    dataen_polarity: ebi::Polarity::ActiveLow,
    hsync_polarity:  ebi::Polarity::ActiveLow,
    vsync_polarity:  ebi::Polarity::ActiveLow,
    hsize:           320,
    h_porch_front:   1,
    h_porch_back:    30,
    h_pulse_width:   2,
    vsize:           240,
    v_porch_front:   1,
    v_porch_back:    4,
    v_pulse_width:   2,
    address_offset:  0x0000,
    dclk_period:     8,
    start_position:  0,
    setup_cycles:    0,
    hold_cycles:     0,
};


static NUMBERS: [[[bool; 3]; 5]; 10] = [[
    [true, true, true],
    [true, false, true],
    [true, false, true],
    [true, false, true],
    [true, true, true],
],[
    [false, false, true],
    [false, false, true],
    [false, false, true],
    [false, false, true],
    [false, false, true],
],[
    [true, true, true],
    [false, false, true],
    [true, true, true],
    [true, false, false],
    [true, true, true],
],[
    [true, true, true],
    [false, false, true],
    [true, true, true],
    [false, false, true],
    [true, true, true],
],[
    [true, false, true],
    [true, false, true],
    [true, true, true],
    [false, false, true],
    [false, false, true],
],[
    [true, true, true],
    [true, false, false],
    [true, true, true],
    [false, false, true],
    [true, true, true],
],[
    [true, true, true],
    [true, false, false],
    [true, true, true],
    [true, false, true],
    [true, true, true],
],[
    [true, true, true],
    [false, false, true],
    [false, true, true],
    [false, true, false],
    [false, true, false],
],[
    [true, true, true],
    [true, false, true],
    [true, true, true],
    [true, false, true],
    [true, true, true],
],[
    [true, true, true],
    [true, false, true],
    [true, true, true],
    [false, false, true],
    [false, false, true],
],];


pub fn init() -> bool {
    tft::direct_init(&TFT_INIT)
}

pub fn irq_enable(flags: u32) {
    ebi::int_disable(ebi::IF_MASK);
    ebi::int_clear(ebi::IF_MASK);
    ebi::int_enable(flags);

    nvic::clear_pending_irq(nvic::IRQn::EBI);
    nvic::enable_irq(nvic::IRQn::EBI);
}

// Keep track of horizontal offset
static mut hz_offset: u32 = 0;
static mut frame_ctr: u32 = 0;

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern fn EBI_IRQHandler() {
    let flags = ebi::int_get();
    ebi::int_clear(flags);

    // Process vertical sync interrupt
    if (flags & ebi::IF_VFPORCH) != 0 {
        // Keep track of number of frames drawn
        frame_ctr += 1;

        // Increase this increment to 2/4/8 to increase scroll speed
        hz_offset += 1;

        // TODO: Not sure if this if-statement is required or not. What does it do?
        // Wrap around when a full screen has been displayed
        // if (hz_offset == (D_WIDTH + font16x28.c_width)) {
        //     hz_offset = 0;
        // }
    }

    // Process horizontal sync interrupt
    if (flags & ebi::IF_HSYNC) != 0 {
        let mut line_number: u32 = ebi::tftv_count();

        // Adjust for porch size
        if line_number >= 3 {
            line_number -= 3;
        }

        ebi::tft_frame_base_set(line_number * V_WIDTH as u32 * 2);
    }
}

pub trait BufferLen {
    fn buffer_len() -> usize;
}

impl BufferLen for u8 {
    fn buffer_len() -> usize { (V_WIDTH * V_HEIGHT * 2) as usize }
}

impl BufferLen for u16 {
    fn buffer_len() -> usize { (V_WIDTH * V_HEIGHT) as usize }
}

impl BufferLen for u32 {
    fn buffer_len() -> usize { (V_WIDTH * V_HEIGHT / 2) as usize }
}

impl BufferLen for u64 {
    fn buffer_len() -> usize { (V_WIDTH * V_HEIGHT / 4) as usize }
}

macro_rules! frame_buffer {
    () => (ebi::bank_address(ebi::BANK2) as *mut u16)
}

macro_rules! set {
    ($fb:ident, $i:expr, $val:expr) => {
        unsafe {
            volatile_store($fb.offset($i as isize), $val);
        }
    }
}

pub fn clear() {
    let fb = frame_buffer!();
    // Clear entire display using 32-bit write operations.
    for i in 0 .. V_WIDTH * V_HEIGHT {
        set!(fb, i, 0);
    }
}
pub fn draw_number(number: usize, mut pos: usize, color: u16) {
    let mut current_score = number;
    pos = pos + 16; // Start with the third position

    let fb = frame_buffer!();
    for _ in 0 .. 3 {
        let num: usize = current_score % 10;
        current_score = current_score / 10;
        let mut yy: usize = 0;
        for y in 0 .. 5 {
            let mut xx: usize = 0;
            for x in 0 .. 3 {
                let c = if NUMBERS[num][y][x] { color } else { 0 };

                set!(fb, pos+xx+yy, c);
                xx += 1;
                set!(fb, pos+xx+yy, c);
                xx += 1;
            }
            yy += V_WIDTH as usize;
            xx = 0;
            for x in 0 .. 3 {
                let c = if NUMBERS[num][y][x] { color } else { 0 };

                set!(fb, pos+xx+yy, c);
                xx += 1;
                set!(fb, pos+xx+yy, c);
                xx += 1;
            }
            yy += V_WIDTH as usize;
        }
        pos -= 8;
    }
}

#[inline(always)]
pub fn clear_circle(circle: &Circle) {
    let fb = frame_buffer!();
    for i in 0 .. CIRCLE_SAMPLES {
        let idx = (circle.center as i32 + CIRCLE_OFFSETS[i]) as usize;
        if idx > 0 {
            set!(fb, idx, 0);
        }
    }
}

#[inline(always)]
pub fn draw_circle(circle: &Circle) {
    let fb = frame_buffer!();
    let mut color = circle.color;

    for i in 0 .. CIRCLE_SAMPLES {
        let idx = (circle.center as i32 + CIRCLE_OFFSETS[i]) as usize;
        if idx > 0 {
            set!(fb, idx, color);
            color += 32;
        }
    }
}

#[inline(always)]
pub fn draw_obstacle(obstacle: &Obstacle) {
    let fb = frame_buffer!();
    for i in 0..WIDTH {
        if obstacle.obstacle[i] {
            set!(fb, obstacle.pos + i, 63488);

            if obstacle.pos >= 600 {
                set!(fb, obstacle.pos + i - 1 * V_WIDTH as usize, 57344);
            }
            if obstacle.pos >= 1200 {
                set!(fb, obstacle.pos + i - 2 * V_WIDTH as usize, 64);
            }
            if obstacle.pos >= 2000 {
                set!(fb, obstacle.pos + i - 3 * V_WIDTH as usize, 0);
            }
        }

    }
}
pub fn draw_fps(fps: u32) {

		//draw_number(fps as usize, 10 + 10 * V_WIDTH, 0xffff);

		let text = format!("{} fps ", fps);
		draw_string(0, 10, text);

}

fn draw_string(mut x: usize, y: usize, text: String) {

    for ch in text.chars() {
        draw_font(x, y, ch);
        x += 16;
    }
}

fn draw_font(x: usize, y: usize, c: char) {
    let fb = frame_buffer!();
    let font = &FONT_16X28;
    let font_offset = c as usize - 0x20;
    let mut idx = x + (y*V_WIDTH);


    for j in 0..font.c_height {

        for i in 0..font.c_width {

            let color = font.data[j * font.width + font_offset * font.c_width + i];
            set!(fb, idx, color);
            idx += 1;
        }

        idx += V_WIDTH - font.c_width;

    }

}

pub fn debug_count() {
    let mut num = 999;
    loop {
        draw_number(num, (250 + 10 * V_WIDTH) as usize, 0xffff);
        num = if num == 0 { 999 } else { num - 1 };
        utils::delay(10);
    }
}
