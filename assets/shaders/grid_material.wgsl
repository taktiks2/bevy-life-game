#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct GridUniforms {
    camera_scale: f32,
    grid_visible: f32,
    chunk_cells: f32,
    grid_line_width: f32,
    grid_color: vec4<f32>,
}

@group(2) @binding(0) var cell_texture: texture_2d<f32>;
@group(2) @binding(1) var cell_sampler: sampler;
@group(2) @binding(2) var<uniform> uniforms: GridUniforms;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    // セルテクスチャをサンプリング
    let cell_color = textureSample(cell_texture, cell_sampler, mesh.uv);

    // グリッド非表示なら早期リターン
    if uniforms.grid_visible < 0.5 {
        return cell_color;
    }

    // UV座標をセル単位に変換 (0..chunk_cells の範囲)
    let cell_coord = mesh.uv * uniforms.chunk_cells;

    // セル境界までの距離（セル単位、0..0.5の範囲）
    let cell_fract = fract(cell_coord);
    let dist_to_edge = min(cell_fract, 1.0 - cell_fract);

    // UV空間での1スクリーンピクセルの大きさ
    // chunk_world_size = chunk_cells (cell_world_size=1.0のため)
    // スクリーン上の1ピクセル = camera_scale ワールド単位
    // セル単位での1スクリーンピクセル = camera_scale / cell_world_size = camera_scale
    let pixel_in_cells = uniforms.camera_scale;

    // グリッド線幅（セル単位）
    let line_half_width = uniforms.grid_line_width * pixel_in_cells * 0.5;

    // アンチエイリアシング付きグリッド線判定
    let aa_width = pixel_in_cells * 0.5;
    let grid_x = smoothstep(line_half_width + aa_width, line_half_width, dist_to_edge.x);
    let grid_y = smoothstep(line_half_width + aa_width, line_half_width, dist_to_edge.y);
    let grid_factor = max(grid_x, grid_y);

    // セルのスクリーンピクセルサイズ（cell_world_size=1.0のため 1/camera_scale）
    let cell_screen_pixels = 1.0 / uniforms.camera_scale;
    // セルが小さすぎる場合はグリッド線をフェードアウト
    let fade = smoothstep(2.0, 6.0, cell_screen_pixels);

    return mix(cell_color, uniforms.grid_color, grid_factor * fade);
}
