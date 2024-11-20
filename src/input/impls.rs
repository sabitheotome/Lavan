use super::traits::*;

impl<T> IntoStream for T
where
    T: IntoIterator,
    T::IntoIter: Stream,
{
    type IntoStream = T::IntoIter;

    fn into_scanner(self) -> Self::IntoStream {
        self.into_iter()
    }
}

impl<'a> Stream for std::str::Chars<'a> {
    type SaveState = Self;

    fn savestate(&mut self) -> Self::SaveState {
        self.clone()
    }

    fn backtrack(&mut self, state: Self::SaveState) {
        *self = state;
    }
}

impl<'a> StreamSlice for std::str::Chars<'a> {
    type Slice = &'a str;
    type SliceOffset = &'a str;

    fn slice_since(&self, builder: Self::SliceOffset) -> Self::Slice {
        &builder[..builder.len() - self.as_str().len()]
    }

    fn slice_offset(&self) -> Self::SliceOffset {
        self.as_str()
    }
}

impl<'a, T> Stream for std::slice::Iter<'a, T> {
    type SaveState = Self;

    fn savestate(&mut self) -> Self::SaveState {
        self.clone()
    }

    fn backtrack(&mut self, state: Self::SaveState) {
        *self = state;
    }
}

impl<'a, T> StreamSlice for std::slice::Iter<'a, T> {
    type Slice = &'a [T];
    type SliceOffset = &'a [T];

    fn slice_since(&self, builder: Self::SliceOffset) -> Self::Slice {
        &builder[..builder.len() - self.as_slice().len()]
    }

    fn slice_offset(&self) -> Self::SliceOffset {
        self.as_slice()
    }
}
