pub trait UseCaseInterface {
    type T;
    type U;
    async fn handle(&self, data: Self::T) -> Result<Self::U, String>;
}