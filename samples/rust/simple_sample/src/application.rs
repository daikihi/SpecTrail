/// [@ss-simple-sample] layer: implementation, type: Structure, name: Simple Sample
/// [@ss-simple-sample-application] layer: implementation, type: Structure, name: Application
/// [@ss-simple-sample-requirement-4] layer: implementation, type: Structure, name: Requirement 4
/// [@ss-simple-sample-structure-application] layer: implementation, type: Structure, name: Application Structure

use crate::domain;

pub fn run(name: String) -> Result<String, String> {
    let name = domain::validate_name(name)?;
    Ok(domain::greet(&name))
}
