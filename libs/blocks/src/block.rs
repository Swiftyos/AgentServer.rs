

pub trait Block {
    fn run(&self, input: &str) -> Result<String, Box<dyn Error>>;
}