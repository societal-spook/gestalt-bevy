#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings

struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    @builtin(position) frag_coord: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
};

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {

    let x = in.uv.x % 0.02;
    let y = in.uv.y % 0.02;

    if (x<0.001||y<0.001) {
        return vec4(0.1,0.1,0.1,1.0);
    } else {
        return vec4(0.4,0.4,0.4,1.0);
    }
    return vec4(in.uv, 0.0, 1.0);
}
