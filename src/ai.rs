use {GameEnv, Circle};
use core::prelude::*;

pub fn get_simulate_buttons(env: &GameEnv) -> u32 {

    // The y coordinate of the obstical is given by the frame counter in the GameEnv
    let gap_y = env.frame;

    let (gap_1_start, gap_1_end) = env.obstacle.gap1;
    let gap1 = ((gap_1_start + gap_1_end) / 2, gap_y);

    let button_pattern = match env.obstacle.gap2 {
        Some((gap_2_start, gap_2_end)) => {
            let gap2 = ((gap_2_start + gap_2_end) / 2, gap_y);

            let diff1_1 = distance(&env.circle1, gap1);
            let diff2_1 = distance(&env.circle2, gap1);
            let diff1_2 = distance(&env.circle1, gap2);
            let diff2_2 = distance(&env.circle2, gap2);

            if diff1_1 > diff1_2 && diff2_1 > diff2_2 {
                both_go_to_gap(env, gap2)
            } else if diff1_1 < diff1_2 && diff2_1 < diff2_2 {
                both_go_to_gap(env, gap1)
            } else if diff1_1 < diff1_2 {
                go_to_gap(env, 0, gap1) | go_to_gap(env, 1, gap2)
            } else {
                go_to_gap(env, 0, gap2) | go_to_gap(env, 1, gap1)
            }
        },
        None => both_go_to_gap(env, gap1)
    };

    !button_pattern

}

fn distance(circle: &Circle, gap_center: (i32,i32)) -> i32 {
    let x = circle.rect.dx + 25;
    let y = circle.rect.dy + 25;
    let (g_x, g_y) = gap_center;

    let pow_2 = |a| a * a;

    pow_2(x - g_x) + pow_2(y - g_y)
}

fn both_go_to_gap(env: &GameEnv, gap_center: (i32, i32)) -> u32 {

    let mut buttons = 0;

    let gap_x = gap_center.0;

    // Move circle 1 on x axis
    if (env.circle1.rect.dx + 25) < gap_x {
        buttons |= 0x4;
    } else {
        buttons |= 0x1;
    }

    // Move circle 2 on x axis
    if (env.circle2.rect.dx + 25) < gap_x {
        buttons |= 0x40;
    } else {
        buttons |= 0x10;
    }

    let diff1 = distance(&env.circle1, gap_center);
    let diff2 = distance(&env.circle2, gap_center);

    if diff1 < diff2 {
        buttons |= 0x80;
        if (env.circle1.rect.dy + 25) > 165 {
            buttons |= 0x2;
        }
    } else {
        buttons |= 0x8;
        if (env.circle2.rect.dy + 25) > 165 {
            buttons |= 0x20;
        }
    }

    buttons
}

fn go_to_gap(env: &GameEnv, circle: u32, gap_center: (i32, i32)) -> u32 {

    let gap_x = gap_center.0;

    if circle == 0 {
        let x = env.circle1.rect.dx + 25;
        0x8 | if x < gap_x { 0x4 } else { 0x1 }
    } else {
        let x = env.circle2.rect.dx + 25;
        0x80 | if x < gap_x { 0x40 } else { 0x10 }
    }

}
