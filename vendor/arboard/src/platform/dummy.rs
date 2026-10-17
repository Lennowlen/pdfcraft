use crate::common::Error;
use std::borrow::Cow;
use std::path::{Path, PathBuf};

pub struct Clipboard {
    text: String,
}

impl Clipboard {
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            text: String::new(),
        })
    }
}

pub struct Get<'a> {
    _clipboard: &'a mut Clipboard,
}

impl<'a> Get<'a> {
    pub fn new(clipboard: &'a mut Clipboard) -> Self {
        Self {
            _clipboard: clipboard,
        }
    }

    pub fn text(self) -> Result<String, Error> {
        Ok(self._clipboard.text.clone())
    }

    #[cfg(feature = "image-data")]
    pub fn image(self) -> Result<crate::ImageData<'static>, Error> {
        Err(Error::ContentNotAvailable)
    }

    pub fn html(self) -> Result<String, Error> {
        Err(Error::ContentNotAvailable)
    }

    pub fn file_list(self) -> Result<Vec<PathBuf>, Error> {
        Err(Error::ContentNotAvailable)
    }
}

pub struct Set<'a> {
    clipboard: &'a mut Clipboard,
}

impl<'a> Set<'a> {
    pub fn new(clipboard: &'a mut Clipboard) -> Self {
        Self { clipboard }
    }

    pub fn text(self, text: Cow<'_, str>) -> Result<(), Error> {
        self.clipboard.text = text.into_owned();
        Ok(())
    }

    #[cfg(feature = "image-data")]
    pub fn image(self, _image: crate::ImageData<'_>) -> Result<(), Error> {
        Ok(())
    }

    pub fn html(self, _html: Cow<'_, str>, _alt_text: Option<Cow<'_, str>>) -> Result<(), Error> {
        Ok(())
    }

    pub fn file_list(self, _files: &[impl AsRef<Path>]) -> Result<(), Error> {
        Ok(())
    }
}

pub struct Clear<'a> {
    clipboard: &'a mut Clipboard,
}

impl<'a> Clear<'a> {
    pub fn new(clipboard: &'a mut Clipboard) -> Self {
        Self { clipboard }
    }

    pub fn clear(self) -> Result<(), Error> {
        self.clipboard.text.clear();
        Ok(())
    }
}
