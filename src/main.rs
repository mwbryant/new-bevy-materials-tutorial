use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        camera::ScalingMode,
        render_resource::{AsBindGroup, ShaderRef},
    },
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
    window::PresentMode,
};

pub const CLEAR: Color = Color::rgb(0.3, 0.3, 0.3);
pub const HEIGHT: f32 = 900.0;
pub const RESOLUTION: f32 = 16.0 / 9.0;

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct CoolMaterial {
    #[uniform(0)]
    color: Color,
    #[uniform(0)]
    health: f32,
    #[texture(1)]
    #[sampler(2)]
    image: Handle<Image>,
}

impl Material2d for CoolMaterial {
    fn fragment_shader() -> ShaderRef {
        "my_material.wgsl".into()
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: HEIGHT * RESOLUTION,
                height: HEIGHT,
                title: "Bevy Material Tutorial".to_string(),
                present_mode: PresentMode::Fifo,
                resizable: false,
                ..Default::default()
            },
            ..Default::default()
        }))
        .insert_resource(ClearColor(CLEAR))
        .add_plugin(Material2dPlugin::<CoolMaterial>::default())
        .add_startup_system(spawn_camera)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut my_material_assets: ResMut<Assets<CoolMaterial>>,
    assets: Res<AssetServer>,
) {
    commands.spawn(MaterialMesh2dBundle {
        mesh: mesh_assets.add(Mesh::from(shape::Quad::default())).into(),
        material: my_material_assets.add(CoolMaterial {
            color: Color::rgb(0.2, 0.0, 0.0),
            image: assets.load("awesome.png"),
            health: 0.2,
        }),
        transform: Transform::from_xyz(-0.6, 0.0, 0.0),
        ..default()
    });
    commands.spawn(MaterialMesh2dBundle {
        mesh: mesh_assets.add(Mesh::from(shape::Quad::default())).into(),
        material: my_material_assets.add(CoolMaterial {
            color: Color::rgb(0.8, 0.0, 0.0),
            image: assets.load("awesome.png"),
            health: 0.8,
        }),
        transform: Transform::from_xyz(0.6, 0.0, 0.0),
        ..default()
    });
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.right = 1.0 * RESOLUTION;
    camera.projection.left = -1.0 * RESOLUTION;
    camera.projection.top = 1.0;
    camera.projection.bottom = -1.0;
    camera.projection.scaling_mode = ScalingMode::None;
    commands.spawn(camera);
}
