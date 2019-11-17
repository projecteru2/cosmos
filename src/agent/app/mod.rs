use super::CosmosApp;
pub struct App {}

static mut APP: Option<App> = None;

impl App {
    pub fn get() -> &'static Self {
        unsafe {
            match APP.as_ref() {
                None => Self::init(),
                _ => (),
            };
            APP.as_ref().unwrap()
        }
    }

    fn init() {
        unsafe {
            APP = Some(Self::new());
        }
    }

    fn new() -> Self {
        App {}
    }
}

impl CosmosApp for App {
    fn version(&self) -> String {
        "2019-11-04".to_string()
    }
}
