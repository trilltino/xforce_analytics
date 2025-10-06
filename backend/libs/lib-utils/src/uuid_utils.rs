use uuid::Uuid;

pub fn new_uuid() -> Uuid {
    Uuid::new_v4()
}

pub fn parse_uuid(s: &str) -> Result<Uuid, uuid::Error> {
    Uuid::parse_str(s)
}

pub fn uuid_to_string(uuid: &Uuid) -> String {
    uuid.to_string()
}
