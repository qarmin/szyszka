// Needs to be updated when changed order of notebook tabs

pub(crate) const EXAMPLE_NAME: &str = "Czkawka.Txt";

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum NotebookEnum {
    Custom = 0,
    CaseSize,
    Purge,
    AddNumber,
    AddText,
    Replace,
    Trim,
}
pub fn to_notebook_enum(notebook_number: u32) -> NotebookEnum {
    match notebook_number {
        0 => NotebookEnum::Custom,
        1 => NotebookEnum::CaseSize,
        2 => NotebookEnum::Purge,
        3 => NotebookEnum::AddNumber,
        4 => NotebookEnum::AddText,
        5 => NotebookEnum::Replace,
        6 => NotebookEnum::Trim,
        _ => panic!("Invalid Notebook Tab"),
    }
}
