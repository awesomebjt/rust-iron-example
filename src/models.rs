
#[derive(RustcDecodable, RustcEncodable)]
pub struct Task {
    pub name: String,
    pub task: String
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Tasks {
    pub tasks: Vec<Task>
}
