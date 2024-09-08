use bevy::prelude::*;

use bevy_round_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, RoundUiPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<RoundUiMaterial>>,
//    asset_server: Res<AssetServer>,
    mut windows: Query<&mut Window>,
) {
    let window = windows.single_mut();

    // Camera so we can see UI
    commands.spawn(Camera2dBundle::default());

    let panel_width = 200.0;
    let panel_height = 200.0;
    //let panel_width = window.width();
    //let panel_height = window.height();

    // Add the material
    let panel_material = materials.add(RoundUiMaterial {
        background_color: Color::hex("#F76161").unwrap(),
        border_color: Color::hex("#A53A3D").unwrap(),
        border_radius: RoundUiBorder::all(20.0).into(),
        size: Vec2::new(panel_width, panel_height),
        offset: RoundUiOffset::bottom(10.0).into(),
		power: 1f32,
		time: 0f32,
		resolution: Vec2::new(window.width(), window.height()),
		value: 0f32,
		/*
        texture0: Some(asset_server.load("textures/tex0.png")),
        texture1: Some(asset_server.load("textures/tex1.png")),
        texture2: Some(asset_server.load("textures/tex2.png")),
		*/
    });

    // Spawn the material in the middle of the screen
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|p| {
            p.spawn(MaterialNodeBundle {
                material: panel_material.clone(),
                style: Style {
                    width: Val::Px(panel_width),
                    height: Val::Px(panel_height),
                    ..default()
                },
                ..default()
            });
            p.spawn(MaterialNodeBundle {
                material: panel_material.clone(),
                style: Style {
                    width: Val::Px(panel_width / 2.0),
                    height: Val::Px(panel_height / 2.0),
                    ..default()
                },
                ..default()
            });
            p.spawn(MaterialNodeBundle {
                material: panel_material.clone(),
                style: Style {
                    width: Val::Px(panel_width),
                    height: Val::Px(panel_height),
                    ..default()
                },
                ..default()
            });
        });

    // Spawn a progress bar
	let bar_width = 600.0;
	let bar_height = 10.0;
    let bar_material = materials.add(RoundUiMaterial {
        background_color: Color::hex("#F76161").unwrap(),
        border_color: Color::hex("#A53A3D").unwrap(),
        border_radius: RoundUiBorder::all(0.0).into(),
        size: Vec2::new(bar_width, bar_height),
        offset: RoundUiOffset::all(1.0).into(),
		power: 1f32,
		time: 0f32,
		resolution: Vec2::new(window.width(), window.height()),
		value: 0f32,
    });
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
				//margin: UiRect::all(Val::Px(50.)),
                align_items: AlignItems::End,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|p| {
            p.spawn(MaterialNodeBundle {
                material: bar_material.clone(),
                style: Style {
                    width: Val::Px(bar_width),
                    height: Val::Px(bar_height),
					flex_direction: FlexDirection::Column,
					align_self: AlignSelf::End,
					//position_type: PositionType::Absolute,
					align_items: AlignItems::Center,
					justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            })
			.with_children(|parent| {
				parent.spawn(
					TextBundle::from_section(
						"0.00%",
						TextStyle {
							color: Color::BLACK,
							font_size: 10.,
							..default()
						},
					)
				);
			});
        });
}
