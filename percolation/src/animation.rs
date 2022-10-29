use crate::{
    bindings::{draw_rectangle, draw_text, set_bottom_text},
    percolation::{Percolatable, Percolation},
    percolationstats::PercolationStats,
};

pub const TILE_TIME: u8 = 1;

const TILE_SIZE: f64 = 15.0;
const BORDER_THICKNESS: f64 = 2.0;
const MAX_WIDTH: f64 = 1200.;
const MAX_HEIGHT: f64 = 800.;

pub const GRAPH_WIDTH: f64 = 500.0; // total canvas size
pub const GRAPH_HEIGHT: f64 = 500.0; // total canvas size
const GRAPH_BOTTOM: f64 = 50.0;
const GRAPH_LEFT: f64 = 50.0;
const LINE_WEIGHT: f64 = 3.0;
const TICK_LEN: f64 = 20.0;
const BLACK: &str = "#000000";
const FONT: &str = "bold 10px sans-serif";
const FONT_HEIGHT: f64 = 10.0;

fn real_sizes(width: usize, height: usize) -> (f64, f64) {
    let ts = (MAX_WIDTH / width as f64)
        .min(MAX_HEIGHT / height as f64)
        .min(TILE_SIZE);
    (ts, BORDER_THICKNESS * (ts / TILE_SIZE))
}

pub fn convert(x: f64, y: f64, width: usize, height: usize) -> Option<(usize, usize)> {
    let (tile_size, border_thickness) = real_sizes(width, height);
    let new_row = (y - border_thickness / 2.) / (tile_size + border_thickness);
    let new_col = (x - border_thickness / 2.) / (tile_size + border_thickness);
    if new_row < 0. || new_col < 0. {
        return None;
    }

    let (row, col) = (new_row as usize, new_col as usize);
    if row >= height || col >= width {
        return None;
    }
    Some((row, col))
}

pub fn canvas_size(width: usize, height: usize) -> (f64, f64) {
    let (tile_size, border_thickness) = real_sizes(width, height);
    (
        (width as f64) * (tile_size + border_thickness) + border_thickness,
        (height as f64) * (tile_size + border_thickness) + border_thickness,
    )
}

pub fn draw_percolation(p: &Percolation) {
    let (tile_size, border_thickness) = real_sizes(p.width(), p.height());
    let (canvas_width, canvas_height) = canvas_size(p.width(), p.height());
    draw_rectangle(0., 0., canvas_width, canvas_height, "#000000");
    for row in 0..p.height() {
        for col in 0..p.width() {
            let mut color = "#000000";
            if p.is_open(row, col).expect("asd") {
                color = "#ffffff";
            }
            if p.is_full(row, col).expect("asd") {
                color = "#67c6f3";
            }
            let x = (col as f64) * (tile_size + border_thickness) + border_thickness;
            let y = (row as f64) * (tile_size + border_thickness) + border_thickness;
            draw_rectangle(x, y, tile_size, tile_size, color);
        }
    }
    set_bottom_text(&format!(
        "Open sites: {}, Percolates: {}",
        p.number_of_open_sites(),
        p.percolates()
    ));
}

pub fn draw_graph(stats: &PercolationStats) {
    draw_rectangle(
        GRAPH_LEFT - LINE_WEIGHT,
        GRAPH_HEIGHT - GRAPH_BOTTOM,
        GRAPH_WIDTH - GRAPH_LEFT + LINE_WEIGHT,
        LINE_WEIGHT,
        BLACK,
    );
    draw_rectangle(
        GRAPH_LEFT - LINE_WEIGHT,
        0.,
        LINE_WEIGHT,
        GRAPH_HEIGHT - GRAPH_BOTTOM,
        BLACK,
    );

    let num_ticks: usize = 10;
    for i in 0..(num_ticks + 1) {
        let ratio = i as f64 / num_ticks as f64;
        let x_pos = GRAPH_LEFT + ratio * (GRAPH_HEIGHT - GRAPH_LEFT) - LINE_WEIGHT;
        let y_pos = (1.0 - ratio) * (GRAPH_HEIGHT - GRAPH_BOTTOM);
        draw_rectangle(
            x_pos,
            GRAPH_HEIGHT - GRAPH_BOTTOM,
            LINE_WEIGHT,
            TICK_LEN,
            BLACK,
        );
        draw_text(
            &ratio.to_string(),
            x_pos,
            GRAPH_HEIGHT - (GRAPH_BOTTOM - TICK_LEN) / 2.,
            BLACK,
            FONT,
        );
        draw_rectangle(GRAPH_LEFT - TICK_LEN, y_pos, TICK_LEN, LINE_WEIGHT, BLACK);
        draw_text(
            &ratio.to_string(),
            (GRAPH_LEFT - TICK_LEN) / 2.,
            y_pos + FONT_HEIGHT / 2.,
            BLACK,
            FONT,
        );
    }

    let total: usize = stats.counts.iter().sum();
    let point_width = 1.0 / (stats.counts.len() as f64);
    let mut running: usize = 0;
    for (i, num_at) in stats.counts.iter().enumerate() {
        running += *num_at;
        let proportion = running as f64 / total as f64;

        draw_rectangle(
            GRAPH_LEFT + (i as f64 * point_width) * (GRAPH_HEIGHT - GRAPH_LEFT),
            (1.0 - proportion) * (GRAPH_HEIGHT - GRAPH_BOTTOM - LINE_WEIGHT),
            LINE_WEIGHT,
            LINE_WEIGHT,
            BLACK,
        )
    }

    set_bottom_text(&stats.to_string());
}
