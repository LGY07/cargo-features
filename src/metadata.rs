use serde::Deserialize;
use std::collections::BTreeMap;
use std::str::FromStr;
use colored::Colorize;

#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub packages: Vec<Package>,
}

#[derive(Debug, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,

    /// feature_name -> enabled items
    /// e.g. "gtk" -> ["dep:gtk4", "glib"]
    pub features: BTreeMap<String, Vec<String>>,

    pub dependencies: Vec<Dependency>,
}

#[derive(Debug, Deserialize)]
pub struct Dependency {
    pub name: String,
    pub req: String,

    /// true if optional = true
    pub optional: bool,

    /// features this dependency exposes (rarely needed)
    #[serde(default)]
    pub features: Vec<String>,
}

impl FromStr for Metadata{
    type Err = serde_json::error::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str::<Self>(s)
    }
}

impl Metadata{
    pub fn show_features(&self){
        // 当前项目对应的 Package
        let pkg = self.packages.first().unwrap();
        println!("{} {} v{}","Package".bright_green(),pkg.name,pkg.version);

        // 遍历依赖
        for dep in &self.packages.first().unwrap().dependencies{
            println!("\t{} {} {}","Dependency".bright_green(),dep.name,dep.req);
            println!("\t\tFeatures:");
            // 当前依赖对应的 Package
            let dep_pkg = self.packages
                .iter()
                .find(|x|x.name == dep.name)
                .expect("Incorrect metadata format");
            // 未使用的 features
            let unused_features = dep_pkg.features
                .iter()
                .map(|x| {x.0})
                .filter(|x| {
                    dep.features
                        .iter()
                        .any(|x1| {x1!=*x})
                });
            for feature in &dep.features{
                println!("\t\t{} {}","+".green(),feature);
            }
            for unused_feature in unused_features{
                println!("\t\t{} {}","-".bright_red(),unused_feature)
            }
        }
    }
}