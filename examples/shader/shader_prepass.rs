//! Bevy has an optional prepass that is controlled per-material. A prepass is a rendering pass that runs before the main pass.
//! It will optionally generate various view textures. Currently it supports depth, normal, and motion vector textures.
//! The textures are not generated for any material using alpha blending.

use bevy::{
    core_pipeline::fxaa::Fxaa,
    core_pipeline::prepass::{DepthPrepass, MotionVectorPrepass, NormalPrepass, DeferredPrepass},
    pbr::{NotShadowCaster, PbrPlugin, DefaultOpaqueRendererMethod},
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef, ShaderType},
    window::{PresentMode, WindowPlugin, WindowResolution},
};

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};


const RESOLUTION : f32 = 1024.0 * 2.0;
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(PbrPlugin {
                // The prepass is enabled by default on the StandardMaterial,
                // but you can disable it if you need to.
                //
                // prepass_enabled: false,
                ..default()
            }).set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: PresentMode::AutoNoVsync,
                    resolution: WindowResolution::new(RESOLUTION, RESOLUTION)
                        .with_scale_factor_override(1.0),
                    ..default()
                }),
                ..default()
            }),
            MaterialPlugin::<CustomMaterial>::default(),
            MaterialPlugin::<PrepassOutputMaterial> {
                // This material only needs to read the prepass textures,
                // but the meshes using it should not contribute to the prepass render, so we can disable it.
                prepass_enabled: false,
                ..default()
            },
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate, toggle_prepass_view))
        // Disabling MSAA for maximum compatibility. Shader prepass with MSAA needs GPU capability MULTISAMPLED_SHADING
        .insert_resource(Msaa::Off)
        .insert_resource(DefaultOpaqueRendererMethod::deferred())

        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    mut std_materials: ResMut<Assets<StandardMaterial>>,
    mut depth_materials: ResMut<Assets<PrepassOutputMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-12.0, 13., 15.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        // To enable the prepass you need to add the components associated with the ones you need
        // This will write the depth buffer to a texture that you can use in the main pass
        DepthPrepass,
        // This will generate a texture containing world normals (with normal maps applied)
        NormalPrepass,
        // This will generate a texture containing screen space pixel motion vectors
        MotionVectorPrepass,

        // This will generate a texture containing the output of the deferred prepass
        DeferredPrepass,

        Fxaa::default(),
    ));

    // plane
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(shape::Plane::from_size(5.0).into()),
    //     material: std_materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    //     ..default()
    // });

    // // A quad that shows the outputs of the prepass
    // // To make it easy, we just draw a big quad right in front of the camera.
    // // For a real application, this isn't ideal.
    // commands.spawn((
    //     MaterialMeshBundle {
    //         mesh: meshes.add(shape::Quad::new(Vec2::new(20.0, 20.0)).into()),
    //         material: depth_materials.add(PrepassOutputMaterial {
    //             settings: ShowPrepassSettings::default(),
    //         }),
    //         transform: Transform::from_xyz(-0.75, 1.25, 3.0)
    //             .looking_at(Vec3::new(2.0, -2.5, -5.0), Vec3::Y),
    //         ..default()
    //     },
    //     NotShadowCaster,
    // ));

    // Opaque cube
    // commands.spawn((
    //     MaterialMeshBundle {
    //         mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //         material: materials.add(CustomMaterial {
    //             color: Color::WHITE,
    //             color_texture: Some(asset_server.load("branding/icon.png")),
    //             alpha_mode: AlphaMode::Opaque,
    //         }),
    //         transform: Transform::from_xyz(-1.0, 0.5, 0.0),
    //         ..default()
    //     },
    //     Rotates,
    // ));

    // Flight Helmet
    commands.spawn((
    SceneBundle {
        transform: Transform::from_xyz(-1.0, 1.0, 0.0).with_scale(Vec3::ONE * 2.0),
        scene: asset_server.load("models/objaverse/002b5dcd1a7844b19c7ffa63a9b23c68.glb#Scene0"),
        ..default()
    },
    Rotates,
    ));

    // // Cube with alpha mask
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::UVSphere { radius: 0.5, sectors: 36, stacks: 18 })),
    //     material: std_materials.add(StandardMaterial {
    //         alpha_mode: AlphaMode::Mask(1.0),
    //         base_color_texture: Some(asset_server.load("branding/icon.png")),
    //         ..default()
    //     }),
    //     transform: Transform::from_xyz(0.0, 0.5, 0.0),
    //     ..default()
    // });

    // // Cube with alpha blending.
    // // Transparent materials are ignored by the prepass
    // commands.spawn(MaterialMeshBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //     material: materials.add(CustomMaterial {
    //         color: Color::WHITE,
    //         color_texture: Some(asset_server.load("branding/icon.png")),
    //         alpha_mode: AlphaMode::Blend,
    //     }),
    //     transform: Transform::from_xyz(1.0, 0.5, 0.0),
    //     ..default()
    // });





    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    let style = TextStyle {
        font_size: 18.0,
        ..default()
    };

    commands.spawn(
        TextBundle::from_sections(vec![
            TextSection::new("Prepass Output: transparent\n", style.clone()),
            TextSection::new("\n\n", style.clone()),
            TextSection::new("Controls\n", style.clone()),
            TextSection::new("---------------\n", style.clone()),
            TextSection::new("Space - Change output\n", style),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
    );
}

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {
    #[uniform(0)]
    color: Color,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
    alpha_mode: AlphaMode,
}

/// Not shown in this example, but if you need to specialize your material, the specialize
/// function will also be used by the prepass
impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }

    // You can override the default shaders used in the prepass if your material does
    // anything not supported by the default prepass
    // fn prepass_fragment_shader() -> ShaderRef {
    //     "shaders/custom_material.wgsl".into()
    // }
}

