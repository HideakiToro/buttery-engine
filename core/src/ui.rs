use crate::game::ButteryGame;

#[derive(Default)]
pub struct ButteryUIModel<G: ButteryGame> {
    pub windows: Vec<ButteryUIWindow<G>>,
}

#[derive(Default)]
pub struct ButteryUIWindow<G: ButteryGame> {
    pub relative_position: ButteryUIWindowRelativePosition,
    pub offset: ButterUIWindowOffset,
    pub corner_radius: f32,
    pub inner_margin: i8,
    pub max_width: f32,
    pub max_height: f32,
    pub background_color: ButteryUIColor,
    pub child: ButteryUIElement<G>,
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
pub enum ButteryUIElement<G: ButteryGame> {
    #[default]
    Default,
    Text(String),
    Column(Vec<ButteryUIElement<G>>),
    Button(ButteryUIButton<G>),
}

#[derive(Default)]
pub struct ButteryUIColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub struct ButteryUIButton<G: ButteryGame> {
    pub label: String,
    pub on_click: ButteryUIButtonCallback<G>,
}

pub type ButteryUIButtonCallback<G> = fn(&mut G);
