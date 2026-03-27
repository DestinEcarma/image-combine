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
        .map(|p| image::open(p).with_context(|| format!("opening {p:?}")))
        .collect::<_>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    use tempfile::tempdir;

    #[test]
    fn expand_inputs_keeps_plain_paths() {
        let inputs = vec!["a.png".to_string(), "b.jpg".to_string()];
        let out = expand_inputs(&inputs).unwrap();

        assert_eq!(out, vec![PathBuf::from("a.png"), PathBuf::from("b.jpg")]);
    }

    #[test]
    fn expand_inputs_reads_list_file() {
        let dir = tempdir().unwrap();
        let list_path = dir.path().join("images.txt");

        fs::write(&list_path, "one.png\ntwo.png\n\n three.png \n").unwrap();

        let input = vec![format!("@{}", list_path.display())];
        let out = expand_inputs(&input).unwrap();

        assert_eq!(
            out,
            vec![
                PathBuf::from("one.png"),
                PathBuf::from("two.png"),
                PathBuf::from("three.png"),
            ]
        );
    }

    #[test]
    fn expand_inputs_supports_nested_list_files() {
        let dir = tempdir().unwrap();
        let inner = dir.path().join("inner.txt");
        let outer = dir.path().join("outer.txt");

        fs::write(&inner, "a.png\nb.png\n").unwrap();
        fs::write(&outer, format!("@{}\nc.png\n", inner.display())).unwrap();

        let input = vec![format!("@{}", outer.display())];
        let out = expand_inputs(&input).unwrap();

        assert_eq!(
            out,
            vec![
                PathBuf::from("a.png"),
                PathBuf::from("b.png"),
                PathBuf::from("c.png"),
            ]
        );
    }

    #[test]
    fn expand_inputs_expands_glob() {
        let dir = tempdir().unwrap();
        let a = dir.path().join("a.png");
        let b = dir.path().join("b.png");
        let c = dir.path().join("c.jpg");

        fs::write(&a, b"x").unwrap();
        fs::write(&b, b"x").unwrap();
        fs::write(&c, b"x").unwrap();

        let pattern = format!("{}/{}.png", dir.path().display(), "*");
        let out = expand_inputs(&[pattern]).unwrap();

        assert_eq!(out.len(), 2);
        assert!(out.contains(&a));
        assert!(out.contains(&b));
    }

    #[test]
    fn expand_inputs_errors_for_missing_list_file() {
        let out = expand_inputs(&["@definitely-missing-file.txt".to_string()]);
        assert!(out.is_err());
    }
}
