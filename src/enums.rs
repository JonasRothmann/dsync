use crate::Error;
use crate::GenerationConfig;

#[cfg(feature = "enums")]
pub fn generate_enums(opts: &GenerationConfig) -> Result<String, Error> {
    use std::{collections::HashMap, fs::File};

    use inflector::Inflector;
    use postgres::{Client, NoTls};

    // Connect to the PostgreSQL database
    let mut client = Client::connect(opts.database_url.as_str(), NoTls)?;

    // Query to get all the ENUM types in the database
    let query = "
        SELECT t.typname AS enumtype, 
               array_agg(e.enumlabel ORDER BY e.enumsortorder) AS enumlabels
        FROM pg_type t 
        JOIN pg_enum e ON t.oid = e.enumtypid  
        GROUP BY enumtype;";

    // Prepare a map to store the ENUM name and its values
    let mut enums: HashMap<String, Vec<String>> = HashMap::new();

    // Execute the query
    for row in client.query(query, &[])? {
        let enum_type: String = row.get("enumtype");
        let enum_labels: Vec<String> = row.get("enumlabels");
        enums.insert(enum_type, enum_labels);
    }

    let mut lines: Vec<String> = Vec::new();

    lines.push("use serde::{Serialize, Deserialize};\n".to_string());

    // Generate Rust ENUMs
    for (enum_name, enum_values) in &enums {
        lines.push("#[derive(Debug, Clone, Serialize, Deserialize, diesel_derive_enum::DbEnum, juniper::GraphQLEnum)]".to_string());
        lines.push(format!(
            "#[ExistingTypePath = \"{schema_path}sql_types::{}\"]",
            enum_name,
            schema_path = opts.schema_path
        ));
        lines.push("#[DbValueStyle = \"SCREAMING_SNAKE_CASE\"]".to_string());
        lines.push(format!("pub enum {} {{", enum_name));
        for value in enum_values {
            lines.push(format!("    {},", value.to_string().to_pascal_case()));
        }
        //buffer.push_str(enum_values.join(",\n").as_str());
        lines.push("}\n".to_string());
    }

    Ok(lines.join("\n"))
}
