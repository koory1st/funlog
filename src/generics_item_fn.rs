use syn::{Attribute, Block, ItemFn, Signature, Visibility};

pub struct GenericsFn {
    #[allow(dead_code)]
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub sig: Signature,
    pub block: Block,
}

impl From<ItemFn> for GenericsFn {
    fn from(item: ItemFn) -> Self {
        GenericsFn {
            attrs: item.attrs,
            vis: item.vis,
            sig: item.sig,
            block: *item.block,
        }
    }
}
