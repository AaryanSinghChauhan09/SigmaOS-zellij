//! zellij model — terminal multiplexer (Rust).

#[derive(Debug, Clone)]
pub struct Session {
    pub name: String,
    pub tabs: Vec<Tab>,
    pub attached: bool,
}

#[derive(Debug, Clone)]
pub struct Tab {
    pub name: String,
    pub panes: Vec<Pane>,
}

#[derive(Debug, Clone)]
pub struct Pane {
    pub id: String,
    pub command: String,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub sessions: Vec<Session>,
}

impl Default for Model {
    fn default() -> Self { Self { sessions: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn new_session(&mut self, name: &str) -> bool {
        if !self.sessions.iter().any(|s| s.name == name) {
            self.sessions.push(Session { name: name.into(), tabs: vec![], attached: false });
            true
        } else { false }
    }

    pub fn new_tab(&mut self, session: &str, name: &str) -> bool {
        if let Some(s) = self.sessions.iter_mut().find(|s| s.name == session) {
            s.tabs.push(Tab { name: name.into(), panes: vec![] });
            true
        } else { false }
    }

    pub fn attach(&mut self, name: &str) -> bool {
        if let Some(s) = self.sessions.iter_mut().find(|s| s.name == name) {
            s.attached = true;
            true
        } else { false }
    }

    pub fn list(&self) -> &[Session] { &self.sessions }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn session_tab_attach() {
        let mut z = Model::new();
        assert!(z.new_session("main"));
        assert!(z.new_tab("main", "shell"));
        assert!(z.attach("main"));
    }
}
