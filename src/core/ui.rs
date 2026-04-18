#[derive(Default)]
pub struct ButteryUIModel {
    pub windows: Vec<ButteryUIWindow>,
}

#[derive(Default)]
pub struct ButteryUIWindow {
    pub relative_position: ButteryUIWindowRelativePosition,
    pub offset: ButterUIWindowOffset,
    pub corner_radius: f32,
    pub inner_margin: i8,
    pub max_width: f32,
    pub max_height: f32,
    pub background_color: ButteryUIColor,
    pub child: ButteryUIElement,
}

#[derive(Default, Clone)]
pub enum ButteryUIWindowRelativePosition {
    #[default]
    Centered,
}

#[derive(Default)]
pub struct ButterUIWindowOffset {
    pub x: f32,
    pub y: f32,
}

#[derive(Default)]
pub enum ButteryUIElement {
    #[default]
    Default,
    Text(String),
    Column(Vec<ButteryUIElement>),
}

#[derive(Default)]
pub struct ButteryUIColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
