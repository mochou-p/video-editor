// video-editor/src/main.rs

use macroquad::prelude::*;


#[macroquad::main("video-editor")]
async fn main() {
    let mut state = State::default();
    let     style = Style::default();

    loop {
        if is_key_down(KeyCode::Escape) {
            break;
        }

        clear_background(style.background);

        state.width                    = screen_width();
        state.height                   = screen_height();
        (state.mouse_x, state.mouse_y) = mouse_position();

        seek_bar(&mut state, &style); 
        video(&state, &style);

        next_frame().await;
    }
}

#[derive(Default)]
struct State {
    // available space
    // x:       f32,
    // y:       f32,
    width:   f32,
    height:  f32,
    // input
    mouse_x: f32,
    mouse_y: f32,
    // seeking
    seek:    f32,
    seeking: bool
}

// these do not change with themes or resizes
const SEEK_BAR_HEIGHT: f32 = 70.0;

struct Style {
    // space
    margin:          f32,
    // colors
    line_width:      f32,
    background:      Color,
    video:           Color,
    seek_bar:        Color,
    seek_bar_hover:  Color,
    seek_bar_click:  Color,
    seek_line:       Color,
    seek_hover_line: Color
}

impl Default for Style {
    fn default() -> Self {
        Self {
            margin:          20.0,
            line_width:      3.0,
            background:      BLACK,
            video:           DARKGRAY,
            seek_bar:        BLUE,
            seek_bar_hover:  SKYBLUE,
            seek_bar_click:  DARKBLUE,
            seek_line:       WHITE,
            seek_hover_line: BLACK
        }
    }
}

fn seek_bar(state: &mut State, style: &Style) {
    let width  = state.width - 2.0 * style.margin;
    let x      = style.margin;
    let y      = state.height - style.margin - SEEK_BAR_HEIGHT;

    let hover = if state.mouse_x >= x && state.mouse_y >= y && state.mouse_x <= x + width && state.mouse_y <= y + SEEK_BAR_HEIGHT {
        if is_mouse_button_pressed(MouseButton::Left) {
            state.seeking = true;
        }

        true
    } else {
        false
    };

    if state.seeking {
        state.seek = ((state.mouse_x - x) / width).clamp(0.0, 1.0);
    }

    if is_mouse_button_released(MouseButton::Left) {
        state.seeking = false;
    }

    draw_rectangle(
        x, y, width, SEEK_BAR_HEIGHT,
        if state.seeking { style.seek_bar_click } else if hover { style.seek_bar_hover } else { style.seek_bar }
    );

    if hover {
        draw_line(state.mouse_x, y, state.mouse_x, y + SEEK_BAR_HEIGHT, style.line_width, style.seek_hover_line);
    }

    draw_line(x + width * state.seek, y, x + width * state.seek, y + SEEK_BAR_HEIGHT, style.line_width, style.seek_line);

    state.height = y;
}

fn video(state: &State, style: &Style) {
    let bottom           = state.height;
    let center_x         = screen_width() * 0.5;
    let center_y         = bottom * 0.5;
    let available_width  = screen_width() - 2.0 * style.margin;
    let available_height = bottom - 2.0 * style.margin;
    let ratio            = 16.0 / 9.0;
    let size             = (available_width / ratio).min(available_height);
    let half_size        = size * 0.5;

    draw_rectangle(center_x - half_size * ratio, center_y - half_size, size * ratio, size, style.video);
}

