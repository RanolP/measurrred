use std::{
    fs::File,
    io::{BufReader, Read},
    path::{Path, PathBuf},
};

use thiserror::Error;

use crate::component::Component;

use super::{Widget, WidgetConfig};

#[derive(Error, Debug)]
pub enum WidgetLoadError {
    #[error("I/O failed from {0}")]
    Io(PathBuf, #[source] std::io::Error),
    #[error("Failed to deserialize {0}: {1}")]
    TomlDeserialize(PathBuf, #[source] toml::de::Error),
    #[error("Failed to deserialize {0}: {1}")]
    XmlDeserialize(PathBuf, #[source] quick_xml::DeError),
}

pub fn load_widget<'a>(directory: impl AsRef<Path>) -> Result<Widget, WidgetLoadError> {
    let directory = directory.as_ref();
    let widget_config = load_widget_config(directory.join("taskbar.config.toml"))?;
    let component = load_widget_components(directory.join("taskbar.component.xml"))?;
    Ok(Widget::new(widget_config, component))
}

pub fn load_widget_config(path: PathBuf) -> Result<WidgetConfig, WidgetLoadError> {
    Ok(toml::from_slice(
        &File::open(path.clone())
            .and_then(|file| file.bytes().collect::<Result<Vec<_>, _>>())
            .map_err(|e| WidgetLoadError::Io(path.clone(), e))?,
    )
    .map_err(|e| WidgetLoadError::TomlDeserialize(path.clone(), e))?)
}

pub fn load_widget_components(path: PathBuf) -> Result<Component, WidgetLoadError> {
    Ok(quick_xml::de::from_reader(BufReader::new(
        File::open(path.clone()).map_err(|e| WidgetLoadError::Io(path.clone(), e))?,
    ))
    .map_err(|e| WidgetLoadError::XmlDeserialize(path.clone(), e))?)
}
