use bevy::{prelude::*, reflect::TypePath, render::render_resource::*};

use crate::prelude::{RoundUiBorder, RoundUiOffset};

pub const SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(66552904175742639684);

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct RoundUiMaterial {
    /// The background color of the material
    #[uniform(0)]
    pub background_color: Color,

    /// The border color of the material
    #[uniform(0)]
    pub border_color: Color,

    /// The border radius of each corner
    /// E.g. Vec4::new(bottom_right, top_right, bottom_left, top_left)
    #[uniform(0)]
    pub border_radius: Vec4,

    /// The border offset along each side of the rect
    /// E.g. Vec4::new((top, left, bottom, right)
    #[uniform(0)]
    pub offset: Vec4,

    /// The size of the material on screen in pixels
    #[uniform(0)]
    pub size: Vec2,

    /// The size of the material on screen in pixels
    #[uniform(0)]
    pub power: f32,

    /// Time
    #[uniform(0)]
    pub time: f32,

    /// Screen resolution
    #[uniform(0)]
    pub resolution: Vec2,

    /// Value if used as progress bar
    #[uniform(0)]
    pub value: f32,
	/*

	#[texture(1)]
	#[sampler(2)]
	pub texture0: Option<Handle<Image>>,
	#[texture(3)]
	#[sampler(4)]
	pub texture1: Option<Handle<Image>>,
	#[texture(5)]
	#[sampler(6)]
	pub texture2: Option<Handle<Image>>,
	*/
}

impl Default for RoundUiMaterial {
    fn default() -> Self {
        Self {
            background_color: Color::WHITE,
            border_color: Color::WHITE,
            border_radius: Vec4::splat(0.),
            size: Vec2::new(1., 1.),
            offset: Vec4::splat(0.),

			power: 1f32,
			time: 0f32,
			resolution: Vec2::new(1024.0, 768.0),
			value: 1f32,
			/*
			texture0: None,
			texture1: None,
			texture2: None,
			*/
        }
    }
}

impl UiMaterial for RoundUiMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_HANDLE.into()
    }
}

impl RoundUiMaterial {
    pub fn get_padding(&self) -> UiRect {
        let offset: RoundUiOffset = self.offset.into();
        let border: RoundUiBorder = self.border_radius.into();
        UiRect {
            left: Val::Px(offset.left + border.top_left.max(border.bottom_left)),
            right: Val::Px(offset.right + border.top_right.max(border.bottom_right)),
            top: Val::Px(offset.top + border.top_left.max(border.top_right)),
            bottom: Val::Px(offset.bottom + border.bottom_left.max(border.bottom_right)),
        }
    }
}
