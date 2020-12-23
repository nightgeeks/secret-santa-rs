#[derive(Clone)]
pub struct Participant {
    pub id: u32,
    pub name: String,
    pub email: String,
    // TODO - add set of groups ^_^
    pub group_id: Option<u32>,
}
