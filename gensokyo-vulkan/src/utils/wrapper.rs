
pub struct VKWrapperInfo<C, I>
    where
        C: Sized,
        I: Sized {

    pub contents: Vec<C>,
    pub infos   : Vec<I>,
}

pub struct VKWrapperPair<C, I>
    where
        C: Sized,
        I: Sized {

    pub content: C,
    pub info   : I,
}

impl<C, I> VKWrapperInfo<C, I> {

    pub fn new() -> VKWrapperInfo<C, I> {

        VKWrapperInfo {
            contents: Vec::new(),
            infos   : Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn with_capability(count: usize) -> VKWrapperInfo<C, I> {

        VKWrapperInfo {
            contents: Vec::with_capacity(count),
            infos   : Vec::with_capacity(count),
        }
    }

    pub fn push(&mut self, pair: VKWrapperPair<C, I>) {
        self.contents.push(pair.content);
        self.infos.push(pair.info);
    }

    pub fn borrow_info(&self) -> &Vec<I> {
        &self.infos
    }
}

