use crate::backend::{
    AsyncFilePickerDialogImpl, AsyncFileSaveDialogImpl,
    AsyncFolderPickerDialogImpl, AsyncMessageDialogImpl, DialogFutureType,
    FilePickerDialogImpl, FileSaveDialogImpl,
    FolderPickerDialogImpl, MessageDialogImpl,
};
use crate::message_dialog::{MessageButtons, MessageDialog, MessageDialogResult, MessageLevel};
use crate::{FileDialog, FileHandle};
use std::path::PathBuf;

pub struct AndroidDialog;

impl FilePickerDialogImpl for FileDialog {
    fn pick_file(self) -> Option<PathBuf> {
        let p = PathBuf::from("/sdcard/Download/ArtCraft");
        if let Ok(entries) = std::fs::read_dir(&p) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    return Some(path);
                }
            }
        }
        Some(p.join("document.pdf"))
    }

    fn pick_files(self) -> Option<Vec<PathBuf>> {
        self.pick_file().map(|p| vec![p])
    }
}

impl FolderPickerDialogImpl for FileDialog {
    fn pick_folder(self) -> Option<PathBuf> {
        Some(PathBuf::from("/sdcard/Download/ArtCraft"))
    }

    fn pick_folders(self) -> Option<Vec<PathBuf>> {
        Some(vec![PathBuf::from("/sdcard/Download/ArtCraft")])
    }
}

impl FileSaveDialogImpl for FileDialog {
    fn save_file(self) -> Option<PathBuf> {
        Some(PathBuf::from("/sdcard/Download/ArtCraft/output.pdf"))
    }
}

impl AsyncFilePickerDialogImpl for FileDialog {
    fn pick_file_async(self) -> DialogFutureType<Option<FileHandle>> {
        let path = FilePickerDialogImpl::pick_file(self);
        Box::pin(async move { path.map(FileHandle::from) })
    }

    fn pick_files_async(self) -> DialogFutureType<Option<Vec<FileHandle>>> {
        let paths = FilePickerDialogImpl::pick_files(self);
        Box::pin(async move {
            paths.map(|v| v.into_iter().map(FileHandle::from).collect())
        })
    }
}

impl AsyncFolderPickerDialogImpl for FileDialog {
    fn pick_folder_async(self) -> DialogFutureType<Option<FileHandle>> {
        let path = FolderPickerDialogImpl::pick_folder(self);
        Box::pin(async move { path.map(FileHandle::from) })
    }

    fn pick_folders_async(self) -> DialogFutureType<Option<Vec<FileHandle>>> {
        let paths = FolderPickerDialogImpl::pick_folders(self);
        Box::pin(async move {
            paths.map(|v| v.into_iter().map(FileHandle::from).collect())
        })
    }
}

impl AsyncFileSaveDialogImpl for FileDialog {
    fn save_file_async(self) -> DialogFutureType<Option<FileHandle>> {
        let path = FileSaveDialogImpl::save_file(self);
        Box::pin(async move { path.map(FileHandle::from) })
    }
}

impl MessageDialogImpl for MessageDialog {
    fn show(self) -> MessageDialogResult {
        MessageDialogResult::Ok
    }
}

impl AsyncMessageDialogImpl for MessageDialog {
    fn show_async(self) -> DialogFutureType<MessageDialogResult> {
        Box::pin(async move { MessageDialogResult::Ok })
    }
}
