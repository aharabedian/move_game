//! Pong Tutorial 1

use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    core::transform::TransformBundle,
};

mod pong;
use crate::pong::Pong;

fn main() -> amethyst::Result<()> {
    // Setup logger
    amethyst::start_logger(Default::default());

    // Setup file paths
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let assets_dir = app_root.join("assets");

    // Setup basic application
    let game_data = GameDataBuilder::default()
        .with_bundle(RenderingBundle::<DefaultBackend>::new()
            .with_plugin(
                RenderToWindow::from_config_path(display_config_path)?
                    .with_clear([0.25, 0.05, 0.3, 1.0]),
            )
            .with_plugin(RenderFlat2D::default()),
        )?
        // Add transform bundle which tracks entity positions
        .with_bundle(TransformBundle::new())?;

    let mut game = Application::new(assets_dir, Pong, game_data)?;

    // Run game
    game.run();

    Ok(())
}