use moon_common::path::WorkspaceRelativePathBuf;
use moon_common::{is_docker, Id};
use moon_hash::hash_content;
use rustc_hash::FxHashMap;
use std::collections::BTreeMap;
use std::env;

hash_content!(
    pub struct ProjectGraphHash<'graph> {
        // Data derived from the project graph builder.
        aliases: BTreeMap<&'graph Id, &'graph String>,
        sources: BTreeMap<&'graph Id, &'graph WorkspaceRelativePathBuf>,

        // Project and workspace configs required for cache invalidation.
        configs: BTreeMap<WorkspaceRelativePathBuf, String>,

        // Environment variables required for cache invalidation.
        env: BTreeMap<String, String>,

        // The project graph stores absolute file paths, which breaks moon when
        // running tasks inside and outside of a container at the same time.
        // This flag helps to continuously bust the cache.
        in_docker: bool,

        // Version of the moon CLI. We need to include this so that the graph
        // cache is invalidated between each release, otherwise internal Rust
        // changes (in project or task crates) are not reflected until the cache
        // is invalidated, which puts the program in a weird state.
        version: String,
    }
);

impl<'cfg> ProjectGraphHash<'cfg> {
    pub fn new() -> Self {
        ProjectGraphHash {
            aliases: BTreeMap::default(),
            sources: BTreeMap::default(),
            configs: BTreeMap::default(),
            env: BTreeMap::default(),
            in_docker: is_docker(),
            version: env::var("MOON_VERSION").unwrap_or_default(),
        }
    }

    pub fn add_aliases(&mut self, aliases: &'cfg FxHashMap<Id, String>) {
        self.aliases.extend(aliases.iter());
    }

    pub fn add_configs(&mut self, configs: BTreeMap<WorkspaceRelativePathBuf, String>) {
        self.configs.extend(configs);
    }

    pub fn add_sources(&mut self, sources: &'cfg FxHashMap<Id, WorkspaceRelativePathBuf>) {
        self.sources.extend(sources.iter());
    }

    pub fn gather_env(&mut self) {
        for key in [
            // Task options
            "MOON_OUTPUT_STYLE",
            "MOON_RETRY_COUNT",
        ] {
            self.env
                .insert(key.to_owned(), env::var(key).unwrap_or_default());
        }
    }
}
