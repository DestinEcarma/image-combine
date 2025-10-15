use std::{fs, path::PathBuf};

use anyhow::{Context, Result};
use glob::glob;
use image::DynamicImage;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub fn expand_inputs(inputs: &[String]) -> Result<Vec<PathBuf>> {
    let mut paths = Vec::new();

    for raw in inputs {
        if let Some(list_path) = raw.strip_prefix("@") {
            let txt = fs::read_to_string(list_path)
                .with_context(|| format!("reading list file \"{list_path}\""))?;

            for line in txt.lines().map(str::trim).filter(|l| !l.is_empty()) {
                paths.extend(expand_inputs(&[line.to_string()])?);
            }
        } else if raw.contains('*') || raw.contains('?') || raw.contains('[') {
            for entry in glob(raw)? {
                paths.push(entry?);
            }
        } else {
            paths.push(raw.into());
        }
    }

    Ok(paths)
}

pub fn load_images(paths: &[PathBuf]) -> Result<Vec<DynamicImage>> {
    paths
        .par_iter()
        .map(|p| Ok(image::open(p).with_context(|| format!("opening {p:?}"))?))
        .collect::<_>()
}
