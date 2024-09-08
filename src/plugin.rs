use bevy::{asset::load_internal_asset, prelude::*};

use crate::prelude::*;

#[cfg(feature = "autosize")]
use crate::autosize::RoundUiAutosizePlugin;

/// A plugin which adds a round rect UI material to the app
pub struct RoundUiPlugin;

impl Plugin for RoundUiPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(app, SHADER_HANDLE, "shader.wgsl", Shader::from_wgsl);

        app.add_plugins(UiMaterialPlugin::<RoundUiMaterial>::default());

        #[cfg(feature = "autosize")]
        app.add_plugins(RoundUiAutosizePlugin);

		app.add_systems(Update, timer_update);
    }
}

fn timer_update(
    mut materials: ResMut<Assets<RoundUiMaterial>>,
//    mut windows: Query<&mut Window>,
	time: Res<Time>,
) {
//    let window = windows.single_mut();
    for (_, material) in materials.iter_mut() {
		material.time = time.elapsed_seconds();
//		material.resolution = Vec2::new(window.width(), window.height());
    }
}
