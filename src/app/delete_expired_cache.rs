use std::path::Path;

use crate::api::schema::*;

use super::cache::Cacheable;

pub fn delete(cache_dir: &Path) -> std::io::Result<()> {
    let helper = Helper::new(cache_dir);
    // helper.delete::<Report>();
    // helper.delete::<RankingAllMonthly>();
    // helper.delete::<RankingSectionMonthly>();
    // helper.delete::<RankingSectionDaily>();
    helper.delete::<RequestReport>()?;

    Ok(())
}

struct Helper<'a> {
    path: &'a Path,
}

impl<'a> Helper<'a> {
    fn new(path: &'a Path) -> Self {
        Self { path }
    }

    fn delete<C: Cacheable>(&self) -> std::io::Result<()> {
        let Some(dir) = C::file_dir() else {
            return Ok(());
        };
        let dir = dir.as_ref();

        eprintln!("delete dir: {:?}", dir);

        let dir = self.path.join(dir);

        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().is_some_and(|ext| ext == "json") && {
                let timestamp = entry.metadata()?.modified()?;
                timestamp.elapsed().is_ok_and(|t| t > C::min_interval())
            } {
                std::fs::remove_file(&path)?;
                eprintln!("file removed: {:?}", path);
            }
        }

        Ok(())
    }
}
