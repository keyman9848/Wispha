use crate::layout_templates::line;
use crate::layouter::Layout;
use super::{InteractConfig, InteractOption};

use libwispha::core::*;
use structopt::StructOpt;

use std::fmt;
use std::error;

#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct LayoutOptions {
    #[structopt(long, short)]
    layout: Option<String>,

    #[structopt(long, short)]
    path: Option<String>,

    #[structopt(long, short, use_delimiter = true)]
    keys: Option<Vec<String>>,

    #[structopt(long, short)]
    hide_key: bool,

    #[structopt(long, short)]
    depth: Option<usize>,
}

struct LayoutConfig {
    layout: String,
    path: String,
    keys: Vec<String>,
    hide_key: bool,
    depth: usize,
}

impl LayoutConfig {
    fn from_opt(layout_opt: LayoutOptions) -> Result<Self, Error> {
        let layout = if let Some(layout) = layout_opt.layout {
            layout
        } else {
            line::LineLayout::new().info().name.clone()
        };

        let path = if let Some(path) = layout_opt.path {
            if path.starts_with("/") {
                path
            } else {
                return Err(Error::NodePathMustBeAbsolute(path));
            }
        } else {
            "/".to_string()
        };

        let keys = if let Some(keys) = layout_opt.keys {
            keys
        } else {
            vec![]
        };

        let hide_key = layout_opt.hide_key.clone();

        let depth = if let Some(depth) = layout_opt.depth {
            depth
        } else {
            3
        };

        Ok(LayoutConfig {
            layout,
            path,
            keys,
            hide_key,
            depth
        })
    }
}

impl InteractOption for LayoutOptions {
    fn run(self, _interact_conf: &InteractConfig, tree: &Tree) -> Result<(), Box<dyn error::Error>> {
        let config = LayoutConfig::from_opt(self)?;

        let node_path = NodePath::from(&config.path, &tree)?;
        let layout_str = crate::layouter::LayoutManager::layout(&config.layout,
                                                                &crate::layout_templates::layout_resolver,
                                                                &tree,
                                                                &node_path,
                                                                config.depth,
                                                                &config.keys,
                                                                config.hide_key)?;
        println!("{}", layout_str);
        Ok(())
    }
}

#[derive(Debug)]
pub enum Error {
    NodePathMustBeAbsolute(String),
}

impl error::Error for Error { }

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        use Error::*;
        let message = match &self {
            NodePathMustBeAbsolute(path) => format!("Node path must be absolute, but {} is not.", path),
        };
        write!(f, "{}", message)
    }
}