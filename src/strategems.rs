use thiserror::Error;
use yaml_rust::Yaml;

use crate::code::{Code, CodeError};

#[derive(Error, Debug, Clone)]
pub enum StrategemError {
    #[error("Strategem does not have a name.")]
    MissingName,
    #[error("Strategem does not have a code.")]
    MissingCode,
    #[error("Strategem code contains invalid characters: `{0}`")]
    InvalidCode(#[from] CodeError),
}

#[derive(Debug, Clone)]
pub struct Strategem {
    pub name: String,
    pub code: Code,
}

impl Strategem {
    pub fn from_yaml(yaml: &Yaml) -> Result<Vec<Strategem>, StrategemError> {
        let strategems = (&yaml["strategems"])
            .as_vec()
            .unwrap_or(&vec![])
            .into_iter()
            .map(|s| {
                let name = ((&s["name"]).clone())
                    .into_string()
                    .ok_or(StrategemError::MissingName)?;

                let code = ((&s["code"]).clone())
                    .into_string()
                    .ok_or(StrategemError::MissingCode)
                    .and_then(|s| Ok(Code::from_string(s)?))?;

                Ok(Strategem { name, code })
            })
            .collect::<Vec<Result<Strategem, StrategemError>>>();

        match ((&strategems).into_iter())
            .cloned()
            .filter_map(|x| x.err())
            .collect::<Vec<StrategemError>>()
            .first()
        {
            None => Ok(strategems.into_iter().filter_map(|x| x.ok()).collect()),
            Some(e) => Err(e.clone()),
        }
    }
}
