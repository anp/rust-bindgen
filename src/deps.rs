/// Generating build depfiles from parsed bindings.
use crate::{
    callbacks::{
        DeriveTrait, EnumVariantCustomBehavior, EnumVariantValue,
        ImplementsTrait, IntKind, MacroParsingBehavior, ParseCallbacks,
    },
    BindgenOptions,
};
use std::{
    collections::BTreeSet,
    path::PathBuf,
    sync::{Arc, Mutex},
};

#[derive(Debug)]
pub(crate) struct DepfileSpec {
    pub output_module: String,
    pub depfile_path: PathBuf,
}

impl DepfileSpec {
    pub fn write(&self, deps: &BTreeSet<String>) -> std::io::Result<()> {
        let mut buf = format!("{}:", self.output_module);

        for file in deps {
            buf = format!("{} {}", buf, file);
        }

        std::fs::write(&self.depfile_path, &buf)
    }
}
