use rand::Rng;
use terrain::generator::TerrainGenerator;
use terrain::model2d::{builder::TerrainModel2DBulider, sites::Site2D};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_terrain_generator(canvas: web_sys::HtmlCanvasElement, imgwidth: u32, imgheight: u32) {
    let num = 10000;
    let bound_min = Site2D { x: 0.0, y: 0.0 };
    let bound_max = Site2D {
        x: 200.0 * 1e3,
        y: 200.0 * 1e3 * (imgheight as f64/imgwidth as f64)
    };

    let mut sites = Vec::with_capacity(num);
    let mut rng = rand::thread_rng();

    for _ in 0..num {
        let x = rng.gen_range(bound_min.x..bound_max.x);
        let y = rng.gen_range(bound_min.y..bound_max.y);
        sites.push(Site2D { x, y });
    }

    let model = TerrainModel2DBulider::default()
        .set_sites(sites)
        .set_bounding_box(Some(bound_min), Some(bound_max))
        .unwrap()
        .iterate_sites(1)
        .unwrap()
        .build()
        .unwrap();

    let terrain = TerrainGenerator::default()
        .set_model(model)
        .set_uplift_rate(1e-4 * 5.0)
        .set_erodibility(1e-7 * 5.61)
        .set_exponent_m(0.5)
        .generate()
        .unwrap();

    let sites = terrain.sites;
    let altitudes = terrain.altitudes;

    let image = terrain_visualizer::Visualizer::new(
        sites
            .iter()
            .enumerate()
            .map(|(i, n)| (terrain_visualizer::Site { x: n.x, y: n.y }, altitudes[i]))
            .collect::<Vec<(terrain_visualizer::Site, f64)>>(),
    )
    .set_x_range(bound_min.x, bound_max.x)
    .set_y_range(bound_min.y, bound_max.y);

    let buffer = image
        .render_image(Some(imgwidth), Some(imgheight), |weight_rate: f64| {
            let c = (weight_rate * 220.0 + 30.0) as u8;
            image::Rgb([c, c, c])
        })
        .unwrap();

    let (width, height) = buffer.dimensions();
    let mut u8_buffer = Vec::with_capacity((width * height * 4) as usize);
    for pixel in buffer.pixels() {
        u8_buffer.extend_from_slice(&[pixel[0], pixel[1], pixel[2], 255]);
    }
    let data = wasm_bindgen::Clamped(u8_buffer);

    // CanvasRenderingContext2d を取得
    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    // ImageData を作成
    let image_data =
        web_sys::ImageData::new_with_u8_clamped_array_and_sh(wasm_bindgen::Clamped(&data), width, height).unwrap();

    // Canvas に描画
    ctx.put_image_data(&image_data, 0.0, 0.0).unwrap();
}
