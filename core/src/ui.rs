use uuid::Uuid;

use crate::game::ButteryGame;

#[derive(Default)]
pub struct ButteryUIModel<G: ButteryGame> {
    pub windows: Vec<ButteryUIWindow<G>>,
}

pub struct ButteryUIWindow<G: ButteryGame> {
    pub id: String,
    pub relative_position: ButteryUIWindowRelativePosition,
    pub offset: ButterUI2D,
    pub corner_radius: f32,
    pub padding: i8,
    pub max_width: f32,
    pub max_height: f32,
    pub background_color: ButteryUIColor,
    pub child: ButteryUIElement<G>,
}

impl<G: ButteryGame> Default for ButteryUIWindow<G> {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            relative_position: Default::default(),
            offset: Default::default(),
            corner_radius: 20.0,
            padding: 16,
            max_width: 400.0,
            max_height: 200.0,
            background_color: Default::default(),
            child: Default::default(),
        }
    }
}

#[derive(Default, Clone)]
pub enum ButteryUIWindowRelativePosition {
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    #[default]
    Centered,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

#[derive(Default)]
pub struct ButterUI2D {
    pub x: f32,
    pub y: f32,
}

#[derive(Default)]
pub enum ButteryUIElement<G: ButteryGame> {
    #[default]
    Default,
    Text(ButteryUIText),
    Column(ButteryUIDirectional<G>),
    Row(ButteryUIDirectional<G>),
    Button(ButteryUIButton<G>),
    Input(ButteryUIInput<G>),
    Container(ButteryUIContainer<G>),
}

pub struct ButteryUIColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Default for ButteryUIColor {
    fn default() -> Self {
        Self {
            r: Default::default(),
            g: Default::default(),
            b: Default::default(),
            a: 255,
        }
    }
}

#[derive(Default)]
pub struct ButteryUIText {
    pub text: String,
    pub size: Option<ButterUI2D>,
}

pub struct ButteryUIDirectional<G: ButteryGame> {
    pub children: Vec<ButteryUIElement<G>>,
    pub centered: bool,
    pub size: Option<ButterUI2D>,
}

pub struct ButteryUIInput<G: ButteryGame> {
    pub on_changed: ButteryUIInputChangedCallback<G>,
    pub current_value: String,
    pub background_color: Option<ButteryUIColor>,
    pub size: Option<ButterUI2D>,
}
pub type ButteryUIInputChangedCallback<G> = fn(String, &mut G);

impl<G: ButteryGame> Default for ButteryUIInput<G> {
    fn default() -> Self {
        Self {
            background_color: None,
            current_value: String::new(),
            on_changed: |_, _| {},
            size: None,
        }
    }
}

pub struct ButteryUIButton<G: ButteryGame> {
    pub label: String,
    pub on_click: ButteryUIButtonCallback<G>,
    pub width: f32,
    pub height: f32,
    pub corner_radius: f32,
}

impl<G: ButteryGame> Default for ButteryUIButton<G> {
    fn default() -> Self {
        Self {
            label: String::default(),
            on_click: |_| {},
            width: 100.0,
            height: 40.0,
            corner_radius: 20.0,
        }
    }
}

pub type ButteryUIButtonCallback<G> = fn(&mut G);

#[derive(Default)]
pub struct ButteryUIContainer<G: ButteryGame> {
    pub size: Option<ButterUI2D>,
    pub children: Vec<ButteryUIElement<G>>,
    pub color: ButteryUIColor,
    pub corner_radius: f32,
    pub outline: Option<ButteryUIContainerOutline>,
}

pub struct ButteryUIContainerOutline {
    pub width: f32,
    pub color: ButteryUIColor,
}
