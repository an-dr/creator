use std::env;

pub struct App {
    state: AppStatus,
    template_path: String,
}

#[derive(Debug)]
enum AppStatus {
    Stop,
    Active
}

impl App {
    pub const TEMPLATES_PATH_VAR_NAME: &str = "CREATOR_TEMPLATES";
    pub const DEFAULT_TEMPLATE_PATH: &str = "D:/dev-templates/templates";

    pub fn new() -> App {
        let mut app = App{state: AppStatus::Stop, template_path: String::from("")};
        app.load_template_path();
        return app;
    }
    
    pub fn get_template_storage_path(&self) -> String {
        self.template_path.clone()
    }
    
    /// Updates the path using the environment variable App::TEMPLATES_PATH_VAR_NAME
    fn load_template_path(&mut self) {
        self.template_path = std::env::var(self::App::TEMPLATES_PATH_VAR_NAME)
            .unwrap_or_else(|_| self::App::DEFAULT_TEMPLATE_PATH.to_string());
    }

    // fn greet(&self) {
    //     let cwd = env::current_dir().expect("Failed to get current directory");
        
    //     println!("Current working directory: {}", cwd.display());
    //     println!("State {:?}", self.state);
    //     println!("Template Path {}", self.template_path);
    // }
    
    // pub fn start(&mut self){
    //     self.state = AppStatus::Active;
    //     self.greet();
    // }
}
