use anyhow::Result;
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};

#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct ProjectSymbolsSettings {
    pub file_icons: bool,
    pub modal_max_width: Option<FileFinderWidth>,
    pub skip_focus_for_active_in_search: bool,
    pub include_ignored: Option<bool>,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, Debug)]
pub struct ProjectSymbolsSettingsContent {
    /// Whether to show file icons in the file finder.
    ///
    /// Default: true
    pub file_icons: Option<bool>,
    
    /// Determines how much space the file finder can take up in relation to the available window width.
    ///
    /// Default: small
    pub modal_max_width: Option<ProjectSymbolsSize>,

    /// Determines how much vertical space the symbol finder can take in relation to the available window height.
    ///
    /// Default: small
    pub modal_max_height: Option<ProjectSymbolsSize>
}

impl Settings for FileFinderSettings {
    const KEY: Option<&'static str> = Some("file_finder");

    type FileContent = ProjectSymbolsSettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut gpui::App) -> Result<Self> {
        sources.json_merge()
    }

    fn import_from_vscode(_vscode: &settings::VsCodeSettings, _current: &mut Self::FileContent) {}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum ProjectSymbolsSize {
    #[default]
    Small,
    Medium,
    Large,
    XLarge,
    Full,
}
