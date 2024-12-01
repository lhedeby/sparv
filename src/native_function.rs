enum NativeFunction {
    Print,
    ReadFile,
    ReadInput,
    Len,
    Split,
    SplitLines,
    Append,
    Parse,
    Typeof,
    Random,
}

impl NativeFunction {
    fn docs(&self) -> String {
        match self {
            NativeFunction::Print => todo!(),
            NativeFunction::ReadFile => todo!(),
            NativeFunction::ReadInput => todo!(),
            NativeFunction::Len => todo!(),
            NativeFunction::Split => todo!(),
            NativeFunction::SplitLines => todo!(),
            NativeFunction::Append => todo!(),
            NativeFunction::Parse => todo!(),
            NativeFunction::Typeof => todo!(),
            NativeFunction::Random => todo!(),
        }
    }
    
}
