struct App {
    state: AppStatus,
    template_path: String,
}

#[derive(Debug)]
enum AppStatus {
    Stop,
    Active
}

impl App {
    const TEMPLATES_PATH_VAR_NAME: &str = "CREATOR_TEMPLATES";
    const DEFAULT_TEMPLATE_PATH: &str = "/Templates";

    fn new() -> App {
        let mut app = App{state: AppStatus::Stop, template_path: String::from("")};
        app.load_template_path();
        return app;
    }
    
    /// Updates the path using the environment variable App::TEMPLATES_PATH_VAR_NAME
    fn load_template_path(&mut self) {
        self.template_path = std::env::var(self::App::TEMPLATES_PATH_VAR_NAME)
            .unwrap_or_else(|_| self::App::DEFAULT_TEMPLATE_PATH.to_string());
    }

    fn greet(&self) {
        println!("State {:?}", self.state);
        println!("Template Path {}", self.template_path);
    }
    
    fn start(&mut self){
        self.state = AppStatus::Active;
        self.greet();
    }
}

fn main() {
    let mut app = App::new();
    app.start();
}
