pub(crate) type Result<T> = core::result::Result<T, Error>;

pub(crate) type DescribeError = String;
pub(crate) type PullError = String;
pub(crate) type PushError = String;
pub(crate) type RunError = String;
pub(crate) type StoreError = String;

#[derive(Debug)]
pub(crate) enum Error {
    Describe(DescribeError),
    Pull(PullError),
    Push(PushError),
    Run(RunError),
    Store(StoreError),
}
