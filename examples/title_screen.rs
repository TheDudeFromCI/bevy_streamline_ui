use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_streamline_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(StreamlineUIPlugin)
        .add_systems(Startup, init)
        .run();
}

fn init(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn(Camera3dBundle::default());
    title_screen().build(&mut commands, &asset_server);
}

fn title_screen() -> UiNode {
    UiNodeBuilder::canvas()
        .child(
            UiNodeBuilder::panel()
                .background(
                    BackgroundBuilder::image("ui/rounded_panel.png")
                        .tint(Color::rgba(0.0, 0.0, 0.0, 0.5))
                        .texture_scaling(TexScalingBuilder.stretch()),
                )
                .position(
                    PositionBuilder::anchored(AnchorPoint::CenterLeft)
                        .size(Val::Px(200.0), Val::Px(200.0))
                        .margin(Val::Px(5.0)),
                )
                .child(
                    UiNodeBuilder::text(
                        TextBuilder::default()
                            .anchor_point(AnchorPoint::Center)
                            .section(
                                TextSectionBuilder::new("Streamline\nUI")
                                    .font("fonts/FiraMono-Medium.ttf")
                                    .color(Color::WHITE)
                                    .font_size(32.0),
                            ),
                    )
                    .position(PositionBuilder::relative().size(Val::Px(300.0), Val::Auto)),
                )
                .child(
                    UiNodeBuilder::text_field(
                        TextFieldBuilder::default()
                            .font("fonts/FiraMono-Medium.ttf")
                            .placeholder_text("Enter your name"),
                    )
                    .background(BackgroundBuilder::color(Color::WHITE))
                    .position(PositionBuilder::relative().size(Val::Px(300.0), Val::Auto)),
                ),
        )
        .into()
}
