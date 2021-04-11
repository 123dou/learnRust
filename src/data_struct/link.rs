use std::mem::replace;

pub enum Link<T> {
    // 空节点
    None,
    // 尾节点,没有下一个节点
    Tail { item: T },
    // 常规节点,包含下一个节点
    Link { item: T, next: Box<Link<T>> },
}
#[derive(Default)]
pub struct Cursor<T> {
    curr: Link<T>,
}

impl<T> Default for Link<T> {
    fn default() -> Self {
        Link::None
    }
}

impl<T> Link<T>
where
    T: Copy,
{
    pub fn new() -> Link<T> {
        Self::default()
    }
    // 改变当前值,必须用&mut self
    pub fn push(&mut self, val: T) {
        match self {
            Self::None => self.be_tail(val),
            Self::Tail { item: _ } => self.connect_link(val),
            Self::Link { item: _, next } => next.push(val),
        }
    }
    pub fn pop(&mut self) -> Option<T> {
        match self {
            Self::None => None,
            Self::Tail { item } => {
                let it = *item;
                self.be_none();
                Some(it)
            }
            Self::Link { item, next } => {
                let mut temp = Box::new(Link::None);
                let it = *item;
                std::mem::swap(next, &mut temp);
                self.be_next(*temp);
                Some(it)
            }
        }
    }
    fn be_tail(&mut self, val: T) {
        *self = match self {
            Self::None => Self::Tail { item: val },
            Self::Link { item: _, next: _ } => Self::Tail { item: val },
            _ => panic!("the link can't convert to Tail"),
        }
    }
    fn connect_link(&mut self, val: T) {
        *self = match self {
            Self::Tail { item } => Self::Link {
                item: *item,
                next: Box::new(Self::Tail { item: val }),
            },
            _ => panic!("the link can't convert to link"),
        }
    }
    fn be_none(&mut self) {
        replace(self, Self::None);
    }
    fn be_next(&mut self, next: Link<T>) {
        *self = next;
    }
}

impl<T> IntoIterator for Link<T>
where
    T: Copy,
{
    type Item = T;
    type IntoIter = Cursor<T>;

    fn into_iter(self) -> Self::IntoIter {
        Cursor { curr: self }
    }
}

impl<T> Iterator for Cursor<T>
where
    T: Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let nxt = match self.curr {
            Link::None => None,
            Link::Tail { item } => {
                self.curr = Link::None;
                Some(item)
            }
            Link::Link { item, ref mut next } => {
                let mut temp = Box::new(Link::None);
                std::mem::swap(next, &mut temp);
                self.curr = *temp;
                Some(item)
            }
        };
        nxt
    }
}
