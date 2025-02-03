#[cfg(test)]
mod tests {
    use tl_interface::Eval;
    use tl_interface::Wrapper;
    use tl_interface::IInterface;

    #[test]
    fn test_wrapper_implements_iinterface() {
        struct DummyVerb;
        struct DummyRes;

        struct DummyWrapper;

        impl Eval<DummyVerb, DummyRes> for DummyWrapper {
            fn eval() -> DummyRes {
                DummyRes
            }
        }

        type TestWrapper = Wrapper<DummyVerb, DummyWrapper>;

        // Ensure that TestWrapper implements IInterface
        fn assert_iinterface<I: IInterface<DummyVerb>>() {}
        assert_iinterface::<TestWrapper>();
    }
}
