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
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to deserialize toml: {0}")]
    TomlDeserialize(#[from] toml::de::Error),
    #[error("Failed to deserialize xml: {0}")]
    XmlDeserialize(#[from] quick_xml::DeError),
}

pub fn load_widget(directory: impl AsRef<Path>) -> Result<Widget, WidgetLoadError> {
    let directory = directory.as_ref();
    let config = load_widget_config(directory.join("taskbar.config.toml"))?;
    let component = load_widget_components(directory.join("taskbar.component.xml"))?;
    Ok(Widget::new(config, component))
}

pub fn load_widget_config(path: PathBuf) -> Result<WidgetConfig, WidgetLoadError> {
    Ok(toml::from_slice(
        &File::open(path)?.bytes().collect::<Result<Vec<_>, _>>()?,
    )?)
}

pub fn load_widget_components(path: PathBuf) -> Result<Component, WidgetLoadError> {
    Ok(quick_xml::de::from_reader(BufReader::new(File::open(
        path,
    )?))?)
}
