use serde::Serialize;

pub trait Comment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: CommentSerializer;
}

pub trait CommentSerializer {
    type Ok;
    type Error;
    fn add_comment(&mut self, path: &str, comment: &str) -> Result<(), Self::Error>;
    fn add_field<S: Comment>(&mut self, path: &str, value: &S) -> Result<(), Self::Error>;
    fn end(self) -> Result<Self::Ok, Self::Error>;
}

trait AutoImplMarker: Serialize {}

macro_rules! auto_impl {
    ($($id: ident),*) => {
        $(
        impl AutoImplMarker for $id {}
        )*
    };
}

auto_impl!(String, i8, u8, i16, u16, i32, u32, i64, u64, i128, u128);

impl<T: AutoImplMarker> Comment for T {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: CommentSerializer,
    {
        serializer.end()
    }
}
