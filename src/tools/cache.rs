use egs_api::api::types::{AssetInfo, EpicAsset, KeyImage};
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::path::PathBuf;

pub(crate) trait Cache {
    fn save(&self, data: Option<Vec<u8>>);
    fn prepare(item: String) -> PathBuf {
        let mut path = match dirs::cache_dir() {
            None => PathBuf::from("cache"),
            Some(mut dir) => {
                dir.push("epic_asset_manager");
                dir
            }
        };
        path.push(&item);

        let cache = Path::new(&path);
        fs::create_dir_all(cache.clone()).unwrap();
        cache.to_path_buf()
    }

    fn load_from_cache(_item: String) -> Option<Self>
    where
        Self: Sized,
    {
        unimplemented!()
    }

    fn load(&self) -> Option<Vec<u8>> {
        unimplemented!()
    }
}

impl Cache for EpicAsset {
    fn save(&self, _: Option<Vec<u8>>) {
        let mut cache = <Self as Cache>::prepare(self.catalog_item_id.clone());
        cache.push("epic_asset.json");
        if let Ok(mut asset_file) = File::create(cache.as_path()) {
            asset_file
                .write(serde_json::to_string(&self).unwrap().as_bytes().as_ref())
                .unwrap();
        }
    }
}

impl Cache for AssetInfo {
    fn save(&self, _: Option<Vec<u8>>) {
        let mut cache = <Self as Cache>::prepare(self.id.clone());
        cache.push("asset_info.json");
        if let Ok(mut asset_file) = File::create(cache.as_path()) {
            asset_file
                .write(serde_json::to_string(&self).unwrap().as_bytes().as_ref())
                .unwrap();
        }
    }

    fn load_from_cache(id: String) -> Option<Self> {
        let mut cache = <Self as Cache>::prepare(id.clone());
        cache.push("asset_info.json");
        match File::open(cache.as_path()) {
            Ok(mut f) => {
                let metadata = fs::metadata(&cache.as_path()).expect("unable to read metadata");
                let mut buffer = vec![0; metadata.len() as usize];
                f.read(&mut buffer).expect("buffer overflow");
                return serde_json::from_slice(buffer.as_ref()).unwrap_or(None);
            }
            Err(_) => {}
        };
        None
    }
}

impl Cache for KeyImage {
    fn save(&self, data: Option<Vec<u8>>) {
        let mut cache = <Self as Cache>::prepare("images".into());
        match data {
            None => {}
            Some(d) => {
                let name = self.url.split(".").collect::<Vec<&str>>();
                cache.push(format!("{}.{}", self.md5, name.last().unwrap_or(&".png")));
                match File::create(cache.as_path()) {
                    Ok(mut asset_file) => {
                        asset_file.write(&d).unwrap();
                    }
                    Err(e) => {
                        println!("{:?}", e);
                    }
                }
            }
        }
    }

    fn load(&self) -> Option<Vec<u8>> {
        let mut cache = <Self as Cache>::prepare("images".into());
        let name = self.url.split(".").collect::<Vec<&str>>();
        cache.push(format!("{}.{}", self.md5, name.last().unwrap_or(&".png")));
        match File::open(cache.as_path()) {
            Ok(mut f) => {
                let metadata = fs::metadata(&cache.as_path()).expect("unable to read metadata");
                let mut buffer = vec![0; metadata.len() as usize];
                f.read(&mut buffer).expect("buffer overflow");
                return Some(buffer);
            }
            Err(_) => {}
        };
        None
    }
}