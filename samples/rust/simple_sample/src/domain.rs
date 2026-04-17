/// [@ss-simple-sample] layer: implementation, type: Structure, name: Simple Sample
/// [@ss-simple-sample-domain] layer: implementation, type: Structure, name: Domain
/// [@ss-simple-sample-requirement-2] layer: implementation, type: Structure, name: Requirement 2
/// [@ss-simple-sample-requirement-3] layer: implementation, type: Structure, name: Requirement 3
/// [@ss-simple-sample-structure-domain] layer: implementation, type: Structure, name: Domain Structure

pub fn validate_name(name: String) -> Result<String, String> {
    let trimmed = name.trim().to_owned();
    if trimmed.is_empty() {
        Err(String::from("name must not be empty"))
    } else {
        Ok(trimmed)
    }
}

pub fn greet(name: &str) -> String {
    format!("Hello, {name}")
}
