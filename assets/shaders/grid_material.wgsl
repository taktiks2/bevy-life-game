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

    // fwidth: フラグメント間のcell_coordの変化量 = 1スクリーンピクセルあたりのセル単位サイズ
    let fw = fwidth(cell_coord);

    // セル境界までの距離（セル単位、0..0.5の範囲）
    let cell_fract = fract(cell_coord);
    let dist_to_edge = min(cell_fract, 1.0 - cell_fract);

    // アンチエイリアシング: AA幅は常に1ピクセル分（均一な描画を保証）
    let grid_x = smoothstep(fw.x, 0.0, dist_to_edge.x);
    let grid_y = smoothstep(fw.y, 0.0, dist_to_edge.y);
    let grid_factor = max(grid_x, grid_y);

    // grid_line_width で線の強度（不透明度）を制御
    // 値が小さいほど薄く細く見える
    let intensity = clamp(uniforms.grid_line_width, 0.0, 1.0);

    // セルのスクリーンピクセルサイズ（fwidthの逆数）
    let cell_screen_pixels = 1.0 / max(fw.x, fw.y);
    // セルが小さすぎる場合はグリッド線をフェードアウト
    let fade = smoothstep(2.0, 6.0, cell_screen_pixels);

    return mix(cell_color, uniforms.grid_color, grid_factor * intensity * fade);
}
