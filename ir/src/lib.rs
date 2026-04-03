#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Project {
    pub project_name: String,
    pub screens: Vec<Screen>,
}

impl Project {
    pub fn new(project_name: impl Into<String>) -> Self {
        Self {
            project_name: project_name.into(),
            screens: Vec::new(),
        }
    }

    pub fn with_screen(mut self, screen: Screen) -> Self {
        self.screens.push(screen);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Screen {
    pub name: String,
    pub root: Widget,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Widget {
    pub kind: WidgetKind,
    pub id: Option<String>,
    pub text: Option<String>,
    pub children: Vec<Widget>,
}

impl Widget {
    pub fn new(kind: WidgetKind) -> Self {
        Self {
            kind,
            id: None,
            text: None,
            children: Vec::new(),
        }
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn with_child(mut self, child: Widget) -> Self {
        self.children.push(child);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WidgetKind {
    Container,
    Row,
    Column,
    Grid,
    Text,
    Button,
    Image,
    Card,
}
