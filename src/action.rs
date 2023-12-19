pub struct Action {
    pub name: String,
    pub function: Box<dyn Fn() + Send + Sync>,
}
