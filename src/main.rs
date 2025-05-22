// video-editor/src/main.rs

use macroquad::prelude::*;


#[macroquad::main("idk")]
async fn main() {
    let m = 50.0;

    let mut seek    = 0.0;
    let mut seeking = false;

    loop {
        if is_key_down(KeyCode::Escape) {
            break;
        }
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        clear_background(BLACK);
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        let sw = screen_width() - 2.0 * m;
        let sh = 75.0;
        let sx = m;
        let sy = screen_height() - m - sh;

        let (mx, my) = mouse_position();
        let hover = if mx >= sx && my >= sy && mx <= sx+sw && my <= sy+sh {
            if is_mouse_button_pressed(MouseButton::Left) {
                seeking = true;
            }
            true
        } else {
            false
        };
        if seeking {
            seek = ((mx - sx) / sw).clamp(0.0, 1.0);
        }
        if is_mouse_button_released(MouseButton::Left) {
            seeking = false;
        }

        draw_rectangle(sx, sy, sw, sh, if seeking { DARKBLUE } else if hover { SKYBLUE } else { BLUE });
        if hover {
            draw_line(mx, sy, mx, sy + sh, 3.0, BLACK);
        }
        draw_line(sx + sw * seek, sy, sx + sw * seek, sy + sh, 3.0, WHITE);
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        let cy = sy - m;
        draw_line(0.0, cy, screen_width(), cy, 1.0, DARKGRAY);

        let vcx = screen_width() * 0.5;
        let vcy = cy * 0.5;

        let ax = screen_width() - 2.0 * m;
        let ay = cy - 2.0 * m;

        let r  = 16.0 / 9.0;
        let hr = 1.0 / r;
        let vs = (ax * hr).min(ay);

        draw_rectangle(vcx - vs * (r * 0.5), vcy - vs * 0.5, vs * r, vs, DARKGRAY);
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        next_frame().await;
    }
}