#[derive(Component)]
struct Rotates;

fn rotate(mut q: Query<&mut Transform, With<Rotates>>, time: Res<Time>) {
    for mut t in q.iter_mut() {
        let rot = (time.elapsed_seconds().sin() * 0.5 + 0.5) * std::f32::consts::PI * 2.0;
        t.rotation = Quat::from_rotation_y(rot);
    }
}

#[derive(Debug, Clone, Default, ShaderType)]
struct ShowPrepassSettings {
    show_depth: u32,
    show_normals: u32,
    show_motion_vectors: u32,
    show_deferred_data: u32,
    prepass_view: u32,
}

// This shader simply loads the prepass texture and outputs it directly
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct PrepassOutputMaterial {
    #[uniform(0)]
    settings: ShowPrepassSettings,
}

impl Material for PrepassOutputMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/show_prepass.wgsl".into()
    }

    // This needs to be transparent in order to show the scene behind the mesh
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

/// Every time you press space, it will cycle between transparent, depth and normals view
fn toggle_prepass_view(
    mut prepass_view: Local<u32>,
    keycode: Res<Input<KeyCode>>,
    material_handle: Query<&Handle<PrepassOutputMaterial>>,
    mut materials: ResMut<Assets<PrepassOutputMaterial>>,
    mut text: Query<&mut Text>,
) {
    let mut dirty = false;
    if keycode.just_pressed(KeyCode::R) {
        *prepass_view = 0;
        dirty = true;
    }

    if keycode.just_pressed(KeyCode::Space) {
        *prepass_view += 1;
        
        dirty  = true;

    }

    if keycode.just_pressed(KeyCode::Back) {
        *prepass_view -= 1;
        
        dirty  = true;

    }

    if dirty {
    
    let prepass_view = *prepass_view; 
    let label = match prepass_view {
        0 => "combined",
        1 => "depth",
        2 => "normals",
        3 => "motion vectors",
        _ => "deferred",
    };
    let mut text = text.single_mut();
    text.sections[0].value = format!("Prepass {prepass_view} Output: {label}\n");
    for section in &mut text.sections {
        section.style.color = Color::WHITE;
    }

    let handle = material_handle.single();
    let mat = materials.get_mut(handle).unwrap();
    mat.settings.show_depth = (prepass_view == 1) as u32;
    mat.settings.show_normals = (prepass_view == 2) as u32;
    mat.settings.show_motion_vectors = (prepass_view == 3) as u32;
    mat.settings.show_deferred_data = (prepass_view >= 4) as u32;
    mat.settings.prepass_view = prepass_view;
    }
}
