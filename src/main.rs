mod construct;
mod core;
mod game;

use amethyst::{
    controls::FlyControlBundle,
    core::TransformBundle,
    input::{InputBundle, StringBindings},
    renderer::{
        plugins::{RenderPbr3D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::{application_root_dir, auto_fov::AutoFovSystem},
    window::DisplayConfig,
    Application, GameDataBuilder,
};

fn main() -> amethyst::Result<()> {
    // Set up the Amethyst logger
    amethyst::start_logger(Default::default());

    // Set up the assets directory (PathBuf)
    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets");

    // Set up the display configuration
    let display_config = DisplayConfig {
        title: "Natural Gravity Engine".to_string(),
        dimensions: Some((800, 600)), // 4:3
        ..Default::default()
    };

    let key_bindings_path = app_root.join("config/input.ron");

    // Set up the GameDataBuilder
    let game_data = GameDataBuilder::default()
        .with_bundle(
            FlyControlBundle::<StringBindings>::new(
                Some(String::from("move_x")),
                Some(String::from("move_y")),
                Some(String::from("move_z")),
            )
            .with_sensitivity(0.1, 0.1)
            .with_speed(8.5),
        )?
        .with_bundle(TransformBundle::new().with_dep(&["fly_movement"]))?
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(&key_bindings_path)?,
        )?
        .with(AutoFovSystem::new(), "auto_fov", &[])
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with(
            core::headlamp::HeadlampSystem::default(),
            "headlamp_system",
            &["input_system"],
        )
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
                .with_plugin(
                    RenderToWindow::from_config(display_config).with_clear([0., 0., 0., 1.0]),
                )
                .with_plugin(RenderUi::default())
                .with_plugin(RenderPbr3D::default()),
        )?;

    // Run the game!
    let mut game = Application::new(assets_dir, game::GameState::default(), game_data)?;
    game.run();

    Ok(())
}
