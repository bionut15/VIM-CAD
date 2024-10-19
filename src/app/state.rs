use bevy::prelude::States;
// add code here
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum StateApp {
    #[default]
    Normal,
    Visual,
    Insert,
    Replace,
}
use StateApp::*;
#[derive(Debug)]
pub enum Transition {
    NormalT,
    VisualT,
    InsertT,
    ReplaceT,
}
use Transition::*;

pub struct MainWindow {
    pub state: StateApp,
}

impl MainWindow {
    pub fn new() -> Self {
        Self { state: Normal }
    }
    pub fn change(&mut self, mode: Transition) {
        match (&self.state, mode) {
            (Normal, NormalT) => self.state = Normal,
            (_, VisualT) => self.state = Visual,
            (_, InsertT) => self.state = Insert,
            (_, ReplaceT) => self.state = Replace,
            (Visual, NormalT) | (Insert, NormalT) | (Replace, NormalT) => self.state = Normal,
        }
    }
}
