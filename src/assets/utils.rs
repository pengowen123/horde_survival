macro_rules! impl_asset_methods {
    ($typ:ident, $asset_type:ident, $field:ident,
     $name1:ident, $name2:ident, $name3:ident, $name4:ident,
     $log_str:expr) => {

        impl<P: AsRef<Path>> $typ<P> {
            pub fn $name1(&self, name: &str) -> Option<&$asset_type> {
                self.$field.get(name).map(|a| a.get()).unwrap_or(None)
            }

            pub fn $name2(&mut self, name: &str, factory: &mut Factory) -> io::Result<()> {
                match self.$field.get_mut(name) {
                    Some(asset) => asset.load(factory),
                    None => Err(Error::new(ErrorKind::Other, format!("{} asset not found: {}", $log_str, name))),
                }
            }

            pub fn $name3(&mut self, name: &str, factory: &mut Factory) -> io::Result<&$asset_type> {
                match self.$field.get_mut(name) {
                    Some(asset) => asset.get_or_load(factory),
                    None => Err(Error::new(ErrorKind::Other, format!("{} asset not found: {}", $log_str, name))),
                }
            }

            pub fn $name4<S: Clone + Into<P>>(&mut self, names: &[(&'static str, S)]) {
                for &(ref name, ref path) in names {
                    self.$field.insert(*name, Asset::new(path.clone().into()));
                }
            }
        }
    };
}
